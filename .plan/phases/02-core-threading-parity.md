# Core Loop, Default Loop, Threading, Async, and Metrics Parity

Phase Name: Core Loop, Default Loop, Threading, Async, and Metrics Parity

Implement Phase ID: `impl-02-core-threading-parity`

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
- `/home/yans/safelibs/port-libuv/safe/tests/regressions/manifest.json`
- `/home/yans/safelibs/port-libuv/safe/tests/harness/README.md`
- `/home/yans/safelibs/port-libuv/safe/tests/upstream/README.md`
- `/home/yans/safelibs/port-libuv/safe/COMPATIBILITY_NOTES.md`
- `/home/yans/safelibs/port-libuv/.cargo/config.toml`
- `/home/yans/safelibs/port-libuv/safe/Cargo.toml`
- `/home/yans/safelibs/port-libuv/safe/build.rs`
- `/home/yans/safelibs/port-libuv/safe/src/**`
- `/home/yans/safelibs/port-libuv/safe/include/**`
- `/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/audit_unsafe.py`
- `/home/yans/safelibs/port-libuv/safe/tests/harness/**`
- `/home/yans/safelibs/port-libuv/safe/tests/regressions/**`
- `/home/yans/safelibs/port-libuv/original/src/uv-common.c`
- `/home/yans/safelibs/port-libuv/original/src/timer.c`
- `/home/yans/safelibs/port-libuv/original/src/thread-common.c`
- `/home/yans/safelibs/port-libuv/original/src/threadpool.c`
- `/home/yans/safelibs/port-libuv/original/src/random.c`
- `/home/yans/safelibs/port-libuv/original/build-checker/**`
- `/home/yans/safelibs/port-libuv/original/test/**`

## New Outputs
- updated `/home/yans/safelibs/port-libuv/safe/src/core/*.rs`
- updated `/home/yans/safelibs/port-libuv/safe/src/threading/*.rs`
- updated `/home/yans/safelibs/port-libuv/safe/src/unix/async.rs`
- updated `/home/yans/safelibs/port-libuv/safe/src/exports/mod.rs`
- updated `/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh` if the focused harness needs additional core/threading coverage
- updated `/home/yans/safelibs/port-libuv/safe/tests/harness/CMakeLists.txt` if focused coverage requires additional upstream source files
- updated fallback templates under `/home/yans/safelibs/port-libuv/safe/tests/harness/**` if the `LIBUV_SAFE_NO_PYTHON=1` path must stay aligned with the generator
- new or updated core/threading regressions under `/home/yans/safelibs/port-libuv/safe/tests/regressions/**` if a focused failure requires a smaller reproducer
- updated `/home/yans/safelibs/port-libuv/safe/tests/regressions/manifest.json` whenever regressions change

## File Changes
- Fix loop lifecycle, default-loop storage, timer scheduling, async wakeups, threadpool scheduling and cancellation, random APIs, and metrics behavior.
- Keep public accessors and request or handle metadata aligned with the C ABI.

## Implementation Details
- Consume the phase-1 verification helpers and regression manifest in place. Do not replace them with alternate runner or history-validation paths.
- Keep `LoopState` rooted in `uv_loop_t.internal_fields` as already established in `safe/src/core/mod.rs` and `safe/src/core/loop.rs`.
- Preserve the default-loop storage strategy in `safe/src/core/default_loop.rs` rather than introducing a parallel loop singleton.
- Keep threadpool task bookkeeping anchored through `uv_req_t.reserved[0]` as already done in `safe/src/threading/threadpool.rs`.
- Tighten:
  - `uv_loop_init`, `uv_loop_close`, `uv_loop_new`, `uv_loop_delete`, `uv_run`, `uv_now`, `uv_backend_fd`, `uv_backend_timeout`
  - `uv_async_init`, `uv_async_send`, and internal async wake or shutdown paths
  - `uv_thread_*`, `uv_mutex_*`, `uv_rwlock_*`, `uv_cond_*`, `uv_sem_*`, `uv_barrier_*`, `uv_once`
  - `uv_queue_work`, `uv_cancel`, `uv_random`
  - `uv_metrics_idle_time`, `uv_metrics_info`
  - manual getters and type-name exports in `safe/src/exports/mod.rs`
- If focused failures reveal missing focused-harness coverage, update the selected-test generation in `safe/tools/build_upstream_harness.sh` and the owning source list in `safe/tests/harness/CMakeLists.txt` together; if that changes the no-Python fallback path, keep `safe/tests/harness/phase-test-list.h` and `safe/tests/harness/uv-safe-run-tests.c` synchronized instead of creating a second focused harness.
- If this phase adds a smaller checked-in reproducer under `safe/tests/regressions/**`, register it in `safe/tests/regressions/manifest.json` in the same commit with `phase_owner: impl-02-core-threading-parity` so `check-04` executes it automatically.
- Do not add `unsafe fn`; keep unsafe inside small blocks with the existing `SAFETY(...)` categories.

## Verification Phases
### `check-03-core-threading-focused-suite`
- Phase ID: `check-03-core-threading-focused-suite`
- Type: `check`
- Fixed `bounce_target`: `impl-02-core-threading-parity`
- Purpose: run the focused harness against loop, async, thread, threadpool, random, and metrics behavior.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-02-core-threading-parity
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-core
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-core --shared --tests version,loop_new_delete,loop_alive,loop_close,loop_stop,loop_configure,run_once,run_nowait,once,async,handle_type_name,req_type_name,thread_create,thread_local_storage,thread_stack_size,thread_stack_size_explicit,thread_mutex,thread_mutex_recursive,thread_rwlock,thread_rwlock_trylock,threadpool_queue_work_simple,threadpool_queue_work_einval,threadpool_multiple_event_loops,threadpool_cancel_work,threadpool_cancel_single,threadpool_cancel_when_busy,random_async,random_sync,barrier_1,barrier_2,barrier_3,condvar_1,condvar_2,semaphore_1,semaphore_2,metrics_idle_time,metrics_idle_time_thread,metrics_idle_time_zero,metrics_info_check,metrics_pool_events
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-core --static --tests version,loop_new_delete,run_once,threadpool_queue_work_simple,random_sync,metrics_info_check
```

### `check-04-core-threading-safety-review`
- Phase ID: `check-04-core-threading-safety-review`
- Type: `check`
- Fixed `bounce_target`: `impl-02-core-threading-parity`
- Purpose: review the core and threading changes for localized unsafe boundaries and re-prove focused-harness fallback parity, link-smoke compatibility, and manifest-backed regression execution.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-02-core-threading-parity
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-core-parity
LIBUV_SAFE_NO_PYTHON=1 /home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-core-parity-nopy
diff -u /tmp/libuv-safe-core-parity/generated/phase-test-list.h /tmp/libuv-safe-core-parity-nopy/generated/phase-test-list.h
diff -u /tmp/libuv-safe-core-parity/generated/uv-safe-run-tests.c /tmp/libuv-safe-core-parity-nopy/generated/uv-safe-run-tests.c
diff -u /tmp/libuv-safe-core-parity/generated/benchmark-sizes-main.c /tmp/libuv-safe-core-parity-nopy/generated/benchmark-sizes-main.c
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-core-parity-nopy --shared --tests version,run_once,async,threadpool_queue_work_simple,random_sync,metrics_info_check
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-core-parity-nopy --static --tests version,run_once,threadpool_queue_work_simple
/home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh --shared --run-smoke run_once --build-dir /home/yans/safelibs/port-libuv/original/build-checker --stage /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh --static --run-smoke run_once --build-dir /home/yans/safelibs/port-libuv/original/build-checker --stage /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/run_regressions.sh --stage /tmp/libuv-safe-stage --up-to-phase impl-02-core-threading-parity
python3 /home/yans/safelibs/port-libuv/safe/tools/audit_unsafe.py /home/yans/safelibs/port-libuv/safe/src
! rg -n "unsafe fn" /home/yans/safelibs/port-libuv/safe/src
```

- Review checks:
  - Confirm loop state stays rooted in `uv_loop_t.internal_fields` and task state stays rooted in `uv_req_t.reserved[0]`.
  - Reject broad new unsafe regions in `safe/src/core/**`, `safe/src/threading/**`, and `safe/src/unix/async.rs`.
  - Confirm any new file added under `safe/tests/regressions/**` is registered in `safe/tests/regressions/manifest.json`.

## Success Criteria
- `check-03-core-threading-focused-suite` passes the loop, async, thread, threadpool, random, and metrics subset.
- `check-04-core-threading-safety-review` passes the unsafe audit, fallback-parity rerun, relink smoke, and regression manifest execution.
- Any harness or fallback-template changes remain synchronized under the existing focused-harness contract.
- The latest commit produced by this phase begins with `impl-02-core-threading-parity`.

## Git Commit Requirement
The implementer must commit the phase work to git before yielding. The newest commit at `HEAD` must remain the handoff commit for this phase, and its subject must begin with `impl-02-core-threading-parity`.
