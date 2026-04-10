# Process, Signal, Fork, and Security-Sensitive Runtime Behavior

Phase Name: Process, Signal, Fork, and Security-Sensitive Runtime Behavior

Implement Phase ID: `impl-04-process-signal-security`

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
- `/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/audit_unsafe.py`
- `/home/yans/safelibs/port-libuv/safe/tests/harness/**`
- `/home/yans/safelibs/port-libuv/safe/tests/regressions/**`
- `/home/yans/safelibs/port-libuv/all_cves.json`
- `/home/yans/safelibs/port-libuv/original/src/unix/process.c`
- `/home/yans/safelibs/port-libuv/original/src/unix/signal.c`
- `/home/yans/safelibs/port-libuv/original/src/unix/linux.c`
- `/home/yans/safelibs/port-libuv/original/test/**`

## New Outputs
- updated `/home/yans/safelibs/port-libuv/safe/src/unix/process.rs`
- updated `/home/yans/safelibs/port-libuv/safe/src/unix/signal.rs`
- updated `/home/yans/safelibs/port-libuv/safe/src/unix/fork.rs`
- updated `/home/yans/safelibs/port-libuv/safe/src/unix/process_title.rs`
- updated `/home/yans/safelibs/port-libuv/safe/src/unix/epoll.rs`
- new or updated regressions under `/home/yans/safelibs/port-libuv/safe/tests/regressions/**` for any process or security bug found here
- updated `/home/yans/safelibs/port-libuv/safe/tests/regressions/manifest.json` whenever regressions change

## File Changes
- Fix spawn stdio handling, env propagation, kill semantics, signal watcher behavior, fork recovery, process-title behavior, and security-sensitive credential-drop ordering.
- Preserve runtime io_uring disablement while leaving compatibility symbols intact.

## Implementation Details
- Consume `all_cves.json` directly as the machine-readable CVE evidence input for this phase.
- Keep process handle state on the existing `uv_process_t` layout and linked queues; do not add a parallel Rust-only process registry.
- Fix the stdio setup, inherited stream handling, child process fd preparation, and post-fork cleanup in `safe/src/unix/process.rs`.
- Preserve and test the `apply_credential_drop` ordering: `setgroups` before `setgid` before `setuid`.
- Ensure signal reset and fork recovery stay compatible with upstream libuv semantics on Linux.
- Keep `uv__kernel_version()` and `uv__use_io_uring()` hard-disabled as the security gate for this port.
- Consume `all_cves.json` as machine-readable evidence in this phase and tie any discovered fix either to a checked-in regression under `safe/tests/regressions/**` or to the explicit source reviews of `process.rs` and `epoll.rs`.
- If this phase exposes CVE-related gaps from `all_cves.json`, create checked-in regressions immediately rather than deferring them to the final sweep, and register them in `safe/tests/regressions/manifest.json` with `phase_owner: impl-04-process-signal-security` so `check-08` executes them.

## Verification Phases
### `check-07-process-signal-security-suite`
- Phase ID: `check-07-process-signal-security-suite`
- Type: `check`
- Fixed `bounce_target`: `impl-04-process-signal-security`
- Purpose: run the focused harness against process, signal, fork, process-title, kill, and dlerror behavior.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-04-process-signal-security
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-proc
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-proc --shared --tests process_title,process_title_threadsafe,spawn_stdout,spawn_stdin,spawn_exit_code,spawn_and_kill,spawn_and_kill_with_std,spawn_and_ping,spawn_detached,spawn_auto_unref,spawn_empty_env,spawn_preserve_env,spawn_setuid_setgid,spawn_setuid_fails,spawn_setgid_fails,kill,kill_invalid_signum,signal_multiple_loops,signal_pending_on_close,fork_timer,fork_socketpair,fork_socketpair_started,fork_signal_to_child,fork_signal_to_child_closed,fork_close_signal_in_child,fork_threadpool_queue_work_simple,dlerror
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-proc --static --tests process_title,spawn_stdout,kill,signal_multiple_loops,fork_timer
```

### `check-08-privilege-drop-and-io-uring-review`
- Phase ID: `check-08-privilege-drop-and-io-uring-review`
- Type: `check`
- Fixed `bounce_target`: `impl-04-process-signal-security`
- Purpose: run the root-gated privilege-drop tests through the explicit no-Python focused-harness fallback path, execute the regression manifest, and review the credential-drop and io_uring security gates.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-04-process-signal-security
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/run_regressions.sh --stage /tmp/libuv-safe-stage --up-to-phase impl-04-process-signal-security
docker run --rm -i -e UV_RUN_AS_ROOT=1 --mount type=bind,src=/home/yans/safelibs/port-libuv,target=/work --mount type=bind,src=/tmp/libuv-safe-stage,target=/tmp/libuv-safe-stage,readonly ubuntu:24.04 bash -lc 'apt-get update && apt-get install -y --no-install-recommends build-essential cmake pkg-config && LIBUV_SAFE_NO_PYTHON=1 /work/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-root-tests && UV_RUN_AS_ROOT=1 /work/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-root-tests --shared --tests spawn_setuid_setgid,spawn_setuid_fails,spawn_setgid_fails'
python3 - <<'PY'
import json
from pathlib import Path
data = json.loads(Path('/home/yans/safelibs/port-libuv/all_cves.json').read_text())
required = {'CVE-2015-0278', 'CVE-2021-22918', 'CVE-2024-22017', 'CVE-2024-24806'}
assert required.issubset(set(data['confirmed_cve_ids'])), data['confirmed_cve_ids']
PY
python3 -c 'from pathlib import Path; text = Path("/home/yans/safelibs/port-libuv/safe/src/unix/process.rs").read_text(); anchor = text.index("fn apply_credential_drop"); window = text[anchor:anchor + 2000]; sg = window.find("setgroups"); gid = window.find("setgid"); uid = window.find("setuid"); assert -1 not in (sg, gid, uid) and sg < gid < uid, (sg, gid, uid)'
python3 - <<'PY'
import re
from pathlib import Path

text = Path('/home/yans/safelibs/port-libuv/safe/src/unix/epoll.rs').read_text()

def function_body(signature: str) -> str:
    start = text.index(signature)
    brace = text.index('{', start)
    depth = 0
    for i in range(brace, len(text)):
        if text[i] == '{':
            depth += 1
        elif text[i] == '}':
            depth -= 1
            if depth == 0:
                return text[brace + 1:i]
    raise AssertionError(signature)

def normalize(body: str) -> str:
    body = re.sub(r'//.*', '', body)
    body = re.sub(r'/\\*.*?\\*/', '', body, flags=re.S)
    body = re.sub(r'\\s+', '', body)
    return body

kernel = normalize(function_body('pub extern "C" fn uv__kernel_version()'))
use = normalize(function_body('pub extern "C" fn uv__use_io_uring()'))
allowed_kernel = {
    'unsafe{0}',
    'unsafe{return0;}',
    'unsafe{0as::core::ffi::c_uint}',
    'unsafe{return0as::core::ffi::c_uint;}',
}
allowed_use = {
    'unsafe{0}',
    'unsafe{return0;}',
    'unsafe{0as::core::ffi::c_int}',
    'unsafe{return0as::core::ffi::c_int;}',
}
assert kernel in allowed_kernel, kernel
assert use in allowed_use, use
PY
python3 /home/yans/safelibs/port-libuv/safe/tools/audit_unsafe.py /home/yans/safelibs/port-libuv/safe/src
```

- Review checks:
  - Confirm the workflow treats the io_uring contract as symbols may remain for compatibility and runtime enablement must stay disabled, not as delete all io_uring identifiers from the tree.
  - Confirm `all_cves.json` is the CVE evidence input used for this phase.
  - Confirm the root-gated harness command sets `LIBUV_SAFE_NO_PYTHON=1` explicitly instead of relying on `ubuntu:24.04` to omit `python3`.
  - Confirm any new file added under `safe/tests/regressions/**` is registered in `safe/tests/regressions/manifest.json`.

## Success Criteria
- `check-07-process-signal-security-suite` passes the focused process, signal, fork, process-title, kill, and dlerror subset.
- `check-08-privilege-drop-and-io-uring-review` passes the regression manifest, the root-gated privilege-drop tests, the CVE evidence check, and the source audits.
- The credential-drop ordering remains `setgroups` then `setgid` then `setuid`, and io_uring runtime enablement stays explicitly disabled.
- The latest commit produced by this phase begins with `impl-04-process-signal-security`.

## Git Commit Requirement
The implementer must commit the phase work to git before yielding. The newest commit at `HEAD` must remain the handoff commit for this phase, and its subject must begin with `impl-04-process-signal-security`.
