# Stream, Socket, FS, Resolver, and OS Parity

Phase Name: Stream, Socket, FS, Resolver, and OS Parity

Implement Phase ID: `impl-03-io-fs-resolver-parity`

## Preexisting Inputs
- `/home/yans/safelibs/port-libuv/.plan/workflow-run-base.txt`
- `/home/yans/safelibs/port-libuv/.gitignore`
- `/home/yans/safelibs/port-libuv/safe/.gitignore`
- `/home/yans/safelibs/port-libuv/original/.gitignore`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_phase_commit_sequence.py`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_stage_install.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_runner_inventory.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/run_regressions.sh`
- `/home/yans/safelibs/port-libuv/safe/tests/harness/README.md`
- `/home/yans/safelibs/port-libuv/safe/tests/upstream/README.md`
- `/home/yans/safelibs/port-libuv/safe/tests/regressions/manifest.json`
- `/home/yans/safelibs/port-libuv/safe/COMPATIBILITY_NOTES.md`
- `/home/yans/safelibs/port-libuv/.cargo/config.toml`
- `/home/yans/safelibs/port-libuv/safe/Cargo.toml`
- `/home/yans/safelibs/port-libuv/safe/build.rs`
- `/home/yans/safelibs/port-libuv/safe/src/**`
- `/home/yans/safelibs/port-libuv/safe/include/**`
- `/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/audit_unsafe.py`
- `/home/yans/safelibs/port-libuv/safe/tests/harness/**`
- `/home/yans/safelibs/port-libuv/safe/tests/regressions/**`
- `/home/yans/safelibs/port-libuv/safe/tests/regressions/fs_readlink_proc_self.c`
- `/home/yans/safelibs/port-libuv/safe/tests/regressions/getaddrinfo_long_hostname.c`
- `/home/yans/safelibs/port-libuv/original/src/unix/*.c`
- `/home/yans/safelibs/port-libuv/original/test/**`

## New Outputs
- updated `/home/yans/safelibs/port-libuv/safe/src/unix/*.rs` for I/O, fs, resolver, and OS helpers
- updated `/home/yans/safelibs/port-libuv/safe/src/util/idna.rs`
- updated `/home/yans/safelibs/port-libuv/safe/src/util/inet.rs`
- updated `/home/yans/safelibs/port-libuv/safe/tests/regressions/**` if smaller reproductions are needed for a newly discovered bug
- updated `/home/yans/safelibs/port-libuv/safe/tests/regressions/manifest.json` whenever regressions change
- updated `/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh` if the focused harness must add missing coverage
- updated `/home/yans/safelibs/port-libuv/safe/tests/harness/CMakeLists.txt` if focused coverage requires additional upstream source files
- updated fallback templates under `/home/yans/safelibs/port-libuv/safe/tests/harness/**` if the `LIBUV_SAFE_NO_PYTHON=1` path must stay aligned with the generator

## File Changes
- Fix stream and socket read or write semantics, pipe and tty behavior, UDP corner cases, filesystem request ownership and cleanup, watcher ownership, resolver behavior, and Linux OS-helper behavior.
- Preserve or extend the existing regression programs for the `/proc/self` and long-hostname issues.

## Implementation Details
- Consume the existing checked-in regressions in place. Do not replace `safe/tests/regressions/fs_readlink_proc_self.c` or `safe/tests/regressions/getaddrinfo_long_hostname.c` with prose-only notes or ad hoc one-off checks.
- Keep the request-slot ownership model in `safe/src/unix/fs.rs` rather than inventing another side table. The `reserved[]` slots, `clone_path_into`, `copy_bufs`, and `cleanup_owned_allocations` logic are the right place to fix fs cleanup and accessor bugs.
- Fix `uv_fs_req_cleanup`, `uv_fs_get_type`, `uv_fs_get_result`, `uv_fs_get_ptr`, `uv_fs_get_statbuf`, and `uv_fs_get_path` semantics together; they share request lifetime and ownership.
- Preserve the existing translated ABI-heavy `safe/src/unix/stream.rs` file shape. Fix behavior in place rather than replacing it with a separate abstraction layer.
- Tighten socket and watcher behavior in `tcp.rs`, `pipe.rs`, `udp.rs`, `poll.rs`, `loop_watcher.rs`, `fs_event.rs`, and `fs_poll.rs`.
- Fix resolver and IDNA behavior in `getaddrinfo.rs`, `getnameinfo.rs`, and `util/idna.rs` so that the long-hostname regression passes without any fixed 256-byte staging buffer.
- If focused failures reveal missing harness coverage, update `safe/tools/build_upstream_harness.sh` and `safe/tests/harness/CMakeLists.txt` together, then keep the fallback templates under `safe/tests/harness/**` synchronized only when the no-Python path needs the same declarations.
- Keep `safe/tests/regressions/fs_readlink_proc_self.c` and `safe/tests/regressions/getaddrinfo_long_hostname.c` as checked-in regression anchors. If more failures appear, add more files beside them rather than creating ad hoc unmanifested checks, and register every new file in `safe/tests/regressions/manifest.json` with `phase_owner: impl-03-io-fs-resolver-parity` so `check-06` executes it.

## Verification Phases
### `check-05-io-fs-resolver-focused-suite`
- Phase ID: `check-05-io-fs-resolver-focused-suite`
- Type: `check`
- Fixed `bounce_target`: `impl-03-io-fs-resolver-parity`
- Purpose: exercise focused network, stream, fs, watcher, resolver, and OS-helper behavior through the staged harness.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-03-io-fs-resolver-parity
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-iofs
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-iofs --shared --tests poll_duplex,poll_unidirectional,poll_bad_fdtype,poll_nested_epoll,tcp_ping_pong,tcp_ping_pong_vec,pipe_ping_pong,pipe_ping_pong_vec,multiple_listen,connection_fail,shutdown_close_tcp,shutdown_close_pipe,shutdown_eof,shutdown_simultaneous,shutdown_twice,udp_bind,udp_connect,udp_connect6,udp_send_and_recv,udp_send_immediate,udp_send_unreachable,udp_try_send,udp_recv_in_a_row,udp_options,udp_options6,udp_no_autobind,udp_mmsg,udp_multicast_join,udp_multicast_join6,udp_multicast_interface,udp_multicast_interface6,udp_multicast_ttl,udp_dual_stack,udp_ipv6_only,udp_dgram_too_big,udp_open,udp_open_twice,udp_open_bound,udp_open_connect,pipe_connect_bad_name,pipe_connect_close_multiple,pipe_connect_multiple,pipe_connect_on_prepare,pipe_getsockname,pipe_pending_instances,pipe_sendmsg,pipe_server_close,pipe_set_non_blocking,pipe_set_chmod,tty,tty_file,tty_pty
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-iofs --shared --tests fs_file_noent,fs_file_nametoolong,fs_file_loop,fs_file_async,fs_file_sync,fs_async_dir,fs_async_sendfile,fs_async_sendfile_nodata,fs_copyfile,fs_mkdtemp,fs_mkstemp,fs_fstat,fs_access,fs_chmod,fs_chown,fs_link,fs_readlink,fs_realpath,fs_symlink,fs_utime,fs_futime,fs_lutime,fs_stat_missing_path,fs_scandir_empty_dir,fs_scandir_non_existent_dir,fs_scandir_file,fs_scandir_early_exit,fs_open_dir,fs_read_dir,fs_read_bufs,fs_read_file_eof,fs_write_multiple_bufs,fs_write_alotof_bufs,fs_write_alotof_bufs_with_offset,fs_partial_read,fs_partial_write,fs_read_write_null_arguments,fs_file_pos_after_op_with_offset,fs_file_open_append,fs_null_req,fs_rename_to_existing_file,fs_statfs,fs_get_system_error,fs_stat_batch_multiple,getters_setters,fs_poll,fs_poll_getpath,fs_poll_close_request,fs_poll_close_request_multi_start_stop,fs_poll_close_request_multi_stop_start,fs_poll_close_request_stop_when_active,fs_event_watch_dir,fs_event_watch_file,fs_event_watch_file_exact_path,fs_event_watch_file_current_dir,fs_event_watch_file_twice,fs_event_no_callback_after_close,fs_event_no_callback_on_close,fs_event_immediate_close,fs_event_close_with_pending_event,fs_event_close_with_pending_delete_event,fs_event_close_in_callback,fs_event_start_and_close,fs_event_error_reporting,fs_event_getpath,fs_event_watch_invalid_path,fs_event_stop_in_cb,getaddrinfo_fail,getaddrinfo_fail_sync,getaddrinfo_basic,getaddrinfo_basic_sync,getaddrinfo_concurrent,getnameinfo_basic_ip4,getnameinfo_basic_ip4_sync,getnameinfo_basic_ip6,gethostname,idna_toascii,utf8_decode1,utf8_decode1_overrun,ip4_addr,ip6_addr_link_local,ip_name,get_currentexe,get_loadavg,get_memory,get_passwd,homedir,tmpdir,cwd_and_chdir,env_vars,platform_output,uname,gettimeofday,hrtime
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-iofs --static --tests tcp_ping_pong,udp_send_and_recv,fs_file_async,getters_setters,getaddrinfo_basic
```

### `check-06-regression-probes-and-resolver-review`
- Phase ID: `check-06-regression-probes-and-resolver-review`
- Type: `check`
- Fixed `bounce_target`: `impl-03-io-fs-resolver-parity`
- Purpose: confirm manifest-backed regressions still cover the known fs or proc and long-hostname resolver edge cases and re-prove focused-harness fallback parity after any I/O-focused harness edits.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-03-io-fs-resolver-parity
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-iofs-parity
LIBUV_SAFE_NO_PYTHON=1 /home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-iofs-parity-nopy
diff -u /tmp/libuv-safe-iofs-parity/generated/phase-test-list.h /tmp/libuv-safe-iofs-parity-nopy/generated/phase-test-list.h
diff -u /tmp/libuv-safe-iofs-parity/generated/uv-safe-run-tests.c /tmp/libuv-safe-iofs-parity-nopy/generated/uv-safe-run-tests.c
diff -u /tmp/libuv-safe-iofs-parity/generated/benchmark-sizes-main.c /tmp/libuv-safe-iofs-parity-nopy/generated/benchmark-sizes-main.c
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-iofs-parity-nopy --shared --tests tcp_ping_pong,udp_send_and_recv,fs_file_async,getaddrinfo_basic
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-iofs-parity-nopy --static --tests tcp_ping_pong,fs_file_async,getaddrinfo_basic
/home/yans/safelibs/port-libuv/safe/tools/run_regressions.sh --stage /tmp/libuv-safe-stage --up-to-phase impl-03-io-fs-resolver-parity
! rg -n "hostname_ascii\\[256\\]" /home/yans/safelibs/port-libuv/safe/src
python3 /home/yans/safelibs/port-libuv/safe/tools/audit_unsafe.py /home/yans/safelibs/port-libuv/safe/src
```

- Review checks:
  - Confirm the implementation keeps using checked-in regression sources with manifest-backed execution instead of replacing them with prose-only notes.
  - Confirm any new file added under `safe/tests/regressions/**` is registered in `safe/tests/regressions/manifest.json`.
  - Confirm `uv_fs_req_cleanup`, `uv_fs_get_*`, and resolver or IDNA changes remain tied to the actual request and buffer ownership in `safe/src/unix/fs.rs` and `safe/src/unix/getaddrinfo.rs`.

## Success Criteria
- `check-05-io-fs-resolver-focused-suite` passes the focused network, stream, fs, watcher, resolver, and OS-helper subset.
- `check-06-regression-probes-and-resolver-review` passes the manifest-backed regressions, the fallback-parity rerun, and the unsafe audit.
- The existing checked-in regression anchors remain in place and any new executable regression is registered in the manifest in the same commit.
- The latest commit produced by this phase begins with `impl-03-io-fs-resolver-parity`.

## Git Commit Requirement
The implementer must commit the phase work to git before yielding. The newest commit at `HEAD` must remain the handoff commit for this phase, and its subject must begin with `impl-03-io-fs-resolver-parity`.
