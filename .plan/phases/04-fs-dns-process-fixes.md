# Filesystem, DNS, and Process Fixes

Phase Name: Filesystem, DNS, and Process Fixes

Implement Phase ID: `impl-17-validator-fs-dns-process-fixes`

## Preexisting Inputs

- `.plan/plan.md` as the immutable source plan.
- `.plan/workflow-run-base.txt` written and committed by `impl-14-validator-refresh-baseline`.
- `validator-report.md` updated by phases 1 through 3, including baseline, packaging/ABI, and core-loop/threading dispositions plus remaining owner assignments.
- Refreshed validator checkout: `validator/.git`, `validator/README.md`, `validator/Makefile`, `validator/test.sh`, `validator/repositories.yml`, and `validator/tests/libuv/**`.
- Prior validator artifacts: `validator/artifacts/libuv-safe/phase-14/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-15/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-16/results/libuv/summary.json`, `validator/artifacts/libuv-safe/check-30/results/libuv/summary.json`, `validator/artifacts/libuv-safe/check-32/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-14/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-15/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-16/results/libuv/*.json`, `validator/artifacts/libuv-safe/check-30/results/libuv/*.json`, `validator/artifacts/libuv-safe/check-32/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-14/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/phase-15/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/phase-16/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/check-30/local-debs/libuv/*.deb`, and `validator/artifacts/libuv-safe/check-32/local-debs/libuv/*.deb`.
- Historical comparison artifacts: `validator/artifacts/libuv-safe/phase-08/**`, `validator/artifacts/libuv-safe/phase-09/**`, `validator/artifacts/libuv-safe/phase-10/**`, `validator/artifacts/libuv-safe/phase-11/**`, `validator/artifacts/libuv-safe/phase-12/**`, `validator/artifacts/libuv-safe/check-25-final/**`, and `validator/site/libuv-safe-final/**`.
- Existing filesystem and resolver regressions: `safe/tests/regressions/fs_readlink_proc_self.c`, `safe/tests/regressions/getaddrinfo_long_hostname.c`, `safe/tests/regressions/validator_fs_enoent_error_names.c`, plus `safe/tests/regressions/manifest.json`.
- Error mapping input: `safe/src/core/error.rs`.
- Filesystem inputs: `safe/src/unix/fs.rs`, including `fs_req_init`, `req_cleanup`, and `access`; `safe/src/unix/fs_event.rs`; and `safe/src/unix/fs_poll.rs`.
- DNS/resolver inputs: `safe/src/unix/getaddrinfo.rs`, `safe/src/unix/getnameinfo.rs`, `safe/src/util/idna.rs`, and `safe/src/upstream_support/inet.rs`.
- Process/signal/pipe/fd inputs: `safe/src/unix/process.rs`, `safe/src/unix/pipe.rs`, `safe/src/unix/signal.rs`, and `safe/src/unix/fd.rs`.
- Async dispatch input: `safe/src/threading/threadpool.rs`.
- Export and ABI inputs: `safe/src/exports/generated.rs`, `safe/src/exports/mod.rs`, and `safe/src/abi/linux_x86_64.rs`.
- Rust crate inputs: `safe/Cargo.toml`, `safe/Cargo.lock`, `safe/rust-toolchain.toml`, `safe/vendor/libc-0.2.184/**`, and `safe/src/lib.rs`.
- Build, package, stage, audit, and regression tools: `safe/tools/build_deb.sh`, `safe/tools/stage_install.sh`, `safe/tools/verify_stage_install.sh`, `safe/tools/audit_unsafe.py`, `safe/tools/run_regressions.sh`, `safe/tools/verify_phase_head.sh`, and `safe/tools/verify_phase_commit_sequence.py`.
- Latest package build metadata if present from prior phases: `safe/dist/artifacts.env`, `safe/dist/libuv1t64_*.deb`, and `safe/dist/libuv1-dev_*.deb`.
- Prepared inputs to consume in place: `all_cves.json`, `relevant_cves.json`, `dependents.json`, `safe/tests/upstream/**`, `safe/tests/harness/**`, `safe/tests/dependents/**`, `safe/docker/**`, and `safe/debian/**`.

## New Outputs

- New regressions for filesystem, DNS, process, or error-name failures, registered with `phase_owner: "impl-17-validator-fs-dns-process-fixes"`.
- Updated implementation files if required.
- `validator/artifacts/libuv-safe/phase-17/**` implementation-run artifacts.
- Updated `validator-report.md`.

## File Changes

- `validator-report.md`
- Conditional: `safe/tests/regressions/manifest.json`
- Conditional: new `safe/tests/regressions/validator_<testcase>_<issue>.c` or `.sh`
- Conditional: `safe/src/core/error.rs`
- Conditional: `safe/src/unix/fs.rs`, `safe/src/unix/fs_event.rs`, `safe/src/unix/fs_poll.rs`
- Conditional: `safe/src/unix/getaddrinfo.rs`, `safe/src/unix/getnameinfo.rs`, `safe/src/util/idna.rs`, `safe/src/upstream_support/inet.rs`
- Conditional: `safe/src/unix/process.rs`, `safe/src/unix/pipe.rs`, `safe/src/unix/signal.rs`, `safe/src/unix/fd.rs`
- Conditional: `safe/src/threading/threadpool.rs`
- Do not modify tracked files under `validator/`
- Do not commit generated `validator/artifacts/**`, `safe/dist/**`, `safe/target/**`, root-level package artifacts, or `original/build-checker/**`

## Implementation Details

- Own failures from source cases `dns-getaddrinfo.sh` and `fs-read-write.sh`, plus process failures where root cause is `uv_spawn`, stdio setup, process exit status, process close, signal restore, or reaping.
- Own Node usage failures involving `fs.*`, `fs.promises.*`, `opendir`, `watch`, `watchFile`, symlink/readlink/realpath, stat/statfs, chmod/truncate/readv/writev, `dns.lookup`, resolver promises, and `child_process.*` unless the proven root cause is pipe/network behavior assigned to phase 5.
- Preserve fs request lifecycle: synchronous calls set `req->result`, `req->ptr`, `req->path`, `req->new_path`, stat buffers, and cleanup ownership compatibly; async calls queue work, complete on the owning loop, and invoke callbacks once.
- When fixing `safe/src/unix/fs.rs`, explicitly inspect request initialization in `fs_req_init`, ownership release in `req_cleanup`, and `access` behavior so fs access/error-name regressions are not treated as unrelated rediscovery work.
- Preserve error mapping in `safe/src/core/error.rs`: positive errno values canonicalize to negative libuv codes for lookup, known values map to upstream names/messages, and unknown values format as `Unknown system error <raw-code>` for both direct and `_r` variants.
- Preserve resolver lifecycle: cloned host/service strings survive async work, `addrinfo` output is freed through `uv_freeaddrinfo`, and long IDNA names are not truncated.
- Preserve process behavior: child setup must apply stdio, cwd, environment, uid/gid flags, and signal state in the correct order; failed exec must be reported through the error pipe; exit callbacks must fire once.
- Add minimal C regressions. For fs/error mapping, follow the structure of `validator_fs_enoent_error_names.c`; for DNS, use resolver interposition only when necessary; for process, spawn `/bin/sh` or `/usr/bin/env` with the smallest stdio/exit witness.
- Rerun strict original-mode override matrix into `validator/artifacts/libuv-safe/phase-17` and update `validator-report.md`.
- If no phase-owned failures exist, update the report with a no-op review.

## Verification Phases

- Phase ID: `check-34-fs-dns-process-validator-tester`
- Type: `check`
- `bounce_target`: `impl-17-validator-fs-dns-process-fixes`
- Purpose: Runs filesystem/DNS/process regressions through this phase and reruns the validator matrix with local overrides.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  "$REPO/safe/tools/verify_phase_head.sh" impl-17-validator-fs-dns-process-fixes
  cargo build --manifest-path "$REPO/safe/Cargo.toml" --release
  "$REPO/safe/tools/stage_install.sh" /tmp/libuv-safe-validator-stage
  "$REPO/safe/tools/verify_stage_install.sh" /tmp/libuv-safe-validator-stage
  "$REPO/safe/tools/run_regressions.sh" \
    --stage /tmp/libuv-safe-validator-stage \
    --up-to-phase impl-17-validator-fs-dns-process-fixes
  "$REPO/safe/tools/build_deb.sh"
  ARTIFACT_ROOT="$REPO/validator/artifacts/libuv-safe/check-34"
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
  total_expected = (
      source_expected +
      usage_expected
  )
  report = (repo / "validator-report.md").read_text()
  allowed_remaining = {
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

  validate_results("phase-17", validator / "artifacts/libuv-safe/phase-17/results/libuv")
  summary, failures = validate_results("check-34", validator / "artifacts/libuv-safe/check-34/results/libuv")
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
  print(f"check-34 matrix results: passed={summary['passed']} failed={summary['failed']}")
  PY
  ```

- Phase ID: `check-35-fs-dns-process-senior-review`
- Type: `check`
- `bounce_target`: `impl-17-validator-fs-dns-process-fixes`
- Purpose: Reviews request lifecycle, error mapping, fs cleanup, resolver cleanup, process stdio/reaping, unsafe audit, report updates, and topology.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  python3 "$REPO/safe/tools/audit_unsafe.py" "$REPO/safe/src"
  rg -n 'impl-17-validator-fs-dns-process-fixes|fs|ENOENT|strerror|err_name|dns|getaddrinfo|process|spawn' \
    "$REPO/validator-report.md"
  git -C "$REPO/validator" diff --exit-code
  git -C "$REPO/validator" diff --cached --exit-code
  python3 "$REPO/safe/tools/verify_phase_commit_sequence.py" \
    --base-file "$REPO/.plan/workflow-run-base.txt" \
    impl-14-validator-refresh-baseline \
    impl-15-validator-packaging-abi-fixes \
    impl-16-validator-core-loop-threading-fixes \
    impl-17-validator-fs-dns-process-fixes
  ```

## Success Criteria

- Filesystem, DNS, process, and error-mapping failures owned by `impl-17-validator-fs-dns-process-fixes` are fixed or proven not to belong to this phase.
- Any real failure fixed in this phase has a minimal checked-in regression with a manifest entry owned by `impl-17-validator-fs-dns-process-fixes`.
- Build, staged install, regressions through phase 17, unsafe audit, and phase commit topology checks pass.
- Phase-17 and check-34 original-mode validator results contain all current libuv cases and `override_debs_installed: true` for every non-summary JSON.
- Remaining validator failures are assigned only to later phases or documented as proven validator bugs.
- No tracked validator files or generated package/build artifacts are committed.

## Git Commit Requirement

The implementer must commit work to git before yielding. The commit subject must begin with `impl-17-validator-fs-dns-process-fixes`; if no source fix is needed, update `validator-report.md` and commit that no-op review or make an explicit `--allow-empty` commit with the phase ID at the start of the subject.
