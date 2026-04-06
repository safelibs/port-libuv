use crate::abi;
use crate::allocator::{self, UV_EINVAL, UV_ENOMEM};
use crate::bindings::*;
use crate::handle::{handle_init, handle_start, queue_init, queue_insert_tail};

unsafe extern "C" {
    static mut environ: *mut *mut libc::c_char;
}

const PROCESS_SUPPORTED_FLAGS: libc::c_uint = uv_process_flags_UV_PROCESS_DETACHED
    | uv_process_flags_UV_PROCESS_SETGID
    | uv_process_flags_UV_PROCESS_SETUID
    | uv_process_flags_UV_PROCESS_WINDOWS_FILE_PATH_EXACT_NAME
    | uv_process_flags_UV_PROCESS_WINDOWS_HIDE
    | uv_process_flags_UV_PROCESS_WINDOWS_HIDE_CONSOLE
    | uv_process_flags_UV_PROCESS_WINDOWS_HIDE_GUI
    | uv_process_flags_UV_PROCESS_WINDOWS_VERBATIM_ARGUMENTS;

#[inline]
unsafe fn close_nocheck(fd: libc::c_int) {
    if fd < 0 {
        return;
    }

    loop {
        if libc::close(fd) == 0 {
            return;
        }
        if *allocator::errno_location() != libc::EINTR {
            return;
        }
    }
}

#[inline]
unsafe fn clear_cloexec(fd: libc::c_int) -> libc::c_int {
    let flags = libc::fcntl(fd, libc::F_GETFD);
    if flags == -1 {
        return allocator::last_error();
    }
    if libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC) == -1 {
        return allocator::last_error();
    }
    0
}

#[inline]
unsafe fn set_nonblock(fd: libc::c_int, on: bool) -> libc::c_int {
    let flags = libc::fcntl(fd, libc::F_GETFL);
    if flags == -1 {
        return allocator::last_error();
    }

    let new_flags = if on {
        flags | libc::O_NONBLOCK
    } else {
        flags & !libc::O_NONBLOCK
    };
    if libc::fcntl(fd, libc::F_SETFL, new_flags) == -1 {
        return allocator::last_error();
    }
    0
}

unsafe fn write_int_and_exit(fd: libc::c_int, value: libc::c_int) -> ! {
    let ptr = std::ptr::addr_of!(value).cast::<libc::c_void>();
    let len = std::mem::size_of::<libc::c_int>();

    loop {
        let written = libc::write(fd, ptr, len);
        if written == len as isize {
            break;
        }
        if written == -1 && *allocator::errno_location() == libc::EINTR {
            continue;
        }
        break;
    }

    libc::_exit(127)
}

unsafe fn write_errno_and_exit(fd: libc::c_int) -> ! {
    write_int_and_exit(fd, allocator::last_error())
}

unsafe extern "C" fn uv__chld(handle: *mut uv_signal_t, signum: libc::c_int) {
    if signum == libc::SIGCHLD {
        abi::uv__wait_children((*handle).loop_);
    }
}

unsafe fn process_init_stdio(
    container: *const uv_stdio_container_t,
    fds: &mut [libc::c_int; 2],
) -> libc::c_int {
    let mask = uv_stdio_flags_UV_IGNORE as libc::c_int
        | uv_stdio_flags_UV_CREATE_PIPE as libc::c_int
        | uv_stdio_flags_UV_INHERIT_FD as libc::c_int
        | uv_stdio_flags_UV_INHERIT_STREAM as libc::c_int;
    let flags = (*container).flags as libc::c_int;

    match flags & mask {
        x if x == uv_stdio_flags_UV_IGNORE as libc::c_int => 0,
        x if x == uv_stdio_flags_UV_CREATE_PIPE as libc::c_int => {
            let stream = (*container).data.stream;
            if stream.is_null()
                || (*stream.cast::<uv_handle_t>()).type_ != uv_handle_type_UV_NAMED_PIPE
            {
                return UV_EINVAL;
            }

            let mut socks = [0 as uv_os_sock_t; 2];
            let rc =
                crate::linux::socket::uv_socketpair(libc::SOCK_STREAM, 0, socks.as_mut_ptr(), 0, 0);
            if rc != 0 {
                return rc;
            }

            fds[0] = socks[0] as libc::c_int;
            fds[1] = socks[1] as libc::c_int;
            0
        }
        x if x == uv_stdio_flags_UV_INHERIT_FD as libc::c_int => {
            let fd = (*container).data.fd;
            if fd == -1 {
                return UV_EINVAL;
            }
            fds[1] = fd;
            0
        }
        x if x == uv_stdio_flags_UV_INHERIT_STREAM as libc::c_int => {
            let stream = (*container).data.stream;
            let mut fd = -1 as uv_os_fd_t;
            if stream.is_null() {
                return UV_EINVAL;
            }

            let rc = crate::linux::socket::uv_fileno(stream.cast(), &mut fd);
            if rc != 0 {
                return rc;
            }
            if fd == -1 {
                return UV_EINVAL;
            }

            fds[1] = fd as libc::c_int;
            0
        }
        _ => UV_EINVAL,
    }
}

unsafe fn process_open_stream(
    container: *const uv_stdio_container_t,
    pipefds: &mut [libc::c_int; 2],
) -> libc::c_int {
    if ((*container).flags & uv_stdio_flags_UV_CREATE_PIPE) == 0 || pipefds[0] < 0 {
        return 0;
    }

    let stream = (*container).data.stream;
    if stream.is_null() {
        return UV_EINVAL;
    }

    close_nocheck(pipefds[1]);
    pipefds[1] = -1;

    let rc = set_nonblock(pipefds[0], true);
    if rc != 0 {
        return rc;
    }

    crate::pipe::uv_pipe_open(stream.cast(), pipefds[0] as uv_file)
}

unsafe fn process_close_stream(container: *const uv_stdio_container_t) {
    if ((*container).flags & uv_stdio_flags_UV_CREATE_PIPE) == 0 {
        return;
    }

    let stream = (*container).data.stream;
    if !stream.is_null() {
        crate::r#loop::uv_close(stream.cast(), None);
    }
}

unsafe fn process_child_init(
    options: *const uv_process_options_t,
    pipes: &mut [[libc::c_int; 2]],
    error_fd: libc::c_int,
) -> ! {
    let stdio_count = pipes.len();

    for signum in 1..32 {
        if signum == libc::SIGKILL || signum == libc::SIGSTOP {
            continue;
        }

        if libc::signal(signum, libc::SIG_DFL) == libc::SIG_ERR {
            write_errno_and_exit(error_fd);
        }
    }

    if ((*options).flags & uv_process_flags_UV_PROCESS_DETACHED) != 0 {
        libc::setsid();
    }

    for fd in 0..stdio_count {
        let use_fd = pipes[fd][1];
        if use_fd < 0 || use_fd as usize >= fd {
            continue;
        }

        let dup_fd = libc::fcntl(use_fd, libc::F_DUPFD_CLOEXEC, stdio_count as libc::c_int);
        if dup_fd == -1 {
            write_errno_and_exit(error_fd);
        }
        pipes[fd][1] = dup_fd;
    }

    for fd in 0..stdio_count {
        let mut close_fd = -1;
        let mut use_fd = pipes[fd][1];

        if use_fd < 0 {
            if fd >= 3 {
                continue;
            }

            close_nocheck(fd as libc::c_int);
            use_fd = libc::open(
                c"/dev/null".as_ptr(),
                if fd == 0 {
                    libc::O_RDONLY
                } else {
                    libc::O_RDWR
                },
            );
            close_fd = use_fd;
            if use_fd < 0 {
                write_errno_and_exit(error_fd);
            }
        }

        if fd as libc::c_int == use_fd {
            if close_fd == -1 && clear_cloexec(use_fd) != 0 {
                write_errno_and_exit(error_fd);
            }
        } else if libc::dup2(use_fd, fd as libc::c_int) == -1 {
            write_errno_and_exit(error_fd);
        }

        if fd <= 2 && close_fd == -1 && set_nonblock(fd as libc::c_int, false) != 0 {
            write_errno_and_exit(error_fd);
        }

        if close_fd >= stdio_count as libc::c_int {
            close_nocheck(close_fd);
        }
    }

    if !(*options).cwd.is_null() && libc::chdir((*options).cwd) != 0 {
        write_errno_and_exit(error_fd);
    }

    if ((*options).flags
        & (uv_process_flags_UV_PROCESS_SETUID | uv_process_flags_UV_PROCESS_SETGID))
        != 0
    {
        let saved_errno = *allocator::errno_location();
        libc::setgroups(0, std::ptr::null());
        *allocator::errno_location() = saved_errno;
    }

    if ((*options).flags & uv_process_flags_UV_PROCESS_SETGID) != 0
        && libc::setgid((*options).gid) != 0
    {
        write_errno_and_exit(error_fd);
    }

    if ((*options).flags & uv_process_flags_UV_PROCESS_SETUID) != 0
        && libc::setuid((*options).uid) != 0
    {
        write_errno_and_exit(error_fd);
    }

    if !(*options).env.is_null() {
        environ = (*options).env;
    }

    let mut sigset = std::mem::zeroed::<libc::sigset_t>();
    if libc::sigemptyset(&mut sigset) != 0 {
        libc::abort();
    }
    if libc::sigprocmask(libc::SIG_SETMASK, &sigset, std::ptr::null_mut()) != 0 {
        libc::abort();
    }

    libc::execvp((*options).file, (*options).args.cast());
    write_errno_and_exit(error_fd)
}

unsafe fn spawn_and_init_child_fork(
    options: *const uv_process_options_t,
    pipes: &mut [[libc::c_int; 2]],
    error_fd: libc::c_int,
    pid: *mut libc::pid_t,
) -> libc::c_int {
    let mut signewset = std::mem::zeroed::<libc::sigset_t>();
    let mut sigoldset = std::mem::zeroed::<libc::sigset_t>();

    libc::sigfillset(&mut signewset);
    libc::sigdelset(&mut signewset, libc::SIGKILL);
    libc::sigdelset(&mut signewset, libc::SIGSTOP);
    libc::sigdelset(&mut signewset, libc::SIGTRAP);
    libc::sigdelset(&mut signewset, libc::SIGSEGV);
    libc::sigdelset(&mut signewset, libc::SIGBUS);
    libc::sigdelset(&mut signewset, libc::SIGILL);
    libc::sigdelset(&mut signewset, libc::SIGSYS);
    libc::sigdelset(&mut signewset, libc::SIGABRT);

    if libc::pthread_sigmask(libc::SIG_BLOCK, &signewset, &mut sigoldset) != 0 {
        libc::abort();
    }

    let child = libc::fork();
    if child == 0 {
        process_child_init(options, pipes, error_fd);
    }

    if libc::pthread_sigmask(libc::SIG_SETMASK, &sigoldset, std::ptr::null_mut()) != 0 {
        libc::abort();
    }

    if child == -1 {
        return allocator::last_error();
    }

    *pid = child;
    0
}

unsafe fn spawn_and_init_child(
    loop_: *mut uv_loop_t,
    options: *const uv_process_options_t,
    pipes: &mut [[libc::c_int; 2]],
    pid: *mut libc::pid_t,
) -> libc::c_int {
    let mut signal_pipe = [-1; 2];
    if libc::pipe2(signal_pipe.as_mut_ptr(), libc::O_CLOEXEC) != 0 {
        return allocator::last_error();
    }

    crate::thread::uv_rwlock_wrlock(std::ptr::addr_of_mut!((*loop_).cloexec_lock));
    let mut err = spawn_and_init_child_fork(options, pipes, signal_pipe[1], pid);
    crate::thread::uv_rwlock_wrunlock(std::ptr::addr_of_mut!((*loop_).cloexec_lock));

    close_nocheck(signal_pipe[1]);

    if err == 0 {
        let mut exec_errorno = 0;
        let nread = loop {
            let nread = libc::read(
                signal_pipe[0],
                std::ptr::addr_of_mut!(exec_errorno).cast(),
                std::mem::size_of::<libc::c_int>(),
            );
            if nread == -1 && *allocator::errno_location() == libc::EINTR {
                continue;
            }
            break nread;
        };

        if nread == 0 {
            err = 0;
        } else if nread == std::mem::size_of::<libc::c_int>() as isize {
            let mut status = 0;
            loop {
                let waited = libc::waitpid(*pid, &mut status, 0);
                if waited == -1 && *allocator::errno_location() == libc::EINTR {
                    continue;
                }
                break;
            }
            err = exec_errorno;
        } else if nread == -1 && *allocator::errno_location() == libc::EPIPE {
            let mut status = 0;
            loop {
                let waited = libc::waitpid(*pid, &mut status, 0);
                if waited == -1 && *allocator::errno_location() == libc::EINTR {
                    continue;
                }
                break;
            }
            err = uv_errno_t_UV_EPIPE;
        } else {
            libc::abort();
        }
    }

    close_nocheck(signal_pipe[0]);
    err
}

#[no_mangle]
pub unsafe extern "C" fn uv_spawn(
    loop_: *mut uv_loop_t,
    process: *mut uv_process_t,
    options: *const uv_process_options_t,
) -> libc::c_int {
    if loop_.is_null() || process.is_null() || options.is_null() {
        return UV_EINVAL;
    }
    if (*options).file.is_null() || (*options).args.is_null() {
        return UV_EINVAL;
    }
    if (*options).stdio_count < 0 {
        return UV_EINVAL;
    }
    if ((*options).flags & !PROCESS_SUPPORTED_FLAGS) != 0 {
        return UV_EINVAL;
    }

    let mut pipes_storage = [[-1; 2]; 8];
    let stdio_count = ((*options).stdio_count as usize).max(3);
    let mut pipes_heap = Vec::new();
    if stdio_count > pipes_storage.len() {
        if pipes_heap.try_reserve_exact(stdio_count).is_err() {
            return UV_ENOMEM;
        }
        pipes_heap.resize(stdio_count, [-1; 2]);
    }

    let pipes = if pipes_heap.is_empty() {
        &mut pipes_storage[..stdio_count]
    } else {
        pipes_heap.as_mut_slice()
    };

    handle_init(loop_, process.cast(), uv_handle_type_UV_PROCESS);
    queue_init(std::ptr::addr_of_mut!((*process).queue));
    (*process).status = 0;
    (*process).pid = 0;
    (*process).exit_cb = None;

    for i in 0..(*options).stdio_count as usize {
        let rc = process_init_stdio(
            std::ptr::addr_of!((*(*options).stdio.add(i))),
            &mut pipes[i],
        );
        if rc != 0 {
            for pipe in pipes.iter() {
                close_nocheck(pipe[0]);
                close_nocheck(pipe[1]);
            }
            return rc;
        }
    }

    let _ = crate::signal::uv_signal_start(
        std::ptr::addr_of_mut!((*loop_).child_watcher),
        Some(uv__chld),
        libc::SIGCHLD,
    );

    let mut pid = 0;
    let exec_errorno = spawn_and_init_child(loop_, options, pipes, &mut pid);

    if exec_errorno == 0 {
        (*process).pid = pid;
        (*process).exit_cb = (*options).exit_cb;
        queue_insert_tail(
            std::ptr::addr_of_mut!((*loop_).process_handles),
            std::ptr::addr_of_mut!((*process).queue),
        );
        handle_start(process.cast());
    }

    for i in 0..(*options).stdio_count as usize {
        let rc = process_open_stream(
            std::ptr::addr_of!((*(*options).stdio.add(i))),
            &mut pipes[i],
        );
        if rc == 0 {
            continue;
        }

        for j in 0..i {
            process_close_stream(std::ptr::addr_of!((*(*options).stdio.add(j))));
        }

        for (idx, pipe) in pipes.iter().enumerate() {
            let flags = if idx < (*options).stdio_count as usize {
                (*(*options).stdio.add(idx)).flags
            } else {
                uv_stdio_flags_UV_IGNORE
            };
            if (flags & (uv_stdio_flags_UV_INHERIT_FD | uv_stdio_flags_UV_INHERIT_STREAM)) != 0 {
                continue;
            }
            close_nocheck(pipe[0]);
            close_nocheck(pipe[1]);
        }

        return rc;
    }

    exec_errorno
}

#[no_mangle]
pub unsafe extern "C" fn uv_process_kill(
    process: *mut uv_process_t,
    signum: libc::c_int,
) -> libc::c_int {
    if process.is_null() {
        return UV_EINVAL;
    }

    uv_kill((*process).pid, signum)
}

#[no_mangle]
pub unsafe extern "C" fn uv_kill(pid: libc::c_int, signum: libc::c_int) -> libc::c_int {
    if libc::kill(pid, signum) == 0 {
        0
    } else {
        allocator::last_error()
    }
}
