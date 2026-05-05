# Core Loop, Timer, Async, Threading, and Random Fixes

Phase Name: Core Loop, Timer, Async, Threading, and Random Fixes

Implement Phase ID: `impl-16-validator-core-loop-threading-fixes`

## Preexisting Inputs

- `.plan/plan.md` as the immutable source plan.
- `.plan/workflow-run-base.txt` written and committed by `impl-14-validator-refresh-baseline`.
- `validator-report.md` updated by phases 1 and 2, including phase 2 packaging/ABI disposition and remaining owner assignments.
- Refreshed validator checkout: `validator/.git`, `validator/README.md`, `validator/Makefile`, `validator/test.sh`, `validator/repositories.yml`, and `validator/tests/libuv/**`.
- Prior validator artifacts: `validator/artifacts/libuv-safe/phase-14/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-15/results/libuv/summary.json`, `validator/artifacts/libuv-safe/check-30/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-14/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-15/results/libuv/*.json`, `validator/artifacts/libuv-safe/check-30/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-14/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/phase-15/local-debs/libuv/*.deb`, and `validator/artifacts/libuv-safe/check-30/local-debs/libuv/*.deb`.
- Historical comparison artifacts: `validator/artifacts/libuv-safe/phase-08/**`, `validator/artifacts/libuv-safe/phase-09/**`, `validator/artifacts/libuv-safe/phase-10/**`, `validator/artifacts/libuv-safe/phase-11/**`, `validator/artifacts/libuv-safe/phase-12/**`, `validator/artifacts/libuv-safe/check-25-final/**`, and `validator/site/libuv-safe-final/**`.
- Rust crate inputs: `safe/Cargo.toml`, `safe/Cargo.lock`, `safe/rust-toolchain.toml`, `safe/vendor/libc-0.2.184/**`, and `safe/src/lib.rs`.
- Existing regressions for timer/threadpool/random surfaces: `safe/tests/regressions/validator_timer_ordering_close.c`, `safe/tests/regressions/validator_threadpool_queue_work.c`, `safe/tests/regressions/validator_random_sync_async.c`, plus `safe/tests/regressions/manifest.json`.
- Core loop inputs: `safe/src/core/loop.rs`, `safe/src/core/default_loop.rs`, `safe/src/core/handle.rs`, `safe/src/core/request.rs`, `safe/src/core/timer.rs`, `safe/src/core/time.rs`, and `safe/src/core/metrics.rs`.
- Backend/threading inputs: `safe/src/unix/epoll.rs`, `safe/src/unix/async.rs`, `safe/src/threading/threadpool.rs`, `safe/src/threading/thread.rs`, `safe/src/threading/sync.rs`, and `safe/src/threading/random.rs`.
- Export and ABI inputs: `safe/src/exports/generated.rs`, `safe/src/exports/mod.rs`, and `safe/src/abi/linux_x86_64.rs`.
- Build, package, stage, audit, and regression tools: `safe/tools/build_deb.sh`, `safe/tools/stage_install.sh`, `safe/tools/verify_stage_install.sh`, `safe/tools/audit_unsafe.py`, `safe/tools/run_regressions.sh`, `safe/tools/verify_phase_head.sh`, and `safe/tools/verify_phase_commit_sequence.py`.
- Latest package build metadata if present from prior phases: `safe/dist/artifacts.env`, `safe/dist/libuv1t64_*.deb`, and `safe/dist/libuv1-dev_*.deb`.
- Prepared inputs to consume in place: `all_cves.json`, `relevant_cves.json`, `dependents.json`, `safe/tests/upstream/**`, `safe/tests/harness/**`, `safe/tests/dependents/**`, `safe/docker/**`, and `safe/debian/**`.

## New Outputs

- New regressions for validator timer/event-loop/async/threadpool/random failures, registered with `phase_owner: "impl-16-validator-core-loop-threading-fixes"`.
- Updated core/threading/random implementation files if required.
- `validator/artifacts/libuv-safe/phase-16/**` implementation-run artifacts.
- Updated `validator-report.md`.

## File Changes

- `validator-report.md`
- Conditional: `safe/tests/regressions/manifest.json`
- Conditional: new `safe/tests/regressions/validator_<testcase>_<issue>.c` or `.sh`
- Conditional: `safe/src/core/loop.rs`, `safe/src/core/default_loop.rs`, `safe/src/core/handle.rs`, `safe/src/core/request.rs`, `safe/src/core/timer.rs`, `safe/src/core/time.rs`, `safe/src/core/metrics.rs`
- Conditional: `safe/src/unix/epoll.rs`, `safe/src/unix/async.rs`
- Conditional: `safe/src/threading/threadpool.rs`, `safe/src/threading/thread.rs`, `safe/src/threading/sync.rs`, `safe/src/threading/random.rs`
- Conditional: `safe/src/exports/generated.rs`
- Do not modify tracked files under `validator/`
- Do not commit generated `validator/artifacts/**`, `safe/dist/**`, `safe/target/**`, root-level package artifacts, or `original/build-checker/**`

## Implementation Details

- Own failures from `event-loop-timer.sh` and usage cases involving `setTimeout`, `setInterval`, `setImmediate`, timer promises, microtask/timer ordering, worker scheduling, `uv_async_*`, `uv_queue_work`, `uv_cancel`, `uv_random`, `crypto.randomBytes`, `randomFill`, `randomUUID`, `scrypt`, or zlib/threadpool completion when root cause is libuv loop/threadpool behavior.
- Preserve `uv_run` semantics: `UV_RUN_DEFAULT` drains active referenced handles/requests, `UV_RUN_ONCE` blocks at most one poll cycle when work is pending, and `UV_RUN_NOWAIT` does not block.
- Preserve handle invariants: active referenced handles increment loop active counts exactly once, stop/unref decrements exactly once, and close callbacks are invoked once after the handle is marked closing.
- Preserve timer invariants: due timers run in order, repeat timers reschedule after callback, `uv_timer_stop` removes pending timers, `uv_timer_again` requires a repeat value, and timer close removes heap entries before the close callback.
- Preserve threadpool invariants: work callbacks execute off-loop, after-work callbacks return to the owning loop exactly once, cancellation reports libuv-compatible status, and resolver/random requests clean up owned memory.
- Add focused C regressions using existing patterns in `safe/tests/regressions/validator_timer_ordering_close.c`, `validator_threadpool_queue_work.c`, and `validator_random_sync_async.c`.
- Rerun strict original-mode override matrix into `validator/artifacts/libuv-safe/phase-16` and update `validator-report.md`.
- If no phase-owned failures exist, update the report with a no-op review.

## Verification Phases

- Phase ID: `check-32-core-loop-validator-tester`
- Type: `check`
- `bounce_target`: `impl-16-validator-core-loop-threading-fixes`
- Purpose: Runs local build/stage/regressions through this phase and reruns the validator matrix with local overrides.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  "$REPO/safe/tools/verify_phase_head.sh" impl-16-validator-core-loop-threading-fixes
  cargo build --manifest-path "$REPO/safe/Cargo.toml" --release
  "$REPO/safe/tools/stage_install.sh" /tmp/libuv-safe-validator-stage
  "$REPO/safe/tools/verify_stage_install.sh" /tmp/libuv-safe-validator-stage
  "$REPO/safe/tools/run_regressions.sh" \
    --stage /tmp/libuv-safe-validator-stage \
    --up-to-phase impl-16-validator-core-loop-threading-fixes
  "$REPO/safe/tools/build_deb.sh"
  ARTIFACT_ROOT="$REPO/validator/artifacts/libuv-safe/check-32"
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

  validate_results("phase-16", validator / "artifacts/libuv-safe/phase-16/results/libuv")
  summary, failures = validate_results("check-32", validator / "artifacts/libuv-safe/check-32/results/libuv")
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
  print(f"check-32 matrix results: passed={summary['passed']} failed={summary['failed']}")
  PY
  ```

- Phase ID: `check-33-core-loop-senior-review`
- Type: `check`
- `bounce_target`: `impl-16-validator-core-loop-threading-fixes`
- Purpose: Reviews loop liveness, close ordering, timer heap behavior, async/threadpool completion, random API behavior, unsafe audit, and commit topology.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  python3 "$REPO/safe/tools/audit_unsafe.py" "$REPO/safe/src"
  rg -n 'impl-16-validator-core-loop-threading-fixes|timer|loop|async|thread|random|queue_work' \
    "$REPO/validator-report.md"
  git -C "$REPO/validator" diff --exit-code
  git -C "$REPO/validator" diff --cached --exit-code
  python3 "$REPO/safe/tools/verify_phase_commit_sequence.py" \
    --base-file "$REPO/.plan/workflow-run-base.txt" \
    impl-14-validator-refresh-baseline \
    impl-15-validator-packaging-abi-fixes \
    impl-16-validator-core-loop-threading-fixes
  ```

## Success Criteria

- Core loop, timer, async, threadpool, and random failures owned by `impl-16-validator-core-loop-threading-fixes` are fixed or proven not to belong to this phase.
- Any real failure fixed in this phase has a minimal checked-in regression with a manifest entry owned by `impl-16-validator-core-loop-threading-fixes`.
- Build, staged install, regressions through phase 16, unsafe audit, and phase commit topology checks pass.
- Phase-16 and check-32 original-mode validator results contain all current libuv cases and `override_debs_installed: true` for every non-summary JSON.
- Remaining validator failures are assigned only to later phases or documented as proven validator bugs.
- No tracked validator files or generated package/build artifacts are committed.

## Git Commit Requirement

The implementer must commit work to git before yielding. The commit subject must begin with `impl-16-validator-core-loop-threading-fixes`; if no source fix is needed, update `validator-report.md` and commit that no-op review or make an explicit `--allow-empty` commit with the phase ID at the start of the subject.
