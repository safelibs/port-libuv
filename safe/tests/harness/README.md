This harness compiles a focused upstream runner against a staged Rust-built `libuv` without modifying the checked-in upstream or prebuilt runner trees.

`safe/tools/build_upstream_harness.sh` owns the focused selected-test inventory and is the only supported entrypoint. It:

- consumes the checked-in `original/build-checker*` inventories in place and verifies the expected `440/440/55` vs `435/435/55` delta
- validates the selected phase tests against the current `original/test/test-list.h`
- generates the reduced focused harness sources under `<build>/generated`, including `phase-test-list.h`, `uv-safe-run-tests.c`, and `benchmark-sizes-main.c`
- configures and builds this CMake harness against the staged library

The CMake project builds:

- `uv_safe_run_tests_shared`
- `uv_safe_run_tests_static`
- `uv_safe_benchmark_sizes_shared`
- `uv_safe_benchmark_sizes_static`

When `python3` is unavailable, callers must set `LIBUV_SAFE_NO_PYTHON=1`. In that mode the checked-in `safe/tests/harness/phase-test-list.h` and `safe/tests/harness/uv-safe-run-tests.c` act as fallback templates, and `check-02` diffs them against the generator-backed outputs.

Edit ownership for the focused harness is fixed:

- `impl-01-baseline-and-artifact-policy`, `impl-02-core-threading-parity`, and `impl-03-io-fs-resolver-parity` may edit `safe/tools/build_upstream_harness.sh`, `safe/tests/harness/CMakeLists.txt`, and the checked-in fallback templates under `safe/tests/harness/**`
- `impl-04-process-signal-security` through `impl-07-dependent-closure-and-release-hardening` must treat those files as frozen contract inputs

Use the scripts in `safe/tools/` instead of invoking this directory directly.
