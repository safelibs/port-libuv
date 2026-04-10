# Full Copied Upstream Suite, Export Surface, and Relink Stability

Phase Name: Full Copied Upstream Suite, Export Surface, and Relink Stability

Implement Phase ID: `impl-05-full-upstream-and-relink`

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
- `/home/yans/safelibs/port-libuv/safe/tools/verify_exports.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_sizes.sh`
- `/home/yans/safelibs/port-libuv/safe/tests/upstream/**`
- `/home/yans/safelibs/port-libuv/safe/tests/regressions/**`
- `/home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/update_bindings.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/abi-baseline.json`
- `/home/yans/safelibs/port-libuv/safe/tools/cc-linker.sh`
- `/home/yans/safelibs/port-libuv/original/build-checker/**`
- `/home/yans/safelibs/port-libuv/original/build-checker-review/**`
- `/home/yans/safelibs/port-libuv/original/build-checker-verify/**`
- `/home/yans/safelibs/port-libuv/original/build-checker-audit/**`
- `/home/yans/safelibs/port-libuv/original/debian/libuv1t64.symbols`

## New Outputs
- updated `/home/yans/safelibs/port-libuv/safe/tests/upstream/CMakeLists.txt` if full-suite build glue needs fixes
- updated `/home/yans/safelibs/port-libuv/safe/tests/upstream/README.md`
- updated `/home/yans/safelibs/port-libuv/safe/tests/upstream/patches-applied.md`
- updated `/home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh`
- updated `/home/yans/safelibs/port-libuv/safe/tools/update_bindings.sh` if export regeneration logic needs corrections
- updated `/home/yans/safelibs/port-libuv/safe/src/abi/linux_x86_64.rs`
- updated `/home/yans/safelibs/port-libuv/safe/src/exports/generated.rs`
- updated `/home/yans/safelibs/port-libuv/safe/src/exports/mod.rs`
- updated `/home/yans/safelibs/port-libuv/safe/tools/abi-baseline.json`
- updated `/home/yans/safelibs/port-libuv/safe/tools/cc-linker.sh`
- final cross-cutting fixes in `/home/yans/safelibs/port-libuv/safe/src/**` needed to get the full suite and relink matrix green

## File Changes
- Stabilize the full copied upstream suite.
- Stabilize the version-script, export, and bindgen surface.
- Stabilize relink behavior across all four prebuilt upstream build trees.

## Implementation Details
- Consume the existing copied upstream tree and the four authoritative build trees in place. Do not rebuild or rediscover alternative object baselines.
- Use `safe/tools/update_bindings.sh` if and only if the ABI surface actually drifts; do not hand-edit generated ABI declarations unless regeneration is impossible.
- Keep the export contract rooted in `safe/tools/abi-baseline.json`, `safe/tools/cc-linker.sh`, `safe/src/exports/generated.rs`, and `safe/src/exports/mod.rs`.
- Any full-suite failure that was not already covered by focused phases should be fixed in the real owning runtime module under `safe/src/**`, not papered over in CMake.
- Do not add new entries under `safe/tests/regressions/**` in this phase; the copied upstream suite plus the relink matrix are already the executable coverage contract for phase-5-only fixes.
- Preserve the copied-source provenance in `safe/tests/upstream/patches-applied.md`.
- Keep the current-source inventory tied to the three `440 / 440 / 55` baselines and the object-relink inventory tied to `original/build-checker`.

## Verification Phases
### `check-09-copied-upstream-suite`
- Phase ID: `check-09-copied-upstream-suite`
- Type: `check`
- Fixed `bounce_target`: `impl-05-full-upstream-and-relink`
- Purpose: run the full copied upstream suite against the staged Rust library and assert the current-source `440 / 440 / 55` inventories.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-05-full-upstream-and-relink
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
cmake -S /home/yans/safelibs/port-libuv/safe/tests/upstream -B /tmp/libuv-safe-ported-tests -DLIBUV_SAFE_STAGE=/tmp/libuv-safe-stage
cmake --build /tmp/libuv-safe-ported-tests -j"$(nproc)"
diff -u <(/tmp/libuv-safe-ported-tests/uv_run_tests --list | sort) <(/home/yans/safelibs/port-libuv/original/build-checker-review/uv_run_tests --list | sort)
diff -u <(/tmp/libuv-safe-ported-tests/uv_run_tests_a --list | sort) <(/home/yans/safelibs/port-libuv/original/build-checker-review/uv_run_tests_a --list | sort)
diff -u <(/tmp/libuv-safe-ported-tests/uv_run_benchmarks_a --list | sort) <(/home/yans/safelibs/port-libuv/original/build-checker-review/uv_run_benchmarks_a --list | sort)
test "$(/tmp/libuv-safe-ported-tests/uv_run_tests --list | awk 'END { print NR }')" = 440
test "$(/tmp/libuv-safe-ported-tests/uv_run_tests_a --list | awk 'END { print NR }')" = 440
test "$(/tmp/libuv-safe-ported-tests/uv_run_benchmarks_a --list | awk 'END { print NR }')" = 55
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 ctest --test-dir /tmp/libuv-safe-ported-tests --output-on-failure
```

### `check-10-relinked-original-runners`
- Phase ID: `check-10-relinked-original-runners`
- Type: `check`
- Fixed `bounce_target`: `impl-05-full-upstream-and-relink`
- Purpose: prove link compatibility by relinking and running the prebuilt upstream object files from all four authoritative build trees.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-05-full-upstream-and-relink
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/verify_exports.sh /tmp/libuv-safe-stage/lib/libuv.so /home/yans/safelibs/port-libuv/original/build-checker/libuv.so.1.0.0 /home/yans/safelibs/port-libuv/original/debian/libuv1t64.symbols
/home/yans/safelibs/port-libuv/safe/tools/verify_sizes.sh /tmp/libuv-safe-stage
for build in /home/yans/safelibs/port-libuv/original/build-checker /home/yans/safelibs/port-libuv/original/build-checker-review /home/yans/safelibs/port-libuv/original/build-checker-verify /home/yans/safelibs/port-libuv/original/build-checker-audit; do /home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh --shared --run --build-dir "$build" --stage /tmp/libuv-safe-stage; done
for build in /home/yans/safelibs/port-libuv/original/build-checker /home/yans/safelibs/port-libuv/original/build-checker-review /home/yans/safelibs/port-libuv/original/build-checker-verify /home/yans/safelibs/port-libuv/original/build-checker-audit; do /home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh --static --run --build-dir "$build" --stage /tmp/libuv-safe-stage; done
```

- Review checks:
  - Preserve the legacy `build-checker` skip set already encoded in `relink_original_tests.sh`; do not silently promote the `435 / 435 / 55` tree into the `440 / 440 / 55` baseline.
  - Treat `.safe-relink/**` as transient outputs inside the existing build trees, not as release artifacts.

## Success Criteria
- `check-09-copied-upstream-suite` passes the full copied upstream suite with the `440 / 440 / 55` inventory.
- `check-10-relinked-original-runners` passes exports, size checks, and full relink runs across all four authoritative build trees.
- The export surface remains rooted in the existing ABI baseline, linker wrapper, and generated-wrapper files.
- The latest commit produced by this phase begins with `impl-05-full-upstream-and-relink`.

## Git Commit Requirement
The implementer must commit the phase work to git before yielding. The newest commit at `HEAD` must remain the handoff commit for this phase, and its subject must begin with `impl-05-full-upstream-and-relink`.
