This harness compiles a focused upstream runner against a staged Rust-built `libuv` without modifying the upstream tree.

`safe/tools/build_upstream_harness.sh` is the entrypoint. It:

- consumes the checked-in `original/build-checker*` inventories in place and verifies the expected `440/440/55` vs `435/435/55` delta
- validates the selected phase tests against the current `original/test/test-list.h`
- generates a reduced runner source list for the current core-loop phase
- configures and builds this CMake harness against the staged library

The CMake project builds:

- `uv_safe_run_tests_shared`
- `uv_safe_run_tests_static`
- `uv_safe_benchmark_sizes_shared`
- `uv_safe_benchmark_sizes_static`

Use the scripts in `safe/tools/` instead of invoking this directory directly.
