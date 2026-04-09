This harness compiles a narrow upstream smoke slice against a staged Rust-built `libuv` without touching the upstream source tree.

Use it after:

```bash
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
cmake -S /home/yans/safelibs/port-libuv/safe/tests/harness -B /tmp/libuv-safe-harness -DSTAGE_PREFIX=/tmp/libuv-safe-stage
cmake --build /tmp/libuv-safe-harness
```

The harness intentionally links direct wrappers around upstream `original/test/test-getters-setters.c` and `original/test/benchmark-sizes.c`. That keeps the smoke target small while still compiling real upstream test sources against the staged headers and libraries.
