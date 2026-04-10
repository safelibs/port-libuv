# Upstream Test Tree Port

This directory carries a direct copy of the current workspace `original/test/**`
tree under [`test/`](/home/yans/safelibs/port-libuv/safe/tests/upstream/test),
plus Linux CMake glue that links the copied upstream runners against the staged
Rust `libuv` install instead of rebuilding the original C library.

The harness preserves the upstream runner layout used by the checked-in
`original/build-checker*` trees:

- `uv_run_tests`
- `uv_run_tests_a`
- `uv_run_benchmarks_a`
- CTest entries: `uv_test`, `uv_test_a`

Behavior preserved from the workspace copy:

- Debian/Ubuntu-patched `test/**` sources are copied as-is.
- CTest keeps serialized execution with `RUN_SERIAL`.
- The test environment sets `UV_TEST_TIMEOUT_MULTIPLIER=2`.
- The test environment sets `RES_OPTIONS=attempts:0`.

Typical flow:

```bash
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
cmake -S /home/yans/safelibs/port-libuv/safe/tests/upstream -B /tmp/libuv-safe-ported-tests -DLIBUV_SAFE_STAGE=/tmp/libuv-safe-stage
cmake --build /tmp/libuv-safe-ported-tests -j"$(nproc)"
```
