use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use quote::quote;
use syn::{parse_file, parse_quote, FnArg, ForeignItem, ForeignItemFn, Item, Pat};

const VARIADIC_EXPORTS: &[&str] = &["uv_loop_configure"];
const PHASE5_RUST_EXPORTS: &[&str] = &[
    "uv_accept",
    "uv_fileno",
    "uv_fs_event_getpath",
    "uv_fs_event_init",
    "uv_fs_event_start",
    "uv_fs_event_stop",
    "uv_fs_poll_getpath",
    "uv_fs_poll_init",
    "uv_fs_poll_start",
    "uv_fs_poll_stop",
    "uv_guess_handle",
    "uv_is_readable",
    "uv_is_writable",
    "uv_listen",
    "uv_pipe",
    "uv_pipe_bind",
    "uv_pipe_bind2",
    "uv_pipe_chmod",
    "uv_pipe_connect",
    "uv_pipe_connect2",
    "uv_pipe_getpeername",
    "uv_pipe_getsockname",
    "uv_pipe_init",
    "uv_pipe_open",
    "uv_pipe_pending_count",
    "uv_pipe_pending_instances",
    "uv_pipe_pending_type",
    "uv_poll_init",
    "uv_poll_init_socket",
    "uv_poll_start",
    "uv_poll_stop",
    "uv_read_start",
    "uv_read_stop",
    "uv_recv_buffer_size",
    "uv_send_buffer_size",
    "uv_shutdown",
    "uv_signal_init",
    "uv_signal_start",
    "uv_signal_start_oneshot",
    "uv_signal_stop",
    "uv_socketpair",
    "uv_stream_set_blocking",
    "uv_tcp_bind",
    "uv_tcp_close_reset",
    "uv_tcp_connect",
    "uv_tcp_getpeername",
    "uv_tcp_getsockname",
    "uv_tcp_init",
    "uv_tcp_init_ex",
    "uv_tcp_keepalive",
    "uv_tcp_nodelay",
    "uv_tcp_open",
    "uv_tcp_simultaneous_accepts",
    "uv_try_write",
    "uv_try_write2",
    "uv_tty_get_vterm_state",
    "uv_tty_get_winsize",
    "uv_tty_init",
    "uv_tty_reset_mode",
    "uv_tty_set_mode",
    "uv_tty_set_vterm_state",
    "uv_udp_bind",
    "uv_udp_connect",
    "uv_udp_getpeername",
    "uv_udp_getsockname",
    "uv_udp_init",
    "uv_udp_init_ex",
    "uv_udp_open",
    "uv_udp_recv_start",
    "uv_udp_recv_stop",
    "uv_udp_send",
    "uv_udp_set_broadcast",
    "uv_udp_set_membership",
    "uv_udp_set_multicast_interface",
    "uv_udp_set_multicast_loop",
    "uv_udp_set_multicast_ttl",
    "uv_udp_set_source_membership",
    "uv_udp_set_ttl",
    "uv_udp_try_send",
    "uv_udp_using_recvmmsg",
    "uv_write",
    "uv_write2",
];
const PHASE5_LEGACY_ALIAS_EXPORTS: &[&str] = &[
    "uv_accept",
    "uv_available_parallelism",
    "uv_backend_fd",
    "uv_backend_timeout",
    "uv_chdir",
    "uv_clock_gettime",
    "uv_close",
    "uv_cpumask_size",
    "uv_cwd",
    "uv_disable_stdio_inheritance",
    "uv_fs_poll_getpath",
    "uv_fs_poll_init",
    "uv_fs_poll_start",
    "uv_fs_poll_stop",
    "uv_get_osfhandle",
    "uv_getrusage",
    "uv_gettimeofday",
    "uv_hrtime",
    "uv_guess_handle",
    "uv_is_readable",
    "uv_is_writable",
    "uv_is_active",
    "uv_is_closing",
    "uv_listen",
    "uv_loop_alive",
    "uv_open_osfhandle",
    "uv_os_environ",
    "uv_os_getenv",
    "uv_os_get_group",
    "uv_os_gethostname",
    "uv_os_get_passwd",
    "uv_os_get_passwd2",
    "uv_os_getpid",
    "uv_os_getppid",
    "uv_os_getpriority",
    "uv_os_homedir",
    "uv_os_setenv",
    "uv_os_setpriority",
    "uv_os_tmpdir",
    "uv_os_uname",
    "uv_os_unsetenv",
    "uv_pipe",
    "uv_pipe_bind",
    "uv_pipe_bind2",
    "uv_pipe_chmod",
    "uv_pipe_connect",
    "uv_pipe_connect2",
    "uv_pipe_getpeername",
    "uv_pipe_getsockname",
    "uv_pipe_init",
    "uv_pipe_open",
    "uv_pipe_pending_count",
    "uv_pipe_pending_instances",
    "uv_pipe_pending_type",
    "uv_poll_init",
    "uv_poll_init_socket",
    "uv_poll_start",
    "uv_poll_stop",
    "uv_read_stop",
    "uv_shutdown",
    "uv_signal_init",
    "uv_signal_start",
    "uv_signal_start_oneshot",
    "uv_signal_stop",
    "uv_socketpair",
    "uv_stream_set_blocking",
    "uv_run",
    "uv_sleep",
    "uv_tcp_close_reset",
    "uv_tcp_getpeername",
    "uv_tcp_getsockname",
    "uv_tcp_init",
    "uv_tcp_init_ex",
    "uv_tcp_keepalive",
    "uv_tcp_nodelay",
    "uv_tcp_open",
    "uv_tcp_simultaneous_accepts",
    "uv_try_write",
    "uv_try_write2",
    "uv_tty_get_vterm_state",
    "uv_tty_get_winsize",
    "uv_tty_init",
    "uv_tty_reset_mode",
    "uv_tty_set_mode",
    "uv_tty_set_vterm_state",
    "uv_udp_getpeername",
    "uv_udp_getsockname",
    "uv_udp_open",
    "uv_udp_set_broadcast",
    "uv_udp_set_membership",
    "uv_udp_set_multicast_interface",
    "uv_udp_set_multicast_loop",
    "uv_udp_set_multicast_ttl",
    "uv_udp_set_source_membership",
    "uv_udp_set_ttl",
    "uv_udp_using_recvmmsg",
    "uv_translate_sys_error",
    "uv_update_time",
    "uv_write",
    "uv_write2",
];
#[allow(dead_code)]
const PHASE5_PRIVATE_SOURCE_EXPORTS: &[(&str, &str, &[&str])] = &[
    (
        "core",
        "../original/src/unix/core.c",
        &[
            "uv_backend_fd",
            "uv_backend_timeout",
            "uv_chdir",
            "uv_clock_gettime",
            "uv_close",
            "uv_cpumask_size",
            "uv_cwd",
            "uv_disable_stdio_inheritance",
            "uv_fileno",
            "uv_available_parallelism",
            "uv_get_osfhandle",
            "uv_getrusage",
            "uv_gettimeofday",
            "uv_hrtime",
            "uv_is_active",
            "uv_is_closing",
            "uv_loop_alive",
            "uv_open_osfhandle",
            "uv_os_environ",
            "uv_os_getenv",
            "uv_os_get_group",
            "uv_os_gethostname",
            "uv_os_get_passwd",
            "uv_os_get_passwd2",
            "uv_os_getpid",
            "uv_os_getppid",
            "uv_os_getpriority",
            "uv_os_homedir",
            "uv_os_setenv",
            "uv_os_setpriority",
            "uv_os_tmpdir",
            "uv_os_uname",
            "uv_os_unsetenv",
            "uv_run",
            "uv_sleep",
            "uv_thread_getpriority",
            "uv_thread_setpriority",
            "uv_translate_sys_error",
            "uv_update_time",
            "uv__fd_exists",
            "uv__io_active",
            "uv__io_close",
            "uv__io_feed",
            "uv__io_init",
            "uv__io_start",
            "uv__io_stop",
        ],
    ),
    (
        "fs-poll",
        "../original/src/fs-poll.c",
        &[
            "uv_fs_poll_getpath",
            "uv_fs_poll_init",
            "uv_fs_poll_start",
            "uv_fs_poll_stop",
        ],
    ),
    (
        "pipe",
        "../original/src/unix/pipe.c",
        &[
            "uv_pipe",
            "uv_pipe_bind",
            "uv_pipe_bind2",
            "uv_pipe_chmod",
            "uv_pipe_connect",
            "uv_pipe_connect2",
            "uv_pipe_getpeername",
            "uv_pipe_getsockname",
            "uv_pipe_init",
            "uv_pipe_open",
            "uv_pipe_pending_count",
            "uv_pipe_pending_instances",
            "uv_pipe_pending_type",
        ],
    ),
    (
        "poll",
        "../original/src/unix/poll.c",
        &[
            "uv_poll_init",
            "uv_poll_init_socket",
            "uv_poll_start",
            "uv_poll_stop",
        ],
    ),
    (
        "signal",
        "../original/src/unix/signal.c",
        &[
            "uv_signal_init",
            "uv_signal_start",
            "uv_signal_start_oneshot",
            "uv_signal_stop",
        ],
    ),
    (
        "stream",
        "../original/src/unix/stream.c",
        &[
            "uv_accept",
            "uv_is_readable",
            "uv_is_writable",
            "uv_listen",
            "uv_read_stop",
            "uv_shutdown",
            "uv_stream_set_blocking",
            "uv_try_write",
            "uv_try_write2",
            "uv_write",
            "uv_write2",
        ],
    ),
    (
        "tcp",
        "../original/src/unix/tcp.c",
        &[
            "uv_socketpair",
            "uv_tcp_close_reset",
            "uv_tcp_getpeername",
            "uv_tcp_getsockname",
            "uv_tcp_init",
            "uv_tcp_init_ex",
            "uv_tcp_keepalive",
            "uv_tcp_nodelay",
            "uv_tcp_open",
            "uv_tcp_simultaneous_accepts",
        ],
    ),
    (
        "tty",
        "../original/src/unix/tty.c",
        &[
            "uv_guess_handle",
            "uv_tty_get_vterm_state",
            "uv_tty_get_winsize",
            "uv_tty_init",
            "uv_tty_reset_mode",
            "uv_tty_set_mode",
            "uv_tty_set_vterm_state",
        ],
    ),
    (
        "udp",
        "../original/src/unix/udp.c",
        &[
            "uv_udp_getpeername",
            "uv_udp_getsockname",
            "uv_udp_open",
            "uv_udp_set_broadcast",
            "uv_udp_set_membership",
            "uv_udp_set_multicast_interface",
            "uv_udp_set_multicast_loop",
            "uv_udp_set_multicast_ttl",
            "uv_udp_set_source_membership",
            "uv_udp_set_ttl",
            "uv_udp_using_recvmmsg",
        ],
    ),
];
const RUST_EXPORTS: &[&str] = &[
    "uv_async_init",
    "uv_async_send",
    "uv_available_parallelism",
    "uv_backend_fd",
    "uv_backend_timeout",
    "uv_barrier_destroy",
    "uv_barrier_init",
    "uv_barrier_wait",
    "uv_buf_init",
    "uv_chdir",
    "uv_check_init",
    "uv_check_start",
    "uv_check_stop",
    "uv_clock_gettime",
    "uv_close",
    "uv_cond_broadcast",
    "uv_cond_destroy",
    "uv_cond_init",
    "uv_cond_signal",
    "uv_cond_timedwait",
    "uv_cond_wait",
    "uv_cwd",
    "uv_default_loop",
    "uv_disable_stdio_inheritance",
    "uv_dlclose",
    "uv_dlerror",
    "uv_dlopen",
    "uv_dlsym",
    "uv_err_name",
    "uv_err_name_r",
    "uv_exepath",
    "uv_fs_get_path",
    "uv_fs_get_ptr",
    "uv_fs_req_cleanup",
    "uv_fs_chmod",
    "uv_fs_chown",
    "uv_fs_close",
    "uv_fs_fchmod",
    "uv_fs_fchown",
    "uv_fs_fdatasync",
    "uv_fs_fstat",
    "uv_fs_fsync",
    "uv_fs_ftruncate",
    "uv_fs_futime",
    "uv_fs_get_result",
    "uv_fs_link",
    "uv_fs_lstat",
    "uv_fs_mkdir",
    "uv_fs_open",
    "uv_fs_read",
    "uv_fs_readlink",
    "uv_fs_realpath",
    "uv_fs_rename",
    "uv_fs_scandir",
    "uv_fs_sendfile",
    "uv_fs_stat",
    "uv_fs_symlink",
    "uv_fs_unlink",
    "uv_fs_utime",
    "uv_fs_write",
    "uv_fs_get_statbuf",
    "uv_fs_get_type",
    "uv_getaddrinfo",
    "uv_getnameinfo",
    "uv_get_available_memory",
    "uv_get_constrained_memory",
    "uv_get_free_memory",
    "uv_get_osfhandle",
    "uv_get_process_title",
    "uv_get_total_memory",
    "uv_getrusage",
    "uv_gettimeofday",
    "uv_handle_get_data",
    "uv_handle_get_loop",
    "uv_handle_get_type",
    "uv_handle_set_data",
    "uv_handle_size",
    "uv_handle_type_name",
    "uv_has_ref",
    "uv_hrtime",
    "uv_idle_init",
    "uv_idle_start",
    "uv_idle_stop",
    "uv_inet_ntop",
    "uv_inet_pton",
    "uv_ip4_addr",
    "uv_ip4_name",
    "uv_ip6_addr",
    "uv_ip6_name",
    "uv_ip_name",
    "uv_is_active",
    "uv_is_closing",
    "uv_key_create",
    "uv_key_delete",
    "uv_key_get",
    "uv_key_set",
    "uv_loadavg",
    "uv_loop_alive",
    "uv_loop_close",
    "uv_loop_delete",
    "uv_loop_fork",
    "uv_loop_get_data",
    "uv_loop_init",
    "uv_loop_new",
    "uv_loop_set_data",
    "uv_loop_size",
    "uv_metrics_idle_time",
    "uv_metrics_info",
    "uv_mutex_destroy",
    "uv_mutex_init",
    "uv_mutex_init_recursive",
    "uv_mutex_lock",
    "uv_mutex_trylock",
    "uv_mutex_unlock",
    "uv_now",
    "uv_once",
    "uv_open_osfhandle",
    "uv_os_environ",
    "uv_os_free_environ",
    "uv_os_free_group",
    "uv_os_free_passwd",
    "uv_os_get_group",
    "uv_os_get_passwd",
    "uv_os_get_passwd2",
    "uv_os_getenv",
    "uv_os_gethostname",
    "uv_os_getpid",
    "uv_os_getppid",
    "uv_os_getpriority",
    "uv_os_homedir",
    "uv_os_setenv",
    "uv_os_setpriority",
    "uv_os_tmpdir",
    "uv_os_uname",
    "uv_os_unsetenv",
    "uv_prepare_init",
    "uv_prepare_start",
    "uv_prepare_stop",
    "uv_process_get_pid",
    "uv_process_kill",
    "uv_queue_work",
    "uv_ref",
    "uv_replace_allocator",
    "uv_cancel",
    "uv_random",
    "uv_req_get_data",
    "uv_req_get_type",
    "uv_req_set_data",
    "uv_req_size",
    "uv_req_type_name",
    "uv_resident_set_memory",
    "uv_run",
    "uv_rwlock_destroy",
    "uv_rwlock_init",
    "uv_rwlock_rdlock",
    "uv_rwlock_rdunlock",
    "uv_rwlock_tryrdlock",
    "uv_rwlock_trywrlock",
    "uv_rwlock_wrlock",
    "uv_rwlock_wrunlock",
    "uv_sem_destroy",
    "uv_sem_init",
    "uv_sem_post",
    "uv_sem_trywait",
    "uv_sem_wait",
    "uv_set_process_title",
    "uv_setup_args",
    "uv_sleep",
    "uv_spawn",
    "uv_stop",
    "uv_stream_get_write_queue_size",
    "uv_strerror",
    "uv_strerror_r",
    "uv_cpumask_size",
    "uv_timer_again",
    "uv_timer_get_due_in",
    "uv_timer_get_repeat",
    "uv_timer_init",
    "uv_timer_set_repeat",
    "uv_timer_start",
    "uv_timer_stop",
    "uv_thread_create",
    "uv_thread_create_ex",
    "uv_thread_equal",
    "uv_thread_getaffinity",
    "uv_thread_getcpu",
    "uv_thread_getpriority",
    "uv_thread_join",
    "uv_thread_self",
    "uv_thread_setaffinity",
    "uv_thread_setpriority",
    "uv_translate_sys_error",
    "uv_unref",
    "uv_update_time",
    "uv_udp_get_send_queue_count",
    "uv_udp_get_send_queue_size",
    "uv_uptime",
    "uv_version",
    "uv_version_string",
    "uv_walk",
    "uv_kill",
];
const LEGACY_ALIAS_EXPORTS: &[&str] = &[
    "uv_barrier_destroy",
    "uv_barrier_init",
    "uv_barrier_wait",
    "uv_cond_broadcast",
    "uv_cond_destroy",
    "uv_cond_init",
    "uv_cond_signal",
    "uv_cond_timedwait",
    "uv_cond_wait",
    "uv_dlclose",
    "uv_dlerror",
    "uv_dlopen",
    "uv_dlsym",
    "uv_exepath",
    "uv_fs_get_path",
    "uv_fs_get_ptr",
    "uv_fs_get_result",
    "uv_fs_get_statbuf",
    "uv_fs_get_type",
    "uv_get_process_title",
    "uv_handle_get_data",
    "uv_handle_get_loop",
    "uv_handle_get_type",
    "uv_handle_set_data",
    "uv_handle_type_name",
    "uv_inet_ntop",
    "uv_inet_pton",
    "uv_key_create",
    "uv_key_delete",
    "uv_key_get",
    "uv_key_set",
    "uv_loop_get_data",
    "uv_loop_set_data",
    "uv_mutex_destroy",
    "uv_mutex_init",
    "uv_mutex_init_recursive",
    "uv_mutex_lock",
    "uv_mutex_trylock",
    "uv_mutex_unlock",
    "uv_once",
    "uv_process_get_pid",
    "uv_req_get_data",
    "uv_req_get_type",
    "uv_req_set_data",
    "uv_req_type_name",
    "uv_rwlock_destroy",
    "uv_rwlock_init",
    "uv_rwlock_rdlock",
    "uv_rwlock_rdunlock",
    "uv_rwlock_tryrdlock",
    "uv_rwlock_trywrlock",
    "uv_rwlock_wrlock",
    "uv_rwlock_wrunlock",
    "uv_sem_destroy",
    "uv_sem_init",
    "uv_sem_post",
    "uv_sem_trywait",
    "uv_sem_wait",
    "uv_set_process_title",
    "uv_setup_args",
    "uv_stream_get_write_queue_size",
    "uv_thread_create",
    "uv_thread_create_ex",
    "uv_thread_equal",
    "uv_thread_getaffinity",
    "uv_thread_getcpu",
    "uv_thread_join",
    "uv_thread_self",
    "uv_thread_setaffinity",
    "uv_udp_get_send_queue_count",
    "uv_udp_get_send_queue_size",
    "uv_version",
    "uv_version_string",
];
const PRIVATE_FORWARD_EXPORTS: &[&str] = &[
    "uv_cpu_info",
    "uv_free_cpu_info",
    "uv_free_interface_addresses",
    "uv_freeaddrinfo",
    "uv_fs_access",
    "uv_fs_closedir",
    "uv_fs_copyfile",
    "uv_fs_get_system_error",
    "uv_fs_lchown",
    "uv_fs_lutime",
    "uv_fs_mkdtemp",
    "uv_fs_mkstemp",
    "uv_fs_opendir",
    "uv_fs_readdir",
    "uv_fs_rmdir",
    "uv_fs_scandir_next",
    "uv_fs_statfs",
    "uv_if_indextoiid",
    "uv_if_indextoname",
    "uv_interface_addresses",
    "uv_library_shutdown",
    "uv_print_active_handles",
    "uv_print_all_handles",
];

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("missing OUT_DIR"));
    let include_dir = manifest_dir.join("include");
    let header = include_dir.join("uv.h");
    let version_script = manifest_dir.join("abi/libuv.map");
    let legacy_manifest = manifest_dir.join("legacy/linux-manifest.toml");
    let exported_symbols_path = manifest_dir.join("abi/linux-exported-symbols.txt");
    let rename_header = manifest_dir.join("legacy/generated/legacy_rename.h");
    let bindings_path = out_dir.join("bindings.rs");
    let legacy_bindings_path = out_dir.join("legacy_bindings.rs");
    let private_bindings_path = out_dir.join("private_bindings.rs");
    let ffi_exports_path = out_dir.join("ffi_exports_generated.rs");
    let ffi_legacy_aliases_path = out_dir.join("ffi_legacy_aliases_generated.rs");
    let build_manifest_path = out_dir.join("libuv-build-manifest.json");
    let dynamic_list_path = out_dir.join("libuv-dynamic-list.txt");
    let legacy_sources = read_legacy_sources(&legacy_manifest);
    let exported_symbols = read_sorted_lines(&exported_symbols_path);

    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("build.rs").display()
    );
    println!("cargo:rerun-if-changed={}", header.display());
    println!("cargo:rerun-if-changed={}", include_dir.display());
    println!("cargo:rerun-if-changed={}", version_script.display());
    println!("cargo:rerun-if-changed={}", legacy_manifest.display());
    println!("cargo:rerun-if-changed={}", exported_symbols_path.display());
    println!("cargo:rerun-if-changed={}", rename_header.display());
    for source in &legacy_sources {
        println!("cargo:rerun-if-changed={source}");
    }

    write_dynamic_list(&dynamic_list_path, &exported_symbols);
    emit_linux_link_args(&version_script, &dynamic_list_path);
    generate_bindings(&header, &include_dir, &bindings_path);

    let uv_functions = parse_uv_functions(&bindings_path);
    generate_legacy_bindings(&legacy_bindings_path, &exported_symbols, &uv_functions);
    generate_private_bindings(&private_bindings_path, &exported_symbols, &uv_functions);
    generate_ffi_exports(&ffi_exports_path, &exported_symbols, &uv_functions);
    generate_ffi_legacy_aliases(&ffi_legacy_aliases_path, &uv_functions);

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let production_non_rust_sources: Vec<String> = Vec::new();
    let generated_production_non_rust_sources: Vec<String> = Vec::new();
    if target_os == "linux" {
        emit_linux_native_link_libs();
    }

    write_build_manifest(
        &build_manifest_path,
        &legacy_sources,
        &production_non_rust_sources,
        &generated_production_non_rust_sources,
    );
}

fn emit_linux_link_args(version_script: &Path, dynamic_list: &Path) {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("linux") {
        return;
    }

    println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libuv.so.1");
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,--version-script={}",
        version_script.display()
    );
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,--export-dynamic-symbol-list={}",
        dynamic_list.display()
    );
}

fn emit_linux_native_link_libs() {
    for lib in ["pthread", "dl", "rt"] {
        println!("cargo:rustc-link-lib={lib}");
    }
}

fn generate_bindings(header: &Path, include_dir: &Path, bindings_path: &Path) {
    let bindings = bindgen::Builder::default()
        .header(header.display().to_string())
        .clang_arg(format!("-I{}", include_dir.display()))
        .clang_arg("-D_GNU_SOURCE")
        .clang_arg("-D_LARGEFILE_SOURCE")
        .clang_arg("-D_FILE_OFFSET_BITS=64")
        .allowlist_function("^uv[_A-Za-z0-9]*$")
        .allowlist_type("^uv[_A-Za-z0-9]*$")
        .allowlist_var("^UV[_A-Za-z0-9]*$")
        .ctypes_prefix("libc")
        .generate_comments(false)
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate libuv bindings");

    bindings
        .write_to_file(bindings_path)
        .expect("failed to write generated bindings");
}

fn parse_uv_functions(bindings_path: &Path) -> BTreeMap<String, ForeignItemFn> {
    let syntax =
        parse_file(&fs::read_to_string(bindings_path).expect("failed to read generated bindings"))
            .expect("failed to parse generated bindings");

    let mut functions = BTreeMap::new();
    for item in syntax.items {
        let Item::ForeignMod(foreign_mod) = item else {
            continue;
        };
        for foreign_item in foreign_mod.items {
            let ForeignItem::Fn(function) = foreign_item else {
                continue;
            };
            let name = function.sig.ident.to_string();
            if name.starts_with("uv_") {
                functions.insert(name, function);
            }
        }
    }
    functions
}

fn generate_legacy_bindings(
    output_path: &Path,
    exported_symbols: &[String],
    uv_functions: &BTreeMap<String, ForeignItemFn>,
) {
    let mut declarations = Vec::with_capacity(exported_symbols.len());

    for symbol in exported_symbols {
        if VARIADIC_EXPORTS.contains(&symbol.as_str()) {
            continue;
        }

        let mut function = uv_functions
            .get(symbol)
            .unwrap_or_else(|| panic!("missing binding declaration for {symbol}"))
            .clone();
        let link_name = syn::LitStr::new(
            &format!("uv_legacy_{symbol}"),
            proc_macro2::Span::call_site(),
        );
        function.attrs = vec![parse_quote!(#[link_name = #link_name])];
        declarations.push(function);
    }

    let tokens = quote! {
        unsafe extern "C" {
            #(#declarations)*
        }
    };

    fs::write(output_path, tokens.to_string()).expect("failed to write legacy bindings");
}

fn generate_private_bindings(
    output_path: &Path,
    exported_symbols: &[String],
    uv_functions: &BTreeMap<String, ForeignItemFn>,
) {
    let mut declarations = Vec::with_capacity(exported_symbols.len());

    for symbol in exported_symbols {
        if VARIADIC_EXPORTS.contains(&symbol.as_str()) {
            continue;
        }

        let mut function = uv_functions
            .get(symbol)
            .unwrap_or_else(|| panic!("missing binding declaration for {symbol}"))
            .clone();
        let link_name = syn::LitStr::new(
            &format!("uv_phase5_private_{symbol}"),
            proc_macro2::Span::call_site(),
        );
        function.attrs = vec![parse_quote!(#[link_name = #link_name])];
        declarations.push(function);
    }

    let tokens = quote! {
        unsafe extern "C" {
            #(#declarations)*
        }
    };

    fs::write(output_path, tokens.to_string()).expect("failed to write private bindings");
}

fn generate_ffi_exports(
    output_path: &Path,
    exported_symbols: &[String],
    uv_functions: &BTreeMap<String, ForeignItemFn>,
) {
    let mut wrappers = Vec::new();
    let rust_exports = RUST_EXPORTS
        .iter()
        .chain(PHASE5_RUST_EXPORTS.iter())
        .copied()
        .collect::<BTreeSet<_>>();
    let private_forward_exports = PRIVATE_FORWARD_EXPORTS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();

    for symbol in exported_symbols {
        if VARIADIC_EXPORTS.contains(&symbol.as_str()) {
            continue;
        }
        if rust_exports.contains(symbol.as_str()) {
            continue;
        }

        let function = uv_functions
            .get(symbol)
            .unwrap_or_else(|| panic!("missing binding declaration for {symbol}"));
        if function.sig.variadic.is_some() {
            panic!("variadic export {symbol} is not handled by Rust code generation");
        }

        let ident = &function.sig.ident;
        let inputs = &function.sig.inputs;
        let output = &function.sig.output;
        let args = function
            .sig
            .inputs
            .iter()
            .map(argument_pattern_ident)
            .collect::<Vec<_>>();

        if private_forward_exports.contains(symbol.as_str()) {
            wrappers.push(quote! {
                #[no_mangle]
                pub unsafe extern "C" fn #ident(#inputs) #output {
                    crate::private_support::#ident(#(#args),*)
                }
            });
        } else {
            wrappers.push(quote! {
                #[no_mangle]
                pub unsafe extern "C" fn #ident(#inputs) #output {
                    crate::legacy::#ident(#(#args),*)
                }
            });
        }
    }

    let tokens = quote! {
        #(#wrappers)*
    };

    fs::write(output_path, tokens.to_string()).expect("failed to write ffi exports");
}

fn generate_ffi_legacy_aliases(output_path: &Path, uv_functions: &BTreeMap<String, ForeignItemFn>) {
    let mut aliases = String::new();
    let alias_exports = LEGACY_ALIAS_EXPORTS
        .iter()
        .chain(PHASE5_LEGACY_ALIAS_EXPORTS.iter())
        .copied()
        .collect::<BTreeSet<_>>();

    for symbol in alias_exports {
        uv_functions
            .get(symbol)
            .unwrap_or_else(|| panic!("missing binding declaration for {symbol}"));
        aliases.push_str(&format!(
            ".globl uv_legacy_{symbol}\n\
             .type uv_legacy_{symbol}, @function\n\
uv_legacy_{symbol}:\n\
             jmp {symbol}\n\
             .size uv_legacy_{symbol}, .-uv_legacy_{symbol}\n"
        ));
    }

    let tokens = format!("std::arch::global_asm!(r#\"\n{aliases}\"#);\n");

    fs::write(output_path, tokens.to_string()).expect("failed to write ffi legacy aliases");
}

fn argument_pattern_ident(argument: &FnArg) -> syn::Ident {
    let FnArg::Typed(pattern) = argument else {
        panic!("unexpected receiver in generated binding");
    };

    match pattern.pat.as_ref() {
        Pat::Ident(ident) => ident.ident.clone(),
        other => panic!(
            "unsupported argument pattern in generated binding: {}",
            quote! { #other }
        ),
    }
}

fn write_build_manifest(
    output_path: &Path,
    legacy_manifest_sources: &[String],
    production_non_rust_sources: &[String],
    generated_production_non_rust_sources: &[String],
) {
    let json = format!(
        concat!(
            "{{\n",
            "  \"generated_production_non_rust_sources\": {},\n",
            "  \"legacy_manifest_sources\": {},\n",
            "  \"production_non_rust_sources\": {}\n",
            "}}\n"
        ),
        json_array(generated_production_non_rust_sources),
        json_array(legacy_manifest_sources),
        json_array(production_non_rust_sources),
    );

    fs::write(output_path, json).expect("failed to write libuv build manifest");
}

fn write_dynamic_list(output_path: &Path, exported_symbols: &[String]) {
    let mut body = String::from("{\n");
    for symbol in exported_symbols {
        body.push_str("  ");
        body.push_str(symbol);
        body.push_str(";\n");
    }
    body.push_str("};\n");
    fs::write(output_path, body).expect("failed to write dynamic list");
}

fn json_array(values: &[String]) -> String {
    let body = values
        .iter()
        .map(|value| format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\"")))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{body}]")
}

fn read_sorted_lines(path: &Path) -> Vec<String> {
    let mut lines = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    if lines.windows(2).any(|window| window[0] >= window[1]) {
        lines.sort();
        lines.dedup();
        panic!("{} must be sorted and unique", path.display());
    }

    lines
}

fn read_legacy_sources(path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));

    let mut sources = Vec::new();
    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(stripped) = trimmed
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix("\","))
        {
            sources.push(stripped.to_string());
            continue;
        }
        if let Some(stripped) = trimmed
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
        {
            sources.push(stripped.to_string());
        }
    }

    if sources.windows(2).any(|window| window[0] >= window[1]) {
        panic!(
            "{} must contain sorted, unique source paths",
            path.display()
        );
    }
    if sources
        .iter()
        .any(|source| !Path::new(source).is_absolute())
    {
        panic!("{} must contain absolute source paths", path.display());
    }

    sources
}
