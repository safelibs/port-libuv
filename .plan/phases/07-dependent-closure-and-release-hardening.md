# Dependent Compatibility Closure, Safety Audit Closure, and Release Hardening

Phase Name: Dependent Compatibility Closure, Safety Audit Closure, and Release Hardening

Implement Phase ID: `impl-07-dependent-closure-and-release-hardening`

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
- `/home/yans/safelibs/port-libuv/safe/tools/verify_exports.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_sizes.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/abi-baseline.json`
- `/home/yans/safelibs/port-libuv/safe/tools/cc-linker.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/build_deb.sh`
- `/home/yans/safelibs/port-libuv/safe/dist/artifacts.env`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_deb_payload_contract.py`
- `/home/yans/safelibs/port-libuv/safe/tools/build_dependents_image.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/run_dependents_suite.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_dependents_image_contract.sh`
- `/home/yans/safelibs/port-libuv/safe/docker/Dockerfile.dependents`
- `/home/yans/safelibs/port-libuv/safe/docker/run-dependent-probes.sh`
- `/home/yans/safelibs/port-libuv/safe/tests/dependents/manifest.json`
- `/home/yans/safelibs/port-libuv/safe/tests/dependents/common.sh`
- `/home/yans/safelibs/port-libuv/safe/tests/harness/**`
- `/home/yans/safelibs/port-libuv/safe/tests/upstream/**`
- `/home/yans/safelibs/port-libuv/safe/tests/dependents/**`
- `/home/yans/safelibs/port-libuv/safe/tests/regressions/**`
- `/home/yans/safelibs/port-libuv/safe/debian/**`
- `/home/yans/safelibs/port-libuv/all_cves.json`
- `/home/yans/safelibs/port-libuv/safe/tools/audit_unsafe.py`
- `/home/yans/safelibs/port-libuv/original/include/**`
- `/home/yans/safelibs/port-libuv/original/debian/libuv1t64.symbols`
- `/home/yans/safelibs/port-libuv/original/build-checker/**`
- `/home/yans/safelibs/port-libuv/original/build-checker-review/**`
- `/home/yans/safelibs/port-libuv/original/build-checker-verify/**`
- `/home/yans/safelibs/port-libuv/original/build-checker-audit/**`
- `/home/yans/safelibs/port-libuv/original/test/**`
- `/home/yans/safelibs/port-libuv/test-original.sh`
- findings from `check-11-package-payload-and-image-contract`
- findings from `check-12-dependent-image-smoke`

## New Outputs
- final compatibility fixes across `/home/yans/safelibs/port-libuv/safe/src/**`
- final regressions and dependent probe updates under `/home/yans/safelibs/port-libuv/safe/tests/regressions/**` and `/home/yans/safelibs/port-libuv/safe/tests/dependents/**`
- updated `/home/yans/safelibs/port-libuv/safe/tests/regressions/manifest.json` whenever regressions change
- updated `/home/yans/safelibs/port-libuv/safe/tools/audit_unsafe.py` if the existing checker needs tighter coverage
- updated `/home/yans/safelibs/port-libuv/safe/COMPATIBILITY_NOTES.md`
- `/home/yans/safelibs/port-libuv/safe/RELEASE_CHECKLIST.md`

## File Changes
- Fix the remaining dependent-specific runtime, packaging, or ABI issues.
- Close out the safety or security audit with checked-in reproductions and release notes.
- Record the final release matrix and known intentional choices.

## Implementation Details
- Use the failures from the dependent smoke image as the defect queue.
- For every fixed dependent issue, add or update a checked-in probe or regression so the issue cannot regress silently; updates under `safe/tests/regressions/**` must also update `safe/tests/regressions/manifest.json` with `phase_owner: impl-07-dependent-closure-and-release-hardening`, and updates under `safe/tests/dependents/**` must also update `safe/tests/dependents/manifest.json`.
- Keep `safe/tests/regressions/**` limited to manifest-backed `c`, `cpp`, and `shell` reproducers. If a dependent-only fix needs Python, R, Lua, or OCaml machinery, capture it under `safe/tests/dependents/**` instead.
- Keep phase 7 responsible for both wrapper modes: the refactored `test-original.sh` must still validate original-mode against an upstream libuv build and safe-package mode against the packaged Rust build, using the same checked-in probe assets in both cases.
- Treat `check-14` as a second independent dependent-matrix witness, not a metadata-only audit: it must rebuild its own checker-specific image and rerun both `run_dependents_suite.sh --mode full` and the two `test-original.sh` wrapper modes from current `HEAD`.
- Update `safe/COMPATIBILITY_NOTES.md` with the final externally visible compatibility story, including the intentional io_uring runtime disablement.
- Add `safe/RELEASE_CHECKLIST.md` that records:
  - the final upstream suite status
  - the relink matrix status
  - the package payload contract
  - the 16-dependent image status
  - the packaged compile-smoke status for `libuv.pc` and `libuv-static.pc`
  - the privileged spawn or security checks
  - the unsafe audit status
  - the `all_cves.json` review status for `CVE-2015-0278`, `CVE-2021-22918`, `CVE-2024-22017`, and `CVE-2024-24806`
- Keep using the existing `safe/tools/audit_unsafe.py` as the enforcement point. Tighten it in place if the final review needs stricter checks, but do not replace it with a new safety-audit tool.

## Verification Phases
### `check-13-dependent-functional-matrix`
- Phase ID: `check-13-dependent-functional-matrix`
- Type: `check`
- Fixed `bounce_target`: `impl-07-dependent-closure-and-release-hardening`
- Purpose: run the full 16-dependent matrix against a freshly rebuilt packaged image and the compatibility wrapper after fixing failures from the smoke image suite.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-07-dependent-closure-and-release-hardening
/home/yans/safelibs/port-libuv/safe/tools/build_deb.sh
/home/yans/safelibs/port-libuv/safe/tools/build_dependents_image.sh --tag libuv-safe-dependents:check-13 --deb-dir /home/yans/safelibs/port-libuv/safe/dist
docker image inspect libuv-safe-dependents:check-13 >/dev/null
/home/yans/safelibs/port-libuv/safe/tools/verify_dependents_image_contract.sh --image libuv-safe-dependents:check-13 --deb-dir /home/yans/safelibs/port-libuv/safe/dist
/home/yans/safelibs/port-libuv/safe/tools/run_dependents_suite.sh --image libuv-safe-dependents:check-13 --mode full --assert-packaged-libuv
LIBUV_TEST_ORIGINAL_IMAGE=libuv-safe-dependents:check-13 /home/yans/safelibs/port-libuv/test-original.sh
LIBUV_SAFE_DEB_DIR=safe/dist LIBUV_TEST_ORIGINAL_IMAGE=libuv-safe-dependents:check-13 /home/yans/safelibs/port-libuv/test-original.sh
```

### `check-14-final-safety-release-review`
- Phase ID: `check-14-final-safety-release-review`
- Type: `check`
- Fixed `bounce_target`: `impl-07-dependent-closure-and-release-hardening`
- Purpose: rerun the full end-to-end verification matrix from current `HEAD` without any repo-wide clean-tree assertion, including a fresh dependent-image rebuild, dependent and wrapper reruns, the final focused-harness fallback parity pass, the full regression manifest, and the release, safety, and history reviews.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-07-dependent-closure-and-release-hardening
cargo build --manifest-path /home/yans/safelibs/port-libuv/safe/Cargo.toml --release
/home/yans/safelibs/port-libuv/safe/tools/stage_install.sh /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/verify_stage_install.sh /tmp/libuv-safe-stage
diff -ruN /home/yans/safelibs/port-libuv/original/include /tmp/libuv-safe-stage/include
/home/yans/safelibs/port-libuv/safe/tools/verify_exports.sh /tmp/libuv-safe-stage/lib/libuv.so /home/yans/safelibs/port-libuv/original/build-checker/libuv.so.1.0.0 /home/yans/safelibs/port-libuv/original/debian/libuv1t64.symbols
/home/yans/safelibs/port-libuv/safe/tools/verify_sizes.sh /tmp/libuv-safe-stage
/home/yans/safelibs/port-libuv/safe/tools/verify_runner_inventory.sh
/home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-final-focused
LIBUV_SAFE_NO_PYTHON=1 /home/yans/safelibs/port-libuv/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-final-focused-nopy
diff -u /tmp/libuv-safe-final-focused/generated/phase-test-list.h /tmp/libuv-safe-final-focused-nopy/generated/phase-test-list.h
diff -u /tmp/libuv-safe-final-focused/generated/uv-safe-run-tests.c /tmp/libuv-safe-final-focused-nopy/generated/uv-safe-run-tests.c
diff -u /tmp/libuv-safe-final-focused/generated/benchmark-sizes-main.c /tmp/libuv-safe-final-focused-nopy/generated/benchmark-sizes-main.c
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-final-focused-nopy --shared --tests version,tcp_ping_pong,spawn_stdout,fs_file_async,getaddrinfo_basic
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 /home/yans/safelibs/port-libuv/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-final-focused-nopy --static --tests version,tcp_ping_pong,fs_file_async,getaddrinfo_basic
cmake -S /home/yans/safelibs/port-libuv/safe/tests/upstream -B /tmp/libuv-safe-final-tests -DLIBUV_SAFE_STAGE=/tmp/libuv-safe-stage
cmake --build /tmp/libuv-safe-final-tests -j"$(nproc)"
diff -u <(/tmp/libuv-safe-final-tests/uv_run_tests --list | sort) <(/home/yans/safelibs/port-libuv/original/build-checker-review/uv_run_tests --list | sort)
diff -u <(/tmp/libuv-safe-final-tests/uv_run_tests_a --list | sort) <(/home/yans/safelibs/port-libuv/original/build-checker-review/uv_run_tests_a --list | sort)
diff -u <(/tmp/libuv-safe-final-tests/uv_run_benchmarks_a --list | sort) <(/home/yans/safelibs/port-libuv/original/build-checker-review/uv_run_benchmarks_a --list | sort)
test "$(/tmp/libuv-safe-final-tests/uv_run_tests --list | awk 'END { print NR }')" = 440
test "$(/tmp/libuv-safe-final-tests/uv_run_tests_a --list | awk 'END { print NR }')" = 440
test "$(/tmp/libuv-safe-final-tests/uv_run_benchmarks_a --list | awk 'END { print NR }')" = 55
RES_OPTIONS=attempts:0 UV_TEST_TIMEOUT_MULTIPLIER=2 ctest --test-dir /tmp/libuv-safe-final-tests --output-on-failure
for build in /home/yans/safelibs/port-libuv/original/build-checker /home/yans/safelibs/port-libuv/original/build-checker-review /home/yans/safelibs/port-libuv/original/build-checker-verify /home/yans/safelibs/port-libuv/original/build-checker-audit; do /home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh --shared --run --build-dir "$build" --stage /tmp/libuv-safe-stage; done
for build in /home/yans/safelibs/port-libuv/original/build-checker /home/yans/safelibs/port-libuv/original/build-checker-review /home/yans/safelibs/port-libuv/original/build-checker-verify /home/yans/safelibs/port-libuv/original/build-checker-audit; do /home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh --static --run --build-dir "$build" --stage /tmp/libuv-safe-stage; done
/home/yans/safelibs/port-libuv/safe/tools/run_regressions.sh --stage /tmp/libuv-safe-stage --up-to-phase impl-07-dependent-closure-and-release-hardening
/home/yans/safelibs/port-libuv/safe/tools/build_deb.sh
python3 /home/yans/safelibs/port-libuv/safe/tools/verify_deb_payload_contract.py /home/yans/safelibs/port-libuv/safe/dist/artifacts.env /home/yans/safelibs/port-libuv/original/include /home/yans/safelibs/port-libuv/safe/debian/not-installed
docker run --rm -i --mount type=bind,src=/home/yans/safelibs/port-libuv,target=/work,readonly ubuntu:24.04 bash -lc '
  set -euo pipefail
  export DEBIAN_FRONTEND=noninteractive
  apt-get update
  apt-get install -y --no-install-recommends build-essential pkg-config
  . /work/safe/dist/artifacts.env
  dpkg -i /work/${LIBUV_SAFE_RUNTIME_DEB_REPO_REL} /work/${LIBUV_SAFE_DEV_DEB_REPO_REL}
  pkg-config --exists libuv
  pkg-config --exists libuv-static
  cat >/tmp/uv_pkgconfig_smoke.c <<'"'"'C'"'"'
#include <uv.h>

int main(void) {
  return uv_version() == 0;
}
C
  cc /tmp/uv_pkgconfig_smoke.c $(pkg-config --cflags --libs libuv) -o /tmp/uv_pkgconfig_smoke
  /tmp/uv_pkgconfig_smoke
  cc /tmp/uv_pkgconfig_smoke.c $(pkg-config --cflags --libs --static libuv-static) -o /tmp/uv_pkgconfig_smoke_static
  /tmp/uv_pkgconfig_smoke_static
'
/home/yans/safelibs/port-libuv/safe/tools/build_dependents_image.sh --tag libuv-safe-dependents:check-14 --deb-dir /home/yans/safelibs/port-libuv/safe/dist
docker image inspect libuv-safe-dependents:check-14 >/dev/null
/home/yans/safelibs/port-libuv/safe/tools/verify_dependents_image_contract.sh --image libuv-safe-dependents:check-14 --deb-dir /home/yans/safelibs/port-libuv/safe/dist
/home/yans/safelibs/port-libuv/safe/tools/run_dependents_suite.sh --image libuv-safe-dependents:check-14 --mode full --assert-packaged-libuv
LIBUV_TEST_ORIGINAL_IMAGE=libuv-safe-dependents:check-14 /home/yans/safelibs/port-libuv/test-original.sh
LIBUV_SAFE_DEB_DIR=safe/dist LIBUV_TEST_ORIGINAL_IMAGE=libuv-safe-dependents:check-14 /home/yans/safelibs/port-libuv/test-original.sh
docker run --rm -i -e UV_RUN_AS_ROOT=1 --mount type=bind,src=/home/yans/safelibs/port-libuv,target=/work --mount type=bind,src=/tmp/libuv-safe-stage,target=/tmp/libuv-safe-stage,readonly ubuntu:24.04 bash -lc 'apt-get update && apt-get install -y --no-install-recommends build-essential cmake pkg-config && LIBUV_SAFE_NO_PYTHON=1 /work/safe/tools/build_upstream_harness.sh --stage /tmp/libuv-safe-stage --build /tmp/libuv-safe-root-tests && UV_RUN_AS_ROOT=1 /work/safe/tools/run_upstream_tests.sh --build /tmp/libuv-safe-root-tests --shared --tests spawn_setuid_setgid,spawn_setuid_fails,spawn_setgid_fails'
python3 - <<'PY'
import json
from pathlib import Path
data = json.loads(Path('/home/yans/safelibs/port-libuv/all_cves.json').read_text())
required = {'CVE-2015-0278', 'CVE-2021-22918', 'CVE-2024-22017', 'CVE-2024-24806'}
assert required.issubset(set(data['confirmed_cve_ids'])), data['confirmed_cve_ids']
PY
python3 /home/yans/safelibs/port-libuv/safe/tools/audit_unsafe.py /home/yans/safelibs/port-libuv/safe/src
! rg -n "unsafe fn" /home/yans/safelibs/port-libuv/safe/src
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
test -f /home/yans/safelibs/port-libuv/safe/COMPATIBILITY_NOTES.md
test -f /home/yans/safelibs/port-libuv/safe/RELEASE_CHECKLIST.md
python3 /home/yans/safelibs/port-libuv/safe/tools/verify_phase_commit_sequence.py --base-file /home/yans/safelibs/port-libuv/.plan/workflow-run-base.txt impl-01-baseline-and-artifact-policy impl-02-core-threading-parity impl-03-io-fs-resolver-parity impl-04-process-signal-security impl-05-full-upstream-and-relink impl-06-packaging-and-dependent-image impl-07-dependent-closure-and-release-hardening
```

- Review checks:
  - Confirm that every dependent failure fixed in phase 7 added or updated a checked-in regression artifact under `safe/tests/regressions/**` or `safe/tests/dependents/**`.
  - Confirm every checked-in executable regression file under `safe/tests/regressions/**` is listed exactly once in `safe/tests/regressions/manifest.json`.
  - Confirm the final privileged checker also sets `LIBUV_SAFE_NO_PYTHON=1` explicitly for its root-gated focused-harness build.
  - Confirm the workflow still does not rely on `git status --short` as a release gate.

## Success Criteria
- `check-13-dependent-functional-matrix` passes the full 16-dependent suite and both `test-original.sh` wrapper modes against a freshly rebuilt packaged image.
- `check-14-final-safety-release-review` passes the full end-to-end matrix, final focused-harness fallback parity, the full regression manifest, and commit-history validation.
- Every fixed dependent issue leaves behind a checked-in reproducer or probe plus the matching manifest update.
- `safe/COMPATIBILITY_NOTES.md` and `safe/RELEASE_CHECKLIST.md` capture the final externally visible compatibility and release status.
- The latest commit produced by this phase begins with `impl-07-dependent-closure-and-release-hardening`.

## Git Commit Requirement
The implementer must commit the phase work to git before yielding. The newest commit at `HEAD` must remain the handoff commit for this phase, and its subject must begin with `impl-07-dependent-closure-and-release-hardening`.
