# Network, Pipe, Stream, Poll, TCP, and UDP Fixes

Phase Name: Network, Pipe, Stream, Poll, TCP, and UDP Fixes

Implement Phase ID: `impl-18-validator-network-io-fixes`

## Preexisting Inputs

- `.plan/plan.md` as the immutable source plan.
- `.plan/workflow-run-base.txt` written and committed by `impl-14-validator-refresh-baseline`.
- `validator-report.md` updated by phases 1 through 4, including baseline, packaging/ABI, core-loop/threading, and filesystem/DNS/process dispositions plus remaining owner assignments.
- Refreshed validator checkout: `validator/.git`, `validator/README.md`, `validator/Makefile`, `validator/test.sh`, `validator/repositories.yml`, and `validator/tests/libuv/**`.
- Prior validator artifacts: `validator/artifacts/libuv-safe/phase-14/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-15/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-16/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-17/results/libuv/summary.json`, `validator/artifacts/libuv-safe/check-30/results/libuv/summary.json`, `validator/artifacts/libuv-safe/check-32/results/libuv/summary.json`, `validator/artifacts/libuv-safe/check-34/results/libuv/summary.json`, `validator/artifacts/libuv-safe/phase-14/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-15/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-16/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-17/results/libuv/*.json`, `validator/artifacts/libuv-safe/check-30/results/libuv/*.json`, `validator/artifacts/libuv-safe/check-32/results/libuv/*.json`, `validator/artifacts/libuv-safe/check-34/results/libuv/*.json`, `validator/artifacts/libuv-safe/phase-14/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/phase-15/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/phase-16/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/phase-17/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/check-30/local-debs/libuv/*.deb`, `validator/artifacts/libuv-safe/check-32/local-debs/libuv/*.deb`, and `validator/artifacts/libuv-safe/check-34/local-debs/libuv/*.deb`.
- Historical comparison artifacts: `validator/artifacts/libuv-safe/phase-08/**`, `validator/artifacts/libuv-safe/phase-09/**`, `validator/artifacts/libuv-safe/phase-10/**`, `validator/artifacts/libuv-safe/phase-11/**`, `validator/artifacts/libuv-safe/phase-12/**`, `validator/artifacts/libuv-safe/check-25-final/**`, and `validator/site/libuv-safe-final/**`.
- Existing network regressions: `safe/tests/regressions/validator_tcp_loopback_echo.c`, `safe/tests/regressions/validator_pipe_socketpair_stream.c`, `safe/tests/regressions/validator_udp_loopback_send_recv.c`, plus `safe/tests/regressions/manifest.json`.
- Network and stream inputs: `safe/src/unix/stream.rs`, `safe/src/unix/tcp.rs`, `safe/src/unix/udp.rs`, `safe/src/unix/pipe.rs`, `safe/src/unix/poll.rs`, `safe/src/unix/fd.rs`, and `safe/src/unix/epoll.rs`.
- Upstream support inputs: `safe/src/upstream_support/unix_core.rs` and `safe/src/upstream_support/unix_loop.rs`.
- Export and ABI inputs: `safe/src/exports/generated.rs`, `safe/src/exports/mod.rs`, and `safe/src/abi/linux_x86_64.rs`.
- Rust crate inputs: `safe/Cargo.toml`, `safe/Cargo.lock`, `safe/rust-toolchain.toml`, `safe/vendor/libc-0.2.184/**`, and `safe/src/lib.rs`.
- Build, package, stage, audit, and regression tools: `safe/tools/build_deb.sh`, `safe/tools/stage_install.sh`, `safe/tools/verify_stage_install.sh`, `safe/tools/audit_unsafe.py`, `safe/tools/run_regressions.sh`, `safe/tools/verify_phase_head.sh`, and `safe/tools/verify_phase_commit_sequence.py`.
- Latest package build metadata if present from prior phases: `safe/dist/artifacts.env`, `safe/dist/libuv1t64_*.deb`, and `safe/dist/libuv1-dev_*.deb`.
- Prepared inputs to consume in place: `all_cves.json`, `relevant_cves.json`, `dependents.json`, `safe/tests/upstream/**`, `safe/tests/harness/**`, `safe/tests/dependents/**`, `safe/docker/**`, and `safe/debian/**`.

## New Outputs

- New regressions for network/pipe/stream/poll/TCP/UDP failures, registered with `phase_owner: "impl-18-validator-network-io-fixes"`.
- Updated network/stream implementation files if required.
- `validator/artifacts/libuv-safe/phase-18/**` implementation-run artifacts.
- Updated `validator-report.md`.

## File Changes

- `validator-report.md`
- Conditional: `safe/tests/regressions/manifest.json`
- Conditional: new `safe/tests/regressions/validator_<testcase>_<issue>.c` or `.sh`
- Conditional: `safe/src/unix/stream.rs`, `safe/src/unix/tcp.rs`, `safe/src/unix/udp.rs`, `safe/src/unix/pipe.rs`, `safe/src/unix/poll.rs`, `safe/src/unix/fd.rs`, `safe/src/unix/epoll.rs`
- Conditional: `safe/src/upstream_support/unix_core.rs`, `safe/src/upstream_support/unix_loop.rs`
- Conditional: `safe/src/exports/generated.rs`
- Do not modify tracked files under `validator/`
- Do not commit generated `validator/artifacts/**`, `safe/dist/**`, `safe/target/**`, root-level package artifacts, or `original/build-checker/**`

## Implementation Details

- Own source failures from `tcp-loopback-smoke.sh` and `process-pipe-smoke.sh` when root cause is pipe/stream behavior rather than process spawning.
- Own usage failures involving Node `net.*`, `dgram.*`, HTTP/HTTP2 loopback, streams/pipeline, TCP client/server behavior, UDP bind/connect/send/recv, multicast/TTL options, EOF/read readiness, write completion, shutdown, and close ordering.
- Preserve stream invariants: `uv_listen` registers callbacks, `uv_accept` transfers accepted sockets, `uv_read_start` reports payload/EOF/errors with libuv-compatible `nread`, `uv_write` and `uv_try_write` update queue size and complete exactly once, and `uv_shutdown` completes once.
- Preserve TCP invariants: nonblocking connect completion, `getsockname` and `getpeername` after bind/connect/accept, keepalive/nodelay options, and close/reset behavior.
- Preserve UDP invariants: buffer lifetime until send callback, connected and unconnected sends, recv callback address population, send queue counters, multicast option compatibility, and libuv-compatible errors for unsupported kernel states.
- Preserve close ordering with pending I/O and avoid double callbacks or use-after-free around queued stream/udp requests.
- Add minimal loopback-only C regressions that do not require external network access.
- Rerun strict original-mode override matrix into `validator/artifacts/libuv-safe/phase-18` and update `validator-report.md`.
- If no phase-owned failures exist, update the report with a no-op review.

## Verification Phases

- Phase ID: `check-36-network-io-validator-tester`
- Type: `check`
- `bounce_target`: `impl-18-validator-network-io-fixes`
- Purpose: Runs network/stream regressions through this phase and reruns the validator matrix with local overrides.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  "$REPO/safe/tools/verify_phase_head.sh" impl-18-validator-network-io-fixes
  cargo build --manifest-path "$REPO/safe/Cargo.toml" --release
  "$REPO/safe/tools/stage_install.sh" /tmp/libuv-safe-validator-stage
  "$REPO/safe/tools/verify_stage_install.sh" /tmp/libuv-safe-validator-stage
  "$REPO/safe/tools/run_regressions.sh" \
    --stage /tmp/libuv-safe-validator-stage \
    --up-to-phase impl-18-validator-network-io-fixes
  "$REPO/safe/tools/build_deb.sh"
  ARTIFACT_ROOT="$REPO/validator/artifacts/libuv-safe/check-36"
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

  validate_results("phase-18", validator / "artifacts/libuv-safe/phase-18/results/libuv")
  summary, failures = validate_results("check-36", validator / "artifacts/libuv-safe/check-36/results/libuv")
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
  print(f"check-36 matrix results: passed={summary['passed']} failed={summary['failed']}")
  PY
  ```

- Phase ID: `check-37-network-io-senior-review`
- Type: `check`
- `bounce_target`: `impl-18-validator-network-io-fixes`
- Purpose: Reviews stream/TCP/UDP/pipe/poll invariants, close ordering, unsafe audit, report updates, topology, and validator cleanliness.
- Commands:

  ```bash
  set -euo pipefail
  REPO=/home/yans/safelibs/pipeline/ports/port-libuv
  python3 "$REPO/safe/tools/audit_unsafe.py" "$REPO/safe/src"
  rg -n 'impl-18-validator-network-io-fixes|tcp|udp|pipe|stream|poll|net|dgram|http' \
    "$REPO/validator-report.md"
  git -C "$REPO/validator" diff --exit-code
  git -C "$REPO/validator" diff --cached --exit-code
  python3 "$REPO/safe/tools/verify_phase_commit_sequence.py" \
    --base-file "$REPO/.plan/workflow-run-base.txt" \
    impl-14-validator-refresh-baseline \
    impl-15-validator-packaging-abi-fixes \
    impl-16-validator-core-loop-threading-fixes \
    impl-17-validator-fs-dns-process-fixes \
    impl-18-validator-network-io-fixes
  ```

## Success Criteria

- Network, pipe, stream, poll, TCP, and UDP failures owned by `impl-18-validator-network-io-fixes` are fixed or proven not to belong to this phase.
- Any real failure fixed in this phase has a minimal checked-in regression with a manifest entry owned by `impl-18-validator-network-io-fixes`.
- Build, staged install, regressions through phase 18, unsafe audit, and phase commit topology checks pass.
- Phase-18 and check-36 original-mode validator results contain all current libuv cases and `override_debs_installed: true` for every non-summary JSON.
- Remaining validator failures are assigned only to phase 6 or documented as proven validator bugs.
- No tracked validator files or generated package/build artifacts are committed.

## Git Commit Requirement

The implementer must commit work to git before yielding. The commit subject must begin with `impl-18-validator-network-io-fixes`; if no source fix is needed, update `validator-report.md` and commit that no-op review or make an explicit `--allow-empty` commit with the phase ID at the start of the subject.
