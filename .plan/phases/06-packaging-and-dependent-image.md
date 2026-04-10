# Debian Packaging, Exact Payload Verification, and Dependent Image Build

Phase Name: Debian Packaging, Exact Payload Verification, and Dependent Image Build

Implement Phase ID: `impl-06-packaging-and-dependent-image`

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
- `/home/yans/safelibs/port-libuv/safe/tools/cc-linker.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/abi-baseline.json`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_exports.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_sizes.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/build_deb.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/relink_original_tests.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/update_bindings.sh`
- `/home/yans/safelibs/port-libuv/safe/tests/upstream/**`
- `/home/yans/safelibs/port-libuv/safe/tests/regressions/**`
- `/home/yans/safelibs/port-libuv/safe/debian/**`
- `/home/yans/safelibs/port-libuv/original/include/**`
- `/home/yans/safelibs/port-libuv/original/debian/libuv1t64.symbols`
- `/home/yans/safelibs/port-libuv/original/build-checker/**`
- `/home/yans/safelibs/port-libuv/original/build-checker-review/**`
- `/home/yans/safelibs/port-libuv/original/build-checker-verify/**`
- `/home/yans/safelibs/port-libuv/original/build-checker-audit/**`
- `/home/yans/safelibs/port-libuv/dependents.json`
- `/home/yans/safelibs/port-libuv/test-original.sh`

## New Outputs
- updated `/home/yans/safelibs/port-libuv/safe/tools/build_deb.sh`
- updated `/home/yans/safelibs/port-libuv/safe/debian/**` as needed for exact package contents
- `/home/yans/safelibs/port-libuv/safe/tools/verify_deb_payload_contract.py`
- `/home/yans/safelibs/port-libuv/safe/tools/build_dependents_image.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/run_dependents_suite.sh`
- `/home/yans/safelibs/port-libuv/safe/tools/verify_dependents_image_contract.sh`
- `/home/yans/safelibs/port-libuv/safe/docker/Dockerfile.dependents`
- `/home/yans/safelibs/port-libuv/safe/docker/run-dependent-probes.sh`
- `/home/yans/safelibs/port-libuv/safe/tests/dependents/manifest.json`
- `/home/yans/safelibs/port-libuv/safe/tests/dependents/common.sh`
- checked-in probe files under `/home/yans/safelibs/port-libuv/safe/tests/dependents/**`
- updated `/home/yans/safelibs/port-libuv/test-original.sh`
- refreshed `/home/yans/safelibs/port-libuv/safe/dist/artifacts.env`

## File Changes
- Make Debian artifact selection and payload verification deterministic.
- Replace the inline dependent harness with checked-in probe assets plus a reproducible Docker image build path.
- Keep `test-original.sh` as the compatibility wrapper, but make it reuse the new checked-in assets.

## Implementation Details
- Keep `safe/tools/build_deb.sh` as the authoritative package build entrypoint. It already cleans `safe/dist`; extend it only where the manifest or package movement logic needs hardening.
- Add `verify_deb_payload_contract.py` so package verification is exact:
  - runtime package: only shared-library payload plus standard Debian docs for `libuv1t64`
  - dev package: `uv.h`, every `original/include/uv/**` header, `libuv.a`, `libuv.so`, `libuv.pc`, `libuv-static.pc`, and standard Debian docs for `libuv1-dev`
  - unpack the dev package and compare the packaged public headers byte-for-byte against `original/include/**`
  - enforce `safe/debian/not-installed`
- `check-11` must also prove packaged compile compatibility in a disposable Ubuntu 24.04 container by installing the locally built `.deb`s from `safe/dist/artifacts.env`, then compiling and running a trivial client through both `pkg-config --libs libuv` and `pkg-config --libs --static libuv-static`.
- Split the inline probes from `test-original.sh` into checked-in assets under `safe/tests/dependents/**`.
- Create `safe/tests/dependents/manifest.json` that maps the 16 entries from `dependents.json` to the fixed IDs `nodejs`, `neovim`, `bind9`, `knot-resolver`, `ttyd`, `luv-lua`, `luv-ocaml`, `uvloop`, `r-fs`, `r-httpuv`, `moarvm`, `h2o`, `libraft`, `dqlite`, `libstorj`, and `gevent`, keeping the same entry order as `dependents.json`.
- `safe/tests/dependents/manifest.json` must use the exact schema `{"schema_version": 1, "dependents": [...]}`.
- Each manifest entry must contain:
  - `id`
  - `software_name`
  - `coverage` as a list containing `runtime`, `source-build`, or both
  - `probe_path` as a relative `.sh` entrypoint under `safe/tests/dependents/`
  - `expected_link_targets` as a non-empty list of `{kind, locator}` objects, where `kind` is one of `path`, `python-module`, `r-package`, or `probe-glob`
- Create a shared helper `safe/tests/dependents/common.sh` for:
  - package-owned libuv discovery
  - `ldd` resolution checks
  - Python module path resolution
  - R package `.so` discovery
  - `probe-glob` resolution for source-built artifacts such as gevent
- Preserve the compile-oriented probes already present in `test-original.sh` when extracting them: `luv-ocaml`, `dqlite`, `libstorj`, and `gevent` must remain source-build checks, not be downgraded to runtime-only probes.
- Do not add new entries under `safe/tests/regressions/**` in this phase; image or build compatibility coverage for phase 6 belongs under `safe/tests/dependents/**` and its manifest.
- Build a real Docker image with `safe/docker/Dockerfile.dependents` that:
  - starts from Ubuntu 24.04
  - installs the dependency set currently installed inline in `test-original.sh`
  - installs the safe runtime and dev `.deb`s from `safe/dist/artifacts.env`
  - provides the image-side dispatcher in `safe/docker/run-dependent-probes.sh`, while the host-side runners bind-mount the current checked-in probe tree from `safe/tests/dependents/**` read-only so current-`HEAD` probe edits cannot hide inside a stale image layer
- Implement `build_dependents_image.sh` with the exact CLI used by the checkers: `--tag <tag>` and `--deb-dir <dir>`, where `<dir>` must contain `artifacts.env`. The helper must always rebuild the requested tag from the current working tree and current `safe/dist/artifacts.env`; it must not treat an already-existing tag as valid evidence for a later checker.
- Implement `run_dependents_suite.sh` with the exact CLI used by the checkers: `--image <tag>`, `--mode smoke|full`, optional `--dependents <comma-separated-id-list>`, and `--assert-packaged-libuv`. It must bind-mount the current checked-in dependent manifest and probe tree into the container and execute `probe_path` from that mounted tree.
- Implement `verify_dependents_image_contract.sh` with the exact CLI used by the checkers: `--image <tag>` and `--deb-dir <dir>`, and have it validate the installed package metadata against `artifacts.env`.
- Update `test-original.sh` so:
  - original-mode still builds and uses upstream libuv when `LIBUV_SAFE_DEB_DIR` is unset
  - safe-package mode reuses the extracted probe assets instead of redefining them inline
  - both modes dispatch through the same checked-in dependent probe entrypoints from the current repository tree, rather than maintaining separate inline copies or relying on probe scripts baked into an older image

## Verification Phases
### `check-11-package-payload-and-image-contract`
- Phase ID: `check-11-package-payload-and-image-contract`
- Type: `check`
- Fixed `bounce_target`: `impl-06-packaging-and-dependent-image`
- Purpose: verify exact package payloads, including byte-identical public headers, and build an explicit Docker image that installs the locally built safe packages.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-06-packaging-and-dependent-image
/home/yans/safelibs/port-libuv/safe/tools/build_deb.sh
test -f /home/yans/safelibs/port-libuv/safe/dist/artifacts.env
. /home/yans/safelibs/port-libuv/safe/dist/artifacts.env && [ -n "${LIBUV_SAFE_RUNTIME_DEB:-}" ] && [ -f "$LIBUV_SAFE_RUNTIME_DEB" ] && [ -n "${LIBUV_SAFE_DEV_DEB:-}" ] && [ -f "$LIBUV_SAFE_DEV_DEB" ]
. /home/yans/safelibs/port-libuv/safe/dist/artifacts.env && [ -n "${LIBUV_SAFE_RUNTIME_DEB_REPO_REL:-}" ] && [ "${LIBUV_SAFE_RUNTIME_DEB_REPO_REL#/}" = "$LIBUV_SAFE_RUNTIME_DEB_REPO_REL" ] && [ -f "/home/yans/safelibs/port-libuv/${LIBUV_SAFE_RUNTIME_DEB_REPO_REL}" ]
. /home/yans/safelibs/port-libuv/safe/dist/artifacts.env && [ -n "${LIBUV_SAFE_DEV_DEB_REPO_REL:-}" ] && [ "${LIBUV_SAFE_DEV_DEB_REPO_REL#/}" = "$LIBUV_SAFE_DEV_DEB_REPO_REL" ] && [ -f "/home/yans/safelibs/port-libuv/${LIBUV_SAFE_DEV_DEB_REPO_REL}" ]
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
/home/yans/safelibs/port-libuv/safe/tools/build_dependents_image.sh --tag libuv-safe-dependents:check-11 --deb-dir /home/yans/safelibs/port-libuv/safe/dist
docker image inspect libuv-safe-dependents:check-11 >/dev/null
/home/yans/safelibs/port-libuv/safe/tools/verify_dependents_image_contract.sh --image libuv-safe-dependents:check-11 --deb-dir /home/yans/safelibs/port-libuv/safe/dist
```

### `check-12-dependent-image-smoke`
- Phase ID: `check-12-dependent-image-smoke`
- Type: `check`
- Fixed `bounce_target`: `impl-06-packaging-and-dependent-image`
- Purpose: prove that a freshly rebuilt dependent image and the extracted probe assets can run a high-signal smoke subset before the full dependent sweep.
- Commands:

```bash
/home/yans/safelibs/port-libuv/safe/tools/verify_phase_head.sh impl-06-packaging-and-dependent-image
/home/yans/safelibs/port-libuv/safe/tools/build_deb.sh
/home/yans/safelibs/port-libuv/safe/tools/build_dependents_image.sh --tag libuv-safe-dependents:check-12 --deb-dir /home/yans/safelibs/port-libuv/safe/dist
docker image inspect libuv-safe-dependents:check-12 >/dev/null
/home/yans/safelibs/port-libuv/safe/tools/verify_dependents_image_contract.sh --image libuv-safe-dependents:check-12 --deb-dir /home/yans/safelibs/port-libuv/safe/dist
/home/yans/safelibs/port-libuv/safe/tools/run_dependents_suite.sh --image libuv-safe-dependents:check-12 --mode smoke --assert-packaged-libuv --dependents nodejs,neovim,bind9,uvloop,luv-ocaml,dqlite
python3 - <<'PY'
import json
from pathlib import Path
dep = json.loads(Path('/home/yans/safelibs/port-libuv/dependents.json').read_text())
manifest = json.loads(Path('/home/yans/safelibs/port-libuv/safe/tests/dependents/manifest.json').read_text())
assert set(manifest) == {'schema_version', 'dependents'}, manifest
expected_ids = [
    'nodejs', 'neovim', 'bind9', 'knot-resolver', 'ttyd', 'luv-lua', 'luv-ocaml',
    'uvloop', 'r-fs', 'r-httpuv', 'moarvm', 'h2o', 'libraft', 'dqlite', 'libstorj', 'gevent',
]
assert manifest['schema_version'] == 1
assert dep['selected_dependents_count'] == 16
assert len(manifest['dependents']) == dep['selected_dependents_count']
assert [item['software_name'] for item in manifest['dependents']] == [item['software_name'] for item in dep['dependents']]
assert [item['id'] for item in manifest['dependents']] == expected_ids
expected_keys = {'id', 'software_name', 'coverage', 'probe_path', 'expected_link_targets'}
allowed_kinds = {'path', 'python-module', 'r-package', 'probe-glob'}
for item in manifest['dependents']:
    assert set(item) == expected_keys, item
    assert isinstance(item['coverage'], list) and item['coverage'], item
    assert set(item['coverage']) <= {'runtime', 'source-build'}, item
    assert item['probe_path'].endswith('.sh') and not item['probe_path'].startswith('/'), item['probe_path']
    assert isinstance(item['expected_link_targets'], list) and item['expected_link_targets'], item
    for target in item['expected_link_targets']:
        assert set(target) == {'kind', 'locator'}, target
        assert target['kind'] in allowed_kinds, target
coverage = {item['id']: set(item['coverage']) for item in manifest['dependents']}
assert 'source-build' in coverage['luv-ocaml']
assert 'source-build' in coverage['dqlite']
assert 'source-build' in coverage['libstorj']
assert 'source-build' in coverage['gevent']
target_kinds = {item['id']: [target['kind'] for target in item['expected_link_targets']] for item in manifest['dependents']}
assert target_kinds['nodejs'] == ['path']
assert target_kinds['uvloop'] == ['python-module']
assert target_kinds['r-fs'] == ['r-package']
assert target_kinds['r-httpuv'] == ['r-package']
assert target_kinds['gevent'] == ['probe-glob']
PY
```

- Review checks:
  - Confirm `verify_deb_payload_contract.py` unpacks the `libuv1-dev` payload and compares the packaged public headers byte-for-byte against `original/include/**`, not just by filename presence.
  - Confirm `test-original.sh` no longer owns the probe logic inline; it should delegate to checked-in probe assets or a shared helper path.
  - Confirm the image installs the locally built `.deb`s referenced by `safe/dist/artifacts.env`, not distro `libuv1t64`.
  - Confirm the smoke subset still contains both runtime probes and source-build probes.

## Success Criteria
- `check-11-package-payload-and-image-contract` passes the package payload verification, packaged compile smoke, and dependent-image contract checks.
- `check-12-dependent-image-smoke` passes the smoke subset and the manifest-to-`dependents.json` consistency assertions.
- The dependent-image flow consumes the locally built `.deb`s from `safe/dist/artifacts.env` and the checked-in probe assets from `safe/tests/dependents/**`.
- The latest commit produced by this phase begins with `impl-06-packaging-and-dependent-image`.

## Git Commit Requirement
The implementer must commit the phase work to git before yielding. The newest commit at `HEAD` must remain the handoff commit for this phase, and its subject must begin with `impl-06-packaging-and-dependent-image`.
