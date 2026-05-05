# Packaging, ABI, Header, and Validator Setup Fixes

Phase Name: Packaging, ABI, Header, and Validator Setup Fixes

Implement Phase ID: `impl-15-validator-packaging-abi-fixes`

## Preexisting Inputs

- `.plan/plan.md` as the immutable source plan.
- `.plan/workflow-run-base.txt` written and committed by `impl-14-validator-refresh-baseline`.
- `.gitignore` containing `/validator/`.
- `validator-report.md` updated by `impl-14-validator-refresh-baseline`, including validator commit, baseline commands, case counts, override coverage, and failure owner assignments.
- Refreshed validator checkout: `validator/.git`, `validator/README.md`, `validator/Makefile`, `validator/test.sh`, `validator/repositories.yml`, and `validator/tests/libuv/**`.
- Phase 1 baseline artifacts: `validator/artifacts/libuv-safe/phase-14/results/libuv/summary.json`, all non-summary JSON result files in `validator/artifacts/libuv-safe/phase-14/results/libuv/*.json`, and `validator/artifacts/libuv-safe/phase-14/local-debs/libuv/*.deb`.
- Historical comparison artifacts: `validator/artifacts/libuv-safe/phase-08/**`, `validator/artifacts/libuv-safe/phase-09/**`, `validator/artifacts/libuv-safe/phase-10/**`, `validator/artifacts/libuv-safe/phase-11/**`, `validator/artifacts/libuv-safe/phase-12/**`, `validator/artifacts/libuv-safe/check-25-final/**`, and `validator/site/libuv-safe-final/**`.
- Latest package build metadata and packages: `safe/dist/artifacts.env`, `safe/dist/libuv1t64_*.deb`, and `safe/dist/libuv1-dev_*.deb`.
- Rust crate/build inputs: `safe/Cargo.toml`, `safe/Cargo.lock`, `safe/rust-toolchain.toml`, `safe/vendor/libc-0.2.184/**`, and `safe/src/lib.rs`.
- Package and ABI inputs: `safe/build.rs`, `safe/debian/**`, `safe/include/**`, `safe/src/exports/mod.rs`, `safe/src/exports/generated.rs`, and `safe/src/abi/linux_x86_64.rs`.
- Build, package, ABI, audit, and regression tools: `safe/tools/build_deb.sh`, `safe/tools/stage_install.sh`, `safe/tools/verify_stage_install.sh`, `safe/tools/verify_exports.sh`, `safe/tools/verify_deb_payload_contract.py`, `safe/tools/render_debian_symbols.py`, `safe/tools/audit_unsafe.py`, `safe/tools/run_regressions.sh`, `safe/tools/verify_phase_head.sh`, and `safe/tools/verify_phase_commit_sequence.py`.
- Regression inventory: `safe/tests/regressions/manifest.json` and `safe/tests/regressions/**`.
- Original package/reference inputs: `original/include/**`, `original/debian/libuv1t64.symbols`, `original/debian/not-installed`, and `original/build-checker/libuv.so.1.0.0` if already materialized.
- Prepared inputs to consume in place: `all_cves.json`, `relevant_cves.json`, `dependents.json`, `safe/tests/upstream/**`, `safe/tests/harness/**`, `safe/tests/dependents/**`, and `safe/docker/**`.

## New Outputs

- Packaging/ABI regression tests under `safe/tests/regressions/` if failures are found.
- Updated `safe/tests/regressions/manifest.json` entries with `phase_owner: "impl-15-validator-packaging-abi-fixes"`.
- Updated packaging, export, or ABI files if required.
- `validator/artifacts/libuv-safe/phase-15/**` implementation-run artifacts.
- Updated `validator-report.md`.

## File Changes

- `validator-report.md`
- Conditional: `safe/tests/regressions/manifest.json`
- Conditional: new `safe/tests/regressions/validator_<testcase>_<issue>.c` or `.sh`
- Conditional: `safe/debian/control`, `safe/debian/*.install`, `safe/debian/rules`, `safe/debian/not-installed`, `safe/debian/libuv1t64.symbols`
- Conditional: `safe/build.rs`, `safe/tools/build_deb.sh`, `safe/tools/stage_install.sh`, `safe/tools/verify_exports.sh`, `safe/tools/verify_deb_payload_contract.py`, `safe/tools/render_debian_symbols.py`
- Conditional: `safe/src/exports/mod.rs`, `safe/src/exports/generated.rs`, `safe/src/abi/linux_x86_64.rs`
- Do not modify tracked files under `validator/`
- Do not commit generated `validator/artifacts/**`, `safe/dist/**`, `safe/target/**`, root-level package artifacts, or `original/build-checker/**`

## Implementation Details

- Own failures where logs show `dpkg` install errors, missing headers, missing `libuv.pc` or `libuv-static.pc`, compile/link failures, incorrect SONAME, wrong package names, wrong multiarch paths, missing symbols, size or ABI mismatch, or a validator case loading Ubuntu's stock libuv instead of local debs.
- Preserve package names `libuv1t64` and `libuv1-dev`, Ubuntu-compatible multiarch library paths, SONAME `libuv.so.1`, and headers byte-for-byte aligned with `original/include`.
- If an export is missing, add the exact `extern "C"` wrapper in `safe/src/exports/generated.rs` or `safe/src/exports/mod.rs`, using the signature from `safe/include/uv.h` and delegating to the existing Rust module.
- Update ABI layout in `safe/src/abi/linux_x86_64.rs` only when the public header and existing layout definitions prove the change is required.
- Preserve `safe/tools/render_debian_symbols.py` as the Debian symbols rendering helper for ABI/package checks; update it only for a proven symbols-generation defect.
- Add the smallest regression that proves the failure:
  - Compile/link failures: C probe including `<uv.h>` and referencing the missing symbol or struct.
  - Package layout failures: shell probe or extension to `verify_deb_payload_contract.py`.
  - Runtime loading failures: probe showing the staged/pkg-config library path resolves to the safe library.
- Rerun the strict original-mode override matrix into `validator/artifacts/libuv-safe/phase-15`, parse every result JSON, and reject any run where `override_debs_installed` is not true for every case.
- If no phase-owned failures exist, update `validator-report.md` with the no-op review.

## Verification Phases

- Phase ID: `check-30-packaging-validator-tester`
- Type: `check`
- `bounce_target`: `impl-15-validator-packaging-abi-fixes`
- Purpose: Rebuilds packages, verifies package payloads and staged install, runs regressions through this phase, and reruns validator with local overrides.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  "$REPO/safe/tools/verify_phase_head.sh" impl-15-validator-packaging-abi-fixes
  if [ ! -f "$REPO/original/build-checker/libuv.so.1.0.0" ]; then
    cmake -S "$REPO/original" -B "$REPO/original/build-checker" \
      -DBUILD_TESTING=OFF -DCMAKE_BUILD_TYPE=RelWithDebInfo
    cmake --build "$REPO/original/build-checker" -j"$(nproc)"
  fi
  "$REPO/safe/tools/build_deb.sh"
  python3 "$REPO/safe/tools/verify_deb_payload_contract.py" \
    "$REPO/safe/dist/artifacts.env" \
    "$REPO/original/include" \
    "$REPO/safe/debian/not-installed"
  cargo build --manifest-path "$REPO/safe/Cargo.toml" --release
  "$REPO/safe/tools/stage_install.sh" /tmp/libuv-safe-validator-stage
  "$REPO/safe/tools/verify_stage_install.sh" /tmp/libuv-safe-validator-stage
  "$REPO/safe/tools/run_regressions.sh" \
    --stage /tmp/libuv-safe-validator-stage \
    --up-to-phase impl-15-validator-packaging-abi-fixes
  ARTIFACT_ROOT="$REPO/validator/artifacts/libuv-safe/check-30"
  OVERRIDE_ROOT="$ARTIFACT_ROOT/local-debs"
  rm -rf "$ARTIFACT_ROOT"
  mkdir -p "$OVERRIDE_ROOT/libuv"
  cp "$REPO"/safe/dist/libuv1t64_*.deb "$OVERRIDE_ROOT/libuv/"
  cp "$REPO"/safe/dist/libuv1-dev_*.deb "$OVERRIDE_ROOT/libuv/"
  set +e
  bash "$REPO/validator/test.sh" \
    --config "$REPO/validator/repositories.yml" \
    --tests-root "$REPO/validator/tests" \
    --artifact-root "$ARTIFACT_ROOT" \
    --mode original \
    --override-deb-root "$OVERRIDE_ROOT" \
    --library libuv \
    --record-casts
  VALIDATOR_STATUS=$?
  set -e
  echo "validator exit status: $VALIDATOR_STATUS"
  python3 - <<'PY'
  import json
  from pathlib import Path

  repo = Path("/home/yans/safelibs/pipeline/ports/port-libuv")
  validator = repo / "validator"
  source_expected = len(list((validator / "tests/libuv/tests/cases/source").glob("*.sh")))
  usage_expected = len(list((validator / "tests/libuv/tests/cases/usage").glob("*.sh")))
  total_expected = source_expected + usage_expected
  report = (repo / "validator-report.md").read_text()
  allowed_remaining = {
      "impl-16-validator-core-loop-threading-fixes",
      "impl-17-validator-fs-dns-process-fixes",
      "impl-18-validator-network-io-fixes",
      "impl-19-validator-catchall-final-report",
  }

  def validate_results(label: str, root: Path) -> tuple[dict, list[str]]:
      summary = json.loads((root / "summary.json").read_text())
      assert summary["library"] == "libuv", summary
      assert summary["mode"] == "original", summary
      assert summary["cases"] == total_expected, summary
      assert summary["source_cases"] == source_expected, summary
      assert summary["usage_cases"] == usage_expected, summary
      result_paths = sorted(path for path in root.glob("*.json") if path.name != "summary.json")
      assert len(result_paths) == total_expected, (label, len(result_paths))
      failures = []
      missing_override = []
      for path in result_paths:
          payload = json.loads(path.read_text())
          if payload.get("override_debs_installed") is not True:
              missing_override.append(path.name)
          if payload.get("status") == "failed":
              failures.append(payload["testcase_id"])
      assert not missing_override, (label, missing_override)
      return summary, failures

  validate_results("phase-15", validator / "artifacts/libuv-safe/phase-15/results/libuv")
  summary, failures = validate_results("check-30", validator / "artifacts/libuv-safe/check-30/results/libuv")
  unowned_or_current = []
  for testcase in failures:
      skip_section = report.split("## Skipped validator bugs", 1)[1].split("\n## ", 1)[0] if "## Skipped validator bugs" in report else ""
      if testcase in skip_section:
          continue
      lines = [line for line in report.splitlines() if testcase in line]
      owners = sorted({owner for owner in allowed_remaining if any(owner in line for line in lines)})
      if len(owners) != 1:
          unowned_or_current.append((testcase, owners))
  assert not unowned_or_current, unowned_or_current
  print(f"check-30 matrix results: passed={summary['passed']} failed={summary['failed']}")
  PY
  ```

- Phase ID: `check-31-packaging-senior-review`
- Type: `check`
- `bounce_target`: `impl-15-validator-packaging-abi-fixes`
- Purpose: Reviews ABI/package compatibility, exported symbols, unsafe audit, report updates, commit topology, and validator cleanliness.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  git -C "$REPO/validator" diff --exit-code
  git -C "$REPO/validator" diff --cached --exit-code
  "$REPO/safe/tools/verify_exports.sh" \
    "$REPO/safe/target/release/libuv.so" \
    "$REPO/original/build-checker/libuv.so.1.0.0" \
    "$REPO/original/debian/libuv1t64.symbols"
  python3 "$REPO/safe/tools/audit_unsafe.py" "$REPO/safe/src"
  rg -n 'impl-15-validator-packaging-abi-fixes|packag|ABI|header|symbol|pkg-config' \
    "$REPO/validator-report.md"
  python3 "$REPO/safe/tools/verify_phase_commit_sequence.py" \
    --base-file "$REPO/.plan/workflow-run-base.txt" \
    impl-14-validator-refresh-baseline \
    impl-15-validator-packaging-abi-fixes
  ```

## Success Criteria

- Packaging, ABI, header, symbol, and local-override failures owned by `impl-15-validator-packaging-abi-fixes` are fixed or proven not to belong to this phase.
- Any real failure fixed in this phase has a minimal checked-in regression with a manifest entry owned by `impl-15-validator-packaging-abi-fixes`.
- Package payload, staged install, export comparison, unsafe audit, and regressions through phase 15 pass.
- Phase-15 and check-30 original-mode validator results contain all current libuv cases and `override_debs_installed: true` for every non-summary JSON.
- Remaining validator failures are assigned only to later phases or documented as proven validator bugs.
- No tracked validator files or generated package/build artifacts are committed.

## Git Commit Requirement

The implementer must commit work to git before yielding. The commit subject must begin with `impl-15-validator-packaging-abi-fixes`; if no source fix is needed, update `validator-report.md` and commit that no-op review or make an explicit `--allow-empty` commit with the phase ID at the start of the subject.
