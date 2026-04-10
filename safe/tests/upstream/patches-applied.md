# Applied Patches

Phase `impl-07-test-port-and-abi-audit` copies the current workspace
`original/test/**` tree directly into
[`safe/tests/upstream/test/`](/home/yans/safelibs/port-libuv/safe/tests/upstream/test).

That means the copied sources already include the Debian/Ubuntu behavior present
in this repository snapshot, including:

- serialized execution at the harness level
- `UV_TEST_TIMEOUT_MULTIPLIER=2`
- `RES_OPTIONS=attempts:0`
- the embedded skip behavior in `test-getnameinfo.c`
- the embedded skip behavior in `test-udp-multicast-join.c`
- the embedded skip behavior in `test-udp-multicast-join6.c`
- the embedded architecture guard in `test-tcp-oob.c`

No additional test-logic patches were applied while porting the tree. The only
phase-7 changes in this directory are provenance notes and the top-level
`CMakeLists.txt` that swaps the original `uv` / `uv_a` build targets for the
staged Rust library artifacts.
