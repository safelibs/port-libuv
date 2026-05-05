# Catch-all Remaining Fixes, Final Port-mode Run, and Final Report

Phase Name: Catch-all Remaining Fixes, Final Port-mode Run, and Final Report

Implement Phase ID: `impl-19-validator-catchall-final-report`

## Preexisting Inputs

- `.plan/plan.md` as the immutable source plan.
- `.plan/workflow-run-base.txt` written and committed by `impl-14-validator-refresh-baseline`.
- `validator-report.md` updated by phases 1 through 5, including every prior phase disposition and all remaining failures assigned to `impl-19-validator-catchall-final-report` or documented as suspected validator bugs.
- Refreshed validator checkout: `validator/.git`, `validator/README.md`, `validator/Makefile`, `validator/test.sh`, `validator/repositories.yml`, and `validator/tests/libuv/**`.
- Prior validator artifacts: `validator/artifacts/libuv-safe/phase-14/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-15/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-16/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-17/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-18/results/libuv/summary.json`, `validator/artifacts/libuv-safe/check-30/results/libuv/summary.json`, `validator/artifacts/libuv-safe/check-32/results/libuv/summary.json`, `validator/artifacts/libuv-safe/check-34/results/libuv/summary.json`, `validator/artifacts/libuv-safe/check-36/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-14/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-15/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-16/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-17/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-18/results/libuv/*.json`, `validator/artifacts/libuv-safe/check-30/results/libuv/*.json`, `validator/artifacts/libuv-safe/check-32/results/libuv/*.json`, `validator/artifacts/libuv-safe/check-34/results/libuv/*.json`, `validator/artifacts/libuv-safe/check-36/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-14/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/phase-15/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/phase-16/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/phase-17/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/phase-18/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/check-30/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/check-32/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/check-34/local-debs/libuv/*.deb`, and `validator/artifacts/libuv-safe/check-36/local-debs/libuv/*.deb`.
- Historical comparison artifacts: `validator/artifacts/libuv-safe/phase-08/**`, `validator/artifacts/libuv-safe/phase-09/**`, `validator/artifacts/libuv-safe/phase-10/**`, `validator/artifacts/libuv-safe/phase-11/**`, `validator/artifacts/libuv-safe/phase-12/**`, `validator/artifacts/libuv-safe/check-25-final/**`, and `validator/site/libuv-safe-final/**`.
- Latest package build metadata and packages: `safe/dist/artifacts.env`, `safe/dist/libuv1t64_*.deb`, and `safe/dist/libuv1-dev_*.deb`.
- Latest regression inventory: `safe/tests/regressions/manifest.json` and all checked-in `safe/tests/regressions/*.c` and `safe/tests/regressions/*.sh`.
- Existing lock-writer regression: `safe/tests/regressions/validator_port_lock_safe_token_sanitized.sh`.
- Local lock writer: `safe/tools/write_validator_port_lock.py`.
- Final validator proof and site tools: `validator/tools/verify_proof_artifacts.py`, `validator/tools/render_site.py`, and `validator/scripts/verify-site.sh`.
- Build, package, stage, audit, and topology tools: `safe/tools/build_deb.sh`, `safe/tools/stage_install.sh`, `safe/tools/verify_stage_install.sh`, `safe/tools/verify_deb_payload_contract.py`, `safe/tools/audit_unsafe.py`, `safe/tools/run_regressions.sh`, `safe/tools/verify_phase_head.sh`, and `safe/tools/verify_phase_commit_sequence.py`.
- Rust, ABI, export, source, package, and test inputs: `safe/Cargo.toml`, `safe/Cargo.lock`, `safe/rust-toolchain.toml`, `safe/vendor/libc-0.2.184/**`, `safe/src/**`, `safe/include/**`, `safe/debian/**`, `original/include/**`, `original/debian/libuv1t64.symbols`, `original/debian/not-installed`, and `original/build-checker/libuv.so.1.0.0` if already materialized.
- Prepared inputs to consume in place: `all_cves.json`, `relevant_cves.json`, `dependents.json`, `safe/tests/upstream/**`, `safe/tests/harness/**`, `safe/tests/dependents/**`, and `safe/docker/**`.

## New Outputs

- Final strict original-mode override rerun if phase 6 needs one: `validator/artifacts/libuv-safe/phase-19/**`.
- Final clean port-mode run: `validator/artifacts/libuv-safe/check-38-final/**`.
- Final local port-deb lock: `validator/artifacts/libuv-safe/check-38-final/proof/local-port-debs-lock.json`.
- Final port-mode proof: `validator/artifacts/libuv-safe/check-38-final/proof/libuv-safe-validation-proof.json`.
- Final rendered site: `validator/site/libuv-safe-final-refresh/**`.
- Updated `validator-report.md` summarizing validator commit, checks executed, failures found, fixes applied, skipped checks if any, final proof/site paths, and final pass/fail counts.
- Conditional catch-all regressions with `phase_owner: "impl-19-validator-catchall-final-report"`.

## File Changes

- `validator-report.md`
- Conditional: `safe/tests/regressions/manifest.json`
- Conditional: new `safe/tests/regressions/validator_<testcase>_<issue>.c` or `.sh`
- Conditional: any `safe/src/**`, `safe/debian/**`, or `safe/tools/**` file needed for remaining real failures
- Conditional: `safe/tools/write_validator_port_lock.py` if proof/site validation exposes local-lock schema or token issues
- Do not modify tracked files under `validator/`
- Do not commit generated `validator/artifacts/**`, `validator/site/**`, `safe/dist/**`, `safe/target/**`, root-level package artifacts, or `original/build-checker/**`

## Implementation Details

- Reparse the latest strict original-mode override results from phase 5. Own any remaining failure not already assigned to a previous implementation phase.
- For each remaining real libuv-safe failure:
  - Add a minimal regression under `safe/tests/regressions/`.
  - Register it in `safe/tests/regressions/manifest.json` with `phase_owner: "impl-19-validator-catchall-final-report"`.
  - Fix the underlying safe implementation or packaging issue.
  - Run `safe/tools/run_regressions.sh` through phase 19.
  - Rerun the strict original-mode override validator matrix and verify `override_debs_installed: true` for every result.
- For a suspected validator bug:
  - Run or inspect the unmodified Ubuntu original-package behavior.
  - Inspect the testcase and logs.
  - Document exact testcase id, evidence, why the validator is wrong, why a safe-port change would be incorrect, and any narrow skip. Do not patch validator.
- Generate the final local port-deb lock with `safe/tools/write_validator_port_lock.py`. The lock must describe the two local debs, have `mode: "port"`, empty `unported_original_packages`, record each deb's `sha256` and size, use deterministic `generated_at`, and use a sanitized synthetic release tag that does not create standalone `safe`/`unsafe` tokens in the rendered site.
- Run final validator in `port` mode with `--port-deb-lock` and `--override-deb-root`.
- Run `verify_proof_artifacts.py --mode port`, render the site, and verify the site.
- Update `validator-report.md` with a final `No skipped check` result if all cases pass. If a validator bug is proven and no validator skip flag exists, add a `## Skipped validator bugs` section listing only the affected testcase ids, the original-package evidence, and why accepting the raw failed result is the narrowest correct disposition.

## Verification Phases

- Phase ID: `check-38-final-validator-proof-site`
- Type: `check`
- `bounce_target`: `impl-19-validator-catchall-final-report`
- Purpose: Requires a clean final port-mode validator run, or only explicitly documented validator-bug skips, plus proof generation, site render, and site verification.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  "$REPO/safe/tools/verify_phase_head.sh" impl-19-validator-catchall-final-report
  docker version >/dev/null
  if [ ! -f "$REPO/original/build-checker/libuv.so.1.0.0" ]; then
    cmake -S "$REPO/original" -B "$REPO/original/build-checker" \
      -DBUILD_TESTING=OFF -DCMAKE_BUILD_TYPE=RelWithDebInfo
    cmake --build "$REPO/original/build-checker" -j"$(nproc)"
  fi
  "$REPO/safe/tools/build_deb.sh"
  ARTIFACT_ROOT="$REPO/validator/artifacts/libuv-safe/check-38-final"
  OVERRIDE_ROOT="$ARTIFACT_ROOT/local-debs"
  PORT_LOCK="$ARTIFACT_ROOT/proof/local-port-debs-lock.json"
  SITE_ROOT="$REPO/validator/site/libuv-safe-final-refresh"
  rm -rf "$ARTIFACT_ROOT" "$SITE_ROOT"
  mkdir -p "$OVERRIDE_ROOT/libuv" "$ARTIFACT_ROOT/proof"
  cp "$REPO"/safe/dist/libuv1t64_*.deb "$OVERRIDE_ROOT/libuv/"
  cp "$REPO"/safe/dist/libuv1-dev_*.deb "$OVERRIDE_ROOT/libuv/"
  python3 "$REPO/safe/tools/write_validator_port_lock.py" \
    --artifacts-env "$REPO/safe/dist/artifacts.env" \
    --override-root "$OVERRIDE_ROOT" \
    --lock-output "$PORT_LOCK" \
    --library libuv \
    --repository safelibs/port-libuv \
    --commit "$(git -C "$REPO" rev-parse HEAD)" \
    --release-tag "local-libuv-safe-$(git -C "$REPO" rev-parse --short HEAD)"
  make -C "$REPO/validator" unit
  make -C "$REPO/validator" check-testcases
  bash "$REPO/validator/test.sh" \
    --config "$REPO/validator/repositories.yml" \
    --tests-root "$REPO/validator/tests" \
    --artifact-root "$ARTIFACT_ROOT" \
    --mode port \
    --port-deb-lock "$PORT_LOCK" \
    --override-deb-root "$OVERRIDE_ROOT" \
    --library libuv \
    --record-casts
  python3 "$REPO/validator/tools/verify_proof_artifacts.py" \
    --config "$REPO/validator/repositories.yml" \
    --tests-root "$REPO/validator/tests" \
    --artifact-root "$ARTIFACT_ROOT" \
    --proof-output proof/libuv-safe-validation-proof.json \
    --mode port \
    --library libuv \
    --require-casts \
    --min-source-cases 5 \
    --min-usage-cases 140 \
    --min-cases 145 \
    --ports-root /home/yans/safelibs/pipeline/ports
  python3 "$REPO/validator/tools/render_site.py" \
    --config "$REPO/validator/repositories.yml" \
    --tests-root "$REPO/validator/tests" \
    --artifact-root "$ARTIFACT_ROOT" \
    --proof-path "$ARTIFACT_ROOT/proof/libuv-safe-validation-proof.json" \
    --output-root "$SITE_ROOT"
  bash "$REPO/validator/scripts/verify-site.sh" \
    --config "$REPO/validator/repositories.yml" \
    --tests-root "$REPO/validator/tests" \
    --artifacts-root "$ARTIFACT_ROOT" \
    --proof-path "$ARTIFACT_ROOT/proof/libuv-safe-validation-proof.json" \
    --site-root "$SITE_ROOT" \
    --library libuv
  python3 - <<'PY'
  import json
  from pathlib import Path

  repo = Path("/home/yans/safelibs/pipeline/ports/port-libuv")
  root = repo / "validator/artifacts/libuv-safe/check-38-final/port/results/libuv"
  report = (repo / "validator-report.md").read_text()
  summary = json.loads((root / "summary.json").read_text())
  assert summary["mode"] == "port", summary
  result_paths = sorted(path for path in root.glob("*.json") if path.name != "summary.json")
  assert len(result_paths) == summary["cases"], len(result_paths)
  failed = []
  for path in result_paths:
      payload = json.loads(path.read_text())
      assert payload.get("mode") == "port", payload
      if payload.get("status") == "failed":
          failed.append(payload["testcase_id"])
  assert summary["failed"] == len(failed), summary
  assert summary["passed"] + summary["failed"] == summary["cases"], summary
  if failed:
      assert "## Skipped validator bugs" in report, failed
      skip_section = report.split("## Skipped validator bugs", 1)[1].split("\n## ", 1)[0]
      missing = [testcase for testcase in failed if testcase not in skip_section]
      assert not missing, missing
      assert "No skipped check" not in report, report
  else:
      assert "No skipped check" in report, report
  print(f"final port-mode validator passed {summary['passed']} / {summary['cases']} cases; documented skips={len(failed)}")
  PY
  ```

- Phase ID: `check-39-final-safe-crate-senior-review`
- Type: `check`
- `bounce_target`: `impl-19-validator-catchall-final-report`
- Purpose: Independently verifies local regressions, package payload, unsafe audit, and validator cleanliness after final fixes.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  cargo build --manifest-path "$REPO/safe/Cargo.toml" --release
  "$REPO/safe/tools/stage_install.sh" /tmp/libuv-safe-validator-stage
  "$REPO/safe/tools/verify_stage_install.sh" /tmp/libuv-safe-validator-stage
  "$REPO/safe/tools/run_regressions.sh" \
    --stage /tmp/libuv-safe-validator-stage \
    --up-to-phase impl-19-validator-catchall-final-report
  "$REPO/safe/tools/build_deb.sh"
  python3 "$REPO/safe/tools/verify_deb_payload_contract.py" \
    "$REPO/safe/dist/artifacts.env" \
    "$REPO/original/include" \
    "$REPO/safe/debian/not-installed"
  python3 "$REPO/safe/tools/audit_unsafe.py" "$REPO/safe/src"
  git -C "$REPO/validator" diff --exit-code
  git -C "$REPO/validator" diff --cached --exit-code
  ```

- Phase ID: `check-40-final-report-and-topology`
- Type: `check`
- `bounce_target`: `impl-19-validator-catchall-final-report`
- Purpose: Verifies the final report is complete and phase commits are linear.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  python3 - <<'PY'
  from pathlib import Path
  report = Path("/home/yans/safelibs/pipeline/ports/port-libuv/validator-report.md").read_text()
  required = [
      "Validator commit",
      "Checks executed",
      "Failures found",
      "Fixes applied",
      "Final result",
      "Remaining failures",
      "port-mode",
  ]
  missing = [text for text in required if text not in report]
  assert not missing, missing
  assert ("No skipped check" in report) ^ ("## Skipped validator bugs" in report), (
      "report must state either no skips or documented validator-bug skips"
  )
  PY
  python3 "$REPO/safe/tools/verify_phase_commit_sequence.py" \
    --base-file "$REPO/.plan/workflow-run-base.txt" \
    impl-14-validator-refresh-baseline \
    impl-15-validator-packaging-abi-fixes \
    impl-16-validator-core-loop-threading-fixes \
    impl-17-validator-fs-dns-process-fixes \
    impl-18-validator-network-io-fixes \
    impl-19-validator-catchall-final-report
  python3 - <<'PY'
  import subprocess
  from pathlib import Path

  repo = Path("/home/yans/safelibs/pipeline/ports/port-libuv")
  base = (repo / ".plan/workflow-run-base.txt").read_text().strip()
  tracked = subprocess.check_output(
      ["git", "-C", str(repo), "ls-files"],
      text=True,
  ).splitlines()
  changed = subprocess.check_output(
      ["git", "-C", str(repo), "diff", "--name-only", f"{base}..HEAD"],
      text=True,
  ).splitlines()
  prefixes = ("safe/dist/", "safe/target/", "original/build-checker/")
  root_exts = (".deb", ".ddeb", ".changes", ".buildinfo")

  def forbidden(path: str) -> bool:
      return (
          path == "validator"
          or path.startswith("validator/")
          or path.startswith(prefixes)
          or ("/" not in path and path.endswith(root_exts))
      )

  bad_tracked = [path for path in tracked if forbidden(path)]
  bad_changed = [path for path in changed if forbidden(path)]
  assert not bad_tracked, bad_tracked
  assert not bad_changed, bad_changed
  PY
  python3 - <<'PY'
  import json
  from pathlib import Path
  repo = Path("/home/yans/safelibs/pipeline/ports/port-libuv")
  proof = repo / "validator/artifacts/libuv-safe/check-38-final/proof/libuv-safe-validation-proof.json"
  lock = repo / "validator/artifacts/libuv-safe/check-38-final/proof/local-port-debs-lock.json"
  assert proof.is_file(), proof
  assert lock.is_file(), lock
  payload = json.loads(lock.read_text())
  assert payload["mode"] == "port", payload
  assert len(payload["libraries"]) == 1, payload
  lib = payload["libraries"][0]
  assert lib["library"] == "libuv", lib
  assert lib["unported_original_packages"] == [], lib
  assert len(lib["debs"]) == 2, lib
  assert payload.get("generated_at") == "1970-01-01T00:00:00Z", payload
  for deb in lib["debs"]:
      assert deb.get("sha256"), deb
      assert deb.get("size") or deb.get("size_bytes"), deb
  import re
  assert re.search(r"\b(?:un)?safe\b", lib["release_tag"], re.I) is None, lib["release_tag"]
  assert re.search(r"\b(?:un)?safe\b", lib["tag_ref"], re.I) is None, lib["tag_ref"]
  PY
  ```

## Final Verification and Acceptance Requirements

The final state is valid only if the end-to-end command sequence encoded in `check-38-final-validator-proof-site`, `check-39-final-safe-crate-senior-review`, and `check-40-final-report-and-topology` passes from `/home/yans/safelibs/pipeline/ports/port-libuv`.

Final acceptance requires:

- The validator checkout commit used for the run is recorded in `validator-report.md`.
- Validator unit tests and testcase manifest checks pass.
- A strict original-mode override run was executed for failure detection and every per-case result had `override_debs_installed: true`.
- A final `port`-mode validator run passes every libuv case with casts recorded, except for any testcase explicitly documented in `validator-report.md` under `## Skipped validator bugs` with original-package evidence and a narrow justification.
- `verify_proof_artifacts.py --mode port`, `render_site.py`, and `verify-site.sh --library libuv` pass.
- Every real validator failure has a checked-in minimal regression in `safe/tests/regressions/` and a manifest entry with the owning phase id.
- `safe/tools/run_regressions.sh` passes through `impl-19-validator-catchall-final-report`.
- Package payload and staged install contracts pass.
- `safe/tools/audit_unsafe.py safe/src` passes.
- No tracked file under `validator/` is modified.
- No generated `validator`, `safe/dist`, `safe/target`, `original/build-checker`, or root-level package artifact is tracked or committed in the workflow range.
- `validator-report.md` states the checks executed, failures found, fixes applied, skipped checks if any, proof/site artifact paths, and final result.
- Implementation commits form a linear first-parent sequence from the workflow base, with each commit subject beginning with the phase id.

## Success Criteria

- Every remaining real libuv-safe validator failure is fixed and covered by a checked-in regression owned by `impl-19-validator-catchall-final-report`.
- Any remaining validator failure is documented under `## Skipped validator bugs` with original-package evidence, exact testcase id, and narrow justification; the preferred final state is zero skipped testcases and zero validator failures.
- The final validator proof comes from the final `port`-mode run using the local lock generated by `safe/tools/write_validator_port_lock.py`, not from an original-mode override run.
- The final local port-deb lock records `sha256`, size, deterministic `generated_at`, and sanitized `release_tag`/`tag_ref`, and the existing `validator_port_lock_safe_token_sanitized.sh` regression remains part of the checked-in regression inventory.
- `verify_proof_artifacts.py --mode port`, `render_site.py`, and `verify-site.sh --library libuv` pass.
- Local build, staged install, regressions through phase 19, package payload checks, unsafe audit, validator cleanliness, report completeness, generated-artifact guard, and phase commit topology checks pass.
- `validator-report.md` records checks executed, failures found, fixes applied, skipped checks if any, proof/site paths, and final result.

## Git Commit Requirement

The implementer must commit work to git before yielding. The commit subject must begin with `impl-19-validator-catchall-final-report`; if no source fix is needed, update `validator-report.md` and commit that no-op review or make an explicit `--allow-empty` commit with the phase ID at the start of the subject.
