# libuv-safe network/stream validator review (phase-18)

## Phase

- Phase: `impl-18-validator-network-io-fixes`
- Purpose: network, pipe, stream, poll, TCP, UDP, loopback HTTP/HTTP2,
  Node.js `net.*`, `dgram.*`, and streams/pipeline validator review.
- Validator commit used: `87b321fe728340d6fc6dd2f638583cca82c667c3`
- Source outcome: no `safe/src/unix/{stream,tcp,udp,pipe,poll,fd,epoll}.rs`,
  `safe/src/upstream_support/{unix_core,unix_loop}.rs`, export, ABI, or
  regression changes were required.

## phase-18 artifacts

- Artifact root: `validator/artifacts/libuv-safe/phase-18`
- Local override root: `validator/artifacts/libuv-safe/phase-18/local-debs/libuv`
- Per-case results: `validator/artifacts/libuv-safe/phase-18/results/libuv`
- Per-case logs: `validator/artifacts/libuv-safe/phase-18/logs/libuv`
- Cast recordings: `validator/artifacts/libuv-safe/phase-18/casts/libuv`
- Artifacts are generated under gitignored `validator/` and are not committed.

## Commands run

```bash
cargo build --manifest-path safe/Cargo.toml --release
safe/tools/stage_install.sh /tmp/libuv-safe-validator-stage
safe/tools/verify_stage_install.sh /tmp/libuv-safe-validator-stage
safe/tools/run_regressions.sh \
  --stage /tmp/libuv-safe-validator-stage \
  --up-to-phase impl-18-validator-network-io-fixes
safe/tools/build_deb.sh

ARTIFACT_ROOT="$PWD/validator/artifacts/libuv-safe/phase-18"
OVERRIDE_ROOT="$ARTIFACT_ROOT/local-debs"
rm -rf "$ARTIFACT_ROOT"
mkdir -p "$OVERRIDE_ROOT/libuv"
cp safe/dist/libuv1t64_*.deb "$OVERRIDE_ROOT/libuv/"
cp safe/dist/libuv1-dev_*.deb "$OVERRIDE_ROOT/libuv/"

bash validator/test.sh \
  --config validator/repositories.yml \
  --tests-root validator/tests \
  --artifact-root "$ARTIFACT_ROOT" \
  --mode original \
  --override-deb-root "$OVERRIDE_ROOT" \
  --library libuv \
  --record-casts
```

The validator command exited with status 0.

## Packages

Read from `safe/dist/artifacts.env` and copied into the phase-18 override root:

| Package | Version | Architecture | SHA-256 |
| --- | --- | --- | --- |
| `libuv1t64` | `1.48.0-1.1build1+safelibs1` | `amd64` | `f46e5a6b20c43f3adbf02fb914e9451b5164c9141e7eae0f82bcc8a26ecc7d7d` |
| `libuv1-dev` | `1.48.0-1.1build1+safelibs1` | `amd64` | `6df4475d00d0e1f3420eb2d46fad94e22a3e4fc3df832eb95ae8add030e63405` |

## Result

Fresh libuv testcase counts from the validator checkout:

| Kind | Count |
| --- | ---: |
| Source cases | 5 |
| Usage cases | 170 |
| Total cases | 175 |

`validator/artifacts/libuv-safe/phase-18/results/libuv/summary.json` records:

| Field | Value |
| --- | ---: |
| `mode` | `original` |
| `passed` | 175 |
| `failed` | 0 |
| `casts` | 175 |

Override coverage: all 175 non-summary result JSON files contain
`override_debs_installed: true`, so every validator case installed the local
`libuv1t64` and `libuv1-dev` packages.

## Review

No phase-18 network, pipe, stream, poll, TCP, UDP, loopback HTTP/HTTP2,
Node.js `net.*`, Node.js `dgram.*`, or streams/pipeline failures were
recorded. The phase-owned source case `tcp-loopback-smoke` passes, and
`process-pipe-smoke` also passes with no pipe/stream root cause to own.

Existing focused regressions covering this surface continue to run through
phase 18: `validator_tcp_loopback_echo.c`,
`validator_pipe_socketpair_stream.c`, and
`validator_udp_loopback_send_recv.c`. No new regression probe is needed because
there was no newly fixed phase-18 defect.

Focused code review found no required source fix for the phase-owned
invariants:

- `safe/src/unix/stream.rs`: listen, accept, read, write, try-write,
  shutdown, and close paths preserve callback registration, accepted socket
  transfer, EOF/error delivery, queue-size draining, and single completion.
- `safe/src/unix/tcp.rs`: bind, connect, accept-derived peer/socket names,
  nodelay, keepalive, and close/reset behavior match the passing validator
  loopback probes.
- `safe/src/unix/udp.rs`: bound loopback recv, connected and unconnected send,
  send callback completion, address population, peer/socket names, queue
  counters, broadcast, TTL, and multicast option paths match the passing
  validator probes.
- `safe/src/unix/pipe.rs`, `safe/src/unix/poll.rs`,
  `safe/src/unix/fd.rs`, and `safe/src/unix/epoll.rs`: pipe stream adoption,
  fd readiness registration, polling, and close invalidation did not produce
  any phase-owned source or usage failure.

## Failures

No `impl-18-validator-network-io-fixes` failure owner assignments, source
fixes, or new regression probes are required.

## Skipped validator bugs

None. No testcase was treated as a validator bug and no testcase was excluded
from acceptance accounting.

## Historical report

# libuv-safe filesystem/DNS/process validator review (phase-17)

## Phase

- Phase: `impl-17-validator-fs-dns-process-fixes`
- Purpose: filesystem, DNS/resolver, process, spawn, stdio/reaping, signal
  restore, and error-name validator review.
- Validator commit used: `87b321fe728340d6fc6dd2f638583cca82c667c3`
- Source outcome: no `safe/src/unix/{fs,fs_event,fs_poll,getaddrinfo,getnameinfo,process,pipe,signal,fd}.rs`,
  `safe/src/core/error.rs`, `safe/src/threading/threadpool.rs`, export, ABI,
  or regression changes were required.

## phase-17 artifacts

- Artifact root: `validator/artifacts/libuv-safe/phase-17`
- Local override root: `validator/artifacts/libuv-safe/phase-17/local-debs/libuv`
- Per-case results: `validator/artifacts/libuv-safe/phase-17/results/libuv`
- Per-case logs: `validator/artifacts/libuv-safe/phase-17/logs/libuv`
- Cast recordings: `validator/artifacts/libuv-safe/phase-17/casts/libuv`
- Artifacts are generated under gitignored `validator/` and are not committed.

## Commands run

```bash
cargo build --manifest-path safe/Cargo.toml --release
safe/tools/stage_install.sh /tmp/libuv-safe-validator-stage
safe/tools/verify_stage_install.sh /tmp/libuv-safe-validator-stage
safe/tools/run_regressions.sh \
  --stage /tmp/libuv-safe-validator-stage \
  --up-to-phase impl-17-validator-fs-dns-process-fixes
safe/tools/build_deb.sh

ARTIFACT_ROOT="$PWD/validator/artifacts/libuv-safe/phase-17"
OVERRIDE_ROOT="$ARTIFACT_ROOT/local-debs"
rm -rf "$ARTIFACT_ROOT"
mkdir -p "$OVERRIDE_ROOT/libuv"
cp safe/dist/libuv1t64_*.deb "$OVERRIDE_ROOT/libuv/"
cp safe/dist/libuv1-dev_*.deb "$OVERRIDE_ROOT/libuv/"

bash validator/test.sh \
  --config validator/repositories.yml \
  --tests-root validator/tests \
  --artifact-root "$ARTIFACT_ROOT" \
  --mode original \
  --override-deb-root "$OVERRIDE_ROOT" \
  --library libuv \
  --record-casts
```

The validator command exited with status 0.

## Packages

Read from `safe/dist/artifacts.env` and copied into the phase-17 override root:

| Package | Version | Architecture | SHA-256 |
| --- | --- | --- | --- |
| `libuv1t64` | `1.48.0-1.1build1+safelibs1` | `amd64` | `f46e5a6b20c43f3adbf02fb914e9451b5164c9141e7eae0f82bcc8a26ecc7d7d` |
| `libuv1-dev` | `1.48.0-1.1build1+safelibs1` | `amd64` | `6df4475d00d0e1f3420eb2d46fad94e22a3e4fc3df832eb95ae8add030e63405` |

## Result

Fresh libuv testcase counts from the validator checkout:

| Kind | Count |
| --- | ---: |
| Source cases | 5 |
| Usage cases | 170 |
| Total cases | 175 |

`validator/artifacts/libuv-safe/phase-17/results/libuv/summary.json` records:

| Field | Value |
| --- | ---: |
| `mode` | `original` |
| `passed` | 175 |
| `failed` | 0 |
| `casts` | 175 |

Override coverage: all 175 non-summary result JSON files contain
`override_debs_installed: true`, so every validator case installed the local
`libuv1t64` and `libuv1-dev` packages.

## Review

No phase-17 filesystem, DNS, resolver, process, spawn, child stdio, process
exit-status, process close, signal-restore, reaping, or error-name failures
were recorded. The phase-owned source cases `dns-getaddrinfo.sh` and
`fs-read-write.sh` pass, as do the Node.js usage probes for `fs.*`,
`fs.promises.*`, `opendir`, `watch`, `watchFile`, symlink/readlink/realpath,
stat/statfs, chmod/truncate/readv/writev, `dns.lookup`, resolver promises, and
`child_process.*`.

Focused code review found no required source fix:

- `safe/src/unix/fs.rs`: `fs_req_init` preserves caller `data`, clears the
  request, and sets type, loop, fs type, and callback fields; `access` routes
  through `init_path_request` and `run_or_submit`; `req_cleanup` releases owned
  async path/new-path/buffer allocations while preserving synchronous request
  ownership compatibly.
- `safe/src/core/error.rs`: positive errno inputs canonicalize to negative
  libuv codes through the ABI constants, known codes including `ENOENT` map to
  upstream `err_name`/`strerror` names and messages, and unknown direct and
  `_r` variants format as `Unknown system error <raw-code>`.
- `safe/src/unix/getaddrinfo.rs` and `safe/src/threading/threadpool.rs`:
  resolver host/service/hints inputs are cloned before async dispatch, freed in
  completion, delivered on the owning loop, and `uv_freeaddrinfo` delegates to
  libc `freeaddrinfo`; the existing long-hostname IDNA regression remains in
  the regression suite.
- `safe/src/unix/process.rs`: spawn validates and opens stdio, reports failed
  child setup or `execvp` through the error pipe, applies cwd, environment,
  uid/gid flags, signal disposition/mask handling, and reaps children through
  `process_reap` with one exit callback.

Existing focused regressions covering this area continue to run through phase
17: `fs_readlink_proc_self.c`, `getaddrinfo_long_hostname.c`, and
`validator_fs_enoent_error_names.c`. No new regression probe is needed because
there was no newly fixed phase-17 defect.

## Failures

No `impl-17-validator-fs-dns-process-fixes` failure owner assignments, source
fixes, or new regression probes are required.

## Skipped validator bugs

None. No testcase was treated as a validator bug and no testcase was excluded
from acceptance accounting.

## Historical report

# libuv-safe core loop/threading validator review (phase-16)

## Phase

- Phase: `impl-16-validator-core-loop-threading-fixes`
- Purpose: core loop, timer, async, threadpool, threading, and random
  validator review.
- Validator commit used: `87b321fe728340d6fc6dd2f638583cca82c667c3`
- Source outcome: no `safe/src/core/**`, `safe/src/unix/{epoll,async}.rs`,
  `safe/src/threading/**`, export, ABI, or regression changes were required.

## phase-16 artifacts

- Artifact root: `validator/artifacts/libuv-safe/phase-16`
- Local override root: `validator/artifacts/libuv-safe/phase-16/local-debs/libuv`
- Per-case results: `validator/artifacts/libuv-safe/phase-16/results/libuv`
- Per-case logs: `validator/artifacts/libuv-safe/phase-16/logs/libuv`
- Cast recordings: `validator/artifacts/libuv-safe/phase-16/casts/libuv`
- Artifacts are generated under gitignored `validator/` and are not committed.

## Commands run

```bash
cargo build --manifest-path safe/Cargo.toml --release
safe/tools/stage_install.sh /tmp/libuv-safe-validator-stage
safe/tools/verify_stage_install.sh /tmp/libuv-safe-validator-stage
safe/tools/run_regressions.sh \
  --stage /tmp/libuv-safe-validator-stage \
  --up-to-phase impl-16-validator-core-loop-threading-fixes
safe/tools/build_deb.sh

ARTIFACT_ROOT="$PWD/validator/artifacts/libuv-safe/phase-16"
OVERRIDE_ROOT="$ARTIFACT_ROOT/local-debs"
rm -rf "$ARTIFACT_ROOT"
mkdir -p "$OVERRIDE_ROOT/libuv"
cp safe/dist/libuv1t64_*.deb "$OVERRIDE_ROOT/libuv/"
cp safe/dist/libuv1-dev_*.deb "$OVERRIDE_ROOT/libuv/"

bash validator/test.sh \
  --config validator/repositories.yml \
  --tests-root validator/tests \
  --artifact-root "$ARTIFACT_ROOT" \
  --mode original \
  --override-deb-root "$OVERRIDE_ROOT" \
  --library libuv \
  --record-casts
```

The validator command exited with status 0.

## Packages

Read from `safe/dist/artifacts.env` and copied into the phase-16 override root:

| Package | Version | Architecture | SHA-256 |
| --- | --- | --- | --- |
| `libuv1t64` | `1.48.0-1.1build1+safelibs1` | `amd64` | `f46e5a6b20c43f3adbf02fb914e9451b5164c9141e7eae0f82bcc8a26ecc7d7d` |
| `libuv1-dev` | `1.48.0-1.1build1+safelibs1` | `amd64` | `6df4475d00d0e1f3420eb2d46fad94e22a3e4fc3df832eb95ae8add030e63405` |

## Result

Fresh libuv testcase counts from the validator checkout:

| Kind | Count |
| --- | ---: |
| Source cases | 5 |
| Usage cases | 170 |
| Total cases | 175 |

`validator/artifacts/libuv-safe/phase-16/results/libuv/summary.json` records:

| Field | Value |
| --- | ---: |
| `mode` | `original` |
| `passed` | 175 |
| `failed` | 0 |
| `casts` | 175 |

Override coverage: all 175 non-summary result JSON files contain
`override_debs_installed: true`, so every validator case installed the local
`libuv1t64` and `libuv1-dev` packages.

## Failures

No phase-16 event-loop, timer, async, threadpool, worker scheduling, random,
crypto randomness, scrypt, or zlib/threadpool-completion failures were
recorded. Existing focused regressions owned by the earlier core-loop pass
continue to run through phase 16:
`validator_timer_ordering_close.c`, `validator_threadpool_queue_work.c`, and
`validator_random_sync_async.c`.

No `impl-16-validator-core-loop-threading-fixes` failure owner assignments,
source fixes, or new regression probes are required.

## Skipped validator bugs

None. No testcase was treated as a validator bug and no testcase was excluded
from acceptance accounting.

## Historical report

# libuv-safe packaging/ABI validator review (phase-15)

## Phase

- Phase: `impl-15-validator-packaging-abi-fixes`
- Purpose: packaging, ABI, public header, symbol, pkg-config, and validator
  local-override setup review.
- Validator commit used: `87b321fe728340d6fc6dd2f638583cca82c667c3`
- Source outcome: no source, packaging, ABI, export, header, tool, or
  regression changes were required.

## phase-15 artifacts

- Artifact root: `validator/artifacts/libuv-safe/phase-15`
- Local override root: `validator/artifacts/libuv-safe/phase-15/local-debs/libuv`
- Per-case results: `validator/artifacts/libuv-safe/phase-15/results/libuv`
- Per-case logs: `validator/artifacts/libuv-safe/phase-15/logs/libuv`
- Cast recordings: `validator/artifacts/libuv-safe/phase-15/casts/libuv`
- Artifacts are generated under gitignored `validator/` and are not committed.

## Commands run

```bash
safe/tools/build_deb.sh
python3 safe/tools/verify_deb_payload_contract.py \
  safe/dist/artifacts.env \
  original/include \
  safe/debian/not-installed
cargo build --manifest-path safe/Cargo.toml --release
safe/tools/stage_install.sh /tmp/libuv-safe-validator-stage
safe/tools/verify_stage_install.sh /tmp/libuv-safe-validator-stage
safe/tools/run_regressions.sh \
  --stage /tmp/libuv-safe-validator-stage \
  --up-to-phase impl-15-validator-packaging-abi-fixes

ARTIFACT_ROOT="$PWD/validator/artifacts/libuv-safe/phase-15"
OVERRIDE_ROOT="$ARTIFACT_ROOT/local-debs"
rm -rf "$ARTIFACT_ROOT"
mkdir -p "$OVERRIDE_ROOT/libuv"
cp safe/dist/libuv1t64_*.deb "$OVERRIDE_ROOT/libuv/"
cp safe/dist/libuv1-dev_*.deb "$OVERRIDE_ROOT/libuv/"

bash validator/test.sh \
  --config validator/repositories.yml \
  --tests-root validator/tests \
  --artifact-root "$ARTIFACT_ROOT" \
  --mode original \
  --override-deb-root "$OVERRIDE_ROOT" \
  --library libuv \
  --record-casts
```

The validator command exited with status 0.

## Packages

Read from `safe/dist/artifacts.env` and copied into the phase-15 override root:

| Package | Version | Architecture | SHA-256 |
| --- | --- | --- | --- |
| `libuv1t64` | `1.48.0-1.1build1+safelibs1` | `amd64` | `f46e5a6b20c43f3adbf02fb914e9451b5164c9141e7eae0f82bcc8a26ecc7d7d` |
| `libuv1-dev` | `1.48.0-1.1build1+safelibs1` | `amd64` | `6df4475d00d0e1f3420eb2d46fad94e22a3e4fc3df832eb95ae8add030e63405` |

## Result

Fresh libuv testcase counts from the validator checkout:

| Kind | Count |
| --- | ---: |
| Source cases | 5 |
| Usage cases | 170 |
| Total cases | 175 |

`validator/artifacts/libuv-safe/phase-15/results/libuv/summary.json` records:

| Field | Value |
| --- | ---: |
| `mode` | `original` |
| `passed` | 175 |
| `failed` | 0 |
| `casts` | 175 |

Override coverage: all 175 non-summary result JSON files contain
`override_debs_installed: true`, so every validator case installed the local
`libuv1t64` and `libuv1-dev` packages.

## Failures

No phase-15 packaging, ABI, header, symbol, pkg-config, Debian package-name,
multiarch path, SONAME, or local-override failures were recorded. No
`impl-15-validator-packaging-abi-fixes` failure owner assignments or new
regression probes are required.

## Skipped validator bugs

None. No testcase was treated as a validator bug and no testcase was excluded
from acceptance accounting.

## Historical report

# libuv-safe validator refresh (phase-14)

## Validator commit

- Phase: `impl-14-validator-refresh-baseline`
- Workflow base before phase-14: `90e55a2b45ebc0db3af14426754c6a2abf656b7e`
- Validator checkout: `validator/` (gitignored nested repository; no tracked validator files committed)
- Validator update command: `git -C validator pull --ff-only`
- Validator commit before refresh: `87b321fe728340d6fc6dd2f638583cca82c667c3`
- Validator commit used for phase-14: `87b321fe728340d6fc6dd2f638583cca82c667c3`
- Fast-forward result: already up to date

## phase-14 artifacts

- Artifact root: `validator/artifacts/libuv-safe/phase-14`
- Local override root: `validator/artifacts/libuv-safe/phase-14/local-debs/libuv`
- Per-case results: `validator/artifacts/libuv-safe/phase-14/results/libuv`
- Per-case logs: `validator/artifacts/libuv-safe/phase-14/logs/libuv`
- Cast recordings: `validator/artifacts/libuv-safe/phase-14/casts/libuv`
- Historical comparison roots preserved in place: `validator/artifacts/libuv-safe/phase-08`, `phase-09`, `phase-10`, `phase-11`, `phase-12`, `validator/artifacts/libuv-safe/check-25-final`, and `validator/site/libuv-safe-final`.

## Commands run

```bash
git rev-parse HEAD > .plan/workflow-run-base.txt
git -C validator pull --ff-only

cmake -S original -B original/build-checker \
  -DBUILD_TESTING=OFF \
  -DCMAKE_BUILD_TYPE=RelWithDebInfo
cmake --build original/build-checker -j"$(nproc)"

safe/tools/build_deb.sh

ARTIFACT_ROOT="$PWD/validator/artifacts/libuv-safe/phase-14"
OVERRIDE_ROOT="$ARTIFACT_ROOT/local-debs"
rm -rf "$ARTIFACT_ROOT"
mkdir -p "$OVERRIDE_ROOT/libuv"
cp safe/dist/libuv1t64_*.deb "$OVERRIDE_ROOT/libuv/"
cp safe/dist/libuv1-dev_*.deb "$OVERRIDE_ROOT/libuv/"

make -C validator unit
make -C validator check-testcases

bash validator/test.sh \
  --config validator/repositories.yml \
  --tests-root validator/tests \
  --artifact-root validator/artifacts/libuv-safe/phase-14 \
  --mode original \
  --override-deb-root validator/artifacts/libuv-safe/phase-14/local-debs \
  --library libuv \
  --record-casts
```

The baseline run intentionally used validator `--mode original` with
`--override-deb-root` pointed at locally built `libuv-safe` packages. No
original-mode proof or final site output was generated from this override run.

## Packages

Read from `safe/dist/artifacts.env` and copied into the phase-14 override root:

| Package | Version | Architecture | SHA-256 |
| --- | --- | --- | --- |
| `libuv1t64` | `1.48.0-1.1build1+safelibs1` | `amd64` | `f46e5a6b20c43f3adbf02fb914e9451b5164c9141e7eae0f82bcc8a26ecc7d7d` |
| `libuv1-dev` | `1.48.0-1.1build1+safelibs1` | `amd64` | `6df4475d00d0e1f3420eb2d46fad94e22a3e4fc3df832eb95ae8add030e63405` |

## Baseline result

Fresh libuv testcase counts from the validator checkout after the pull:

| Kind | Count |
| --- | ---: |
| Source cases | 5 |
| Usage cases | 170 |
| Total cases | 175 |

`validator/artifacts/libuv-safe/phase-14/results/libuv/summary.json` records:

| Field | Value |
| --- | ---: |
| `mode` | `original` |
| `passed` | 175 |
| `failed` | 0 |
| `casts` | 175 |

Override coverage: all 175 non-summary result JSON files contain
`override_debs_installed: true`.

## Failures

No phase-14 baseline failures were recorded. No owner phase assignment is
required for `impl-15-validator-packaging-abi-fixes`,
`impl-16-validator-core-loop-threading-fixes`,
`impl-17-validator-fs-dns-process-fixes`,
`impl-18-validator-network-io-fixes`, or
`impl-19-validator-catchall-final-report`.

## Skipped validator bugs

None. No testcase was treated as a validator bug and no testcase was excluded
from acceptance accounting.

## Historical report

# libuv-safe validator baseline (phase-08, phase-09)

## Validator source

- Repository: https://github.com/safelibs/validator
- Cloned commit: `87b321fe728340d6fc6dd2f638583cca82c667c3`
- Working tree: `validator/` (gitignored, not committed)

## Locally built libuv-safe packages

Read from `safe/dist/artifacts.env` after `safe/tools/build_deb.sh`:

| Package      | Version                          | Architecture | Filename                                                  |
| ------------ | -------------------------------- | ------------ | --------------------------------------------------------- |
| `libuv1t64`  | `1.48.0-1.1build1+safelibs1`     | `amd64`      | `libuv1t64_1.48.0-1.1build1+safelibs1_amd64.deb`          |
| `libuv1-dev` | `1.48.0-1.1build1+safelibs1`     | `amd64`      | `libuv1-dev_1.48.0-1.1build1+safelibs1_amd64.deb`         |

## Artifact roots

- Validator artifact root: `validator/artifacts/libuv-safe/phase-08`
- Local override root: `validator/artifacts/libuv-safe/phase-08/local-debs/libuv`
- Per-case logs: `validator/artifacts/libuv-safe/phase-08/logs/libuv/`
- Per-case results: `validator/artifacts/libuv-safe/phase-08/results/libuv/`
- Casts (`--record-casts`): `validator/artifacts/libuv-safe/phase-08/casts/libuv/`

## Commands run

```bash
# 1. (One-time) build the upstream baseline that safe ABI exporters consume.
cmake -S original -B original/build-checker \
  -DBUILD_TESTING=OFF -DCMAKE_BUILD_TYPE=RelWithDebInfo
cmake --build original/build-checker -j"$(nproc)"

# 2. Build the libuv-safe Debian artifacts.
bash safe/tools/build_deb.sh

# 3. Stage local override .debs in the validator override layout.
mkdir -p validator/artifacts/libuv-safe/phase-08/local-debs/libuv
cp safe/dist/libuv1t64_1.48.0-1.1build1+safelibs1_amd64.deb \
   validator/artifacts/libuv-safe/phase-08/local-debs/libuv/
cp safe/dist/libuv1-dev_1.48.0-1.1build1+safelibs1_amd64.deb \
   validator/artifacts/libuv-safe/phase-08/local-debs/libuv/

# 4. Validator unit tests + manifest checks.
( cd validator && python3 -m unittest discover -s unit -v )
( cd validator && python3 tools/testcases.py \
    --config repositories.yml --tests-root tests \
    --check --library libuv )

# 5. Strict matrix in original mode against locally built overrides.
( cd validator && bash test.sh \
    --config repositories.yml \
    --tests-root tests \
    --artifact-root artifacts/libuv-safe/phase-08 \
    --mode original \
    --override-deb-root artifacts/libuv-safe/phase-08/local-debs \
    --library libuv \
    --record-casts )
```

The matrix runs Ubuntu 24.04 containers via Docker; each container installs the
locally built override `.deb` packages from `/override-debs` before executing
the testcase, and the runner records this in each result via the
`override_debs_installed` field.

## Testcase counts

Cloned validator suite for libuv (counted from
`validator/tests/libuv/tests/cases/`):

| Kind   | Count | Required |
| ------ | ----- | -------- |
| source | 5     | ≥ 5      |
| usage  | 170   | ≥ 140    |
| total  | 175   | ≥ 145    |

All thresholds met.

## Override coverage

All 175 non-summary result JSONs under
`validator/artifacts/libuv-safe/phase-08/results/libuv/` carry
`"override_debs_installed": true`. Every libuv-safe baseline run was therefore
executed against the locally built `libuv1t64` and `libuv1-dev` packages, not
the upstream Ubuntu apt versions.

## Run summary

From `validator/artifacts/libuv-safe/phase-08/results/libuv/summary.json`:

| Field         | Value      |
| ------------- | ---------- |
| schema_version| 2          |
| library       | libuv      |
| mode          | original   |
| cases         | 175        |
| source_cases  | 5          |
| usage_cases   | 170        |
| passed        | 172        |
| failed        | 3          |
| casts         | 175        |

## Proof artifacts

This phase deliberately does not run
`validator/tools/verify_proof_artifacts.py --mode original` against the
override matrix. Original-mode proof is reserved for unmodified Ubuntu
packages; final libuv-safe proof is generated only in
`impl-13-validator-catchall-and-report` using the port-mode flow with the lock
emitted by `safe/tools/write_validator_port_lock.py`.

## Failures

All three failures are usage testcases driven by Node.js, and all three trip
on the same diagnostic mismatch: an `ENOENT`-class error path returns the
string `"Unknown system error -2"` instead of `"ENOENT"`. errno `-2` is the
correct numeric code, but `uv_err_name`/`uv_strerror` (or whatever path the
safe port uses to map errno to a libuv error name) is failing to map `-2` to
`UV_ENOENT` / `"ENOENT"`. This is a libuv-safe regression in the fs/error-name
mapping surface — not a validator bug. Per the phase mapping in the workflow,
filesystem-shaped failures are owned by
`impl-11-validator-fs-dns-process-fixes`.

| testcase_id                                       | kind  | owner_phase                              | status | artifact                                                                 | notes                                                                                                            |
| ------------------------------------------------- | ----- | ---------------------------------------- | ------ | ------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------- |
| usage-nodejs-fs-access-existing-and-missing       | usage | impl-11-validator-fs-dns-process-fixes   | failed | logs/libuv/usage-nodejs-fs-access-existing-and-missing.log               | `fs.access` on a missing path returns `code: 'Unknown system error -2'` instead of `'ENOENT'`.                   |
| usage-nodejs-fs-copyfile-unlink-chain             | usage | impl-11-validator-fs-dns-process-fixes   | failed | logs/libuv/usage-nodejs-fs-copyfile-unlink-chain.log                     | Post-`unlink` `fs.access` reports `code: 'Unknown system error -2'` instead of `'ENOENT'`.                       |
| usage-nodejs-fs-cp-recursive                      | usage | impl-11-validator-fs-dns-process-fixes   | failed | logs/libuv/usage-nodejs-fs-cp-recursive.log                              | `fs.promises.cp` raises `Error: Unknown system error -2 ... lstat '/tmp/validator-tmp/dst'` instead of `ENOENT`. |

### First relevant error line per failure

- `usage-nodejs-fs-access-existing-and-missing`:
  `AssertionError [ERR_ASSERTION]: Expected values to be strictly equal:` …
  `actual: 'Unknown system error -2'`, `expected: 'ENOENT'`.
- `usage-nodejs-fs-copyfile-unlink-chain`:
  `AssertionError [ERR_ASSERTION]: ... actual: 'Unknown system error -2', expected: 'ENOENT'`.
- `usage-nodejs-fs-cp-recursive`:
  `[Error: Unknown system error -2: Unknown error, lstat '/tmp/validator-tmp/dst']`.

## Suspected validator bugs

None. All three failures point at libuv-safe's error-name mapping, not the
validator suite. No skip policy is proposed.

## Triage table

| testcase_id                                   | kind  | owner_phase                              | status |
| --------------------------------------------- | ----- | ---------------------------------------- | ------ |
| usage-nodejs-fs-access-existing-and-missing   | usage | impl-11-validator-fs-dns-process-fixes   | open   |
| usage-nodejs-fs-copyfile-unlink-chain         | usage | impl-11-validator-fs-dns-process-fixes   | open   |
| usage-nodejs-fs-cp-recursive                  | usage | impl-11-validator-fs-dns-process-fixes   | open   |

Subsequent phases are expected to clear these by repairing libuv-safe's errno
→ libuv error-name translation. Updates to this report happen in those
phases.

## phase-09 — packaging, ABI, headers, and validator setup

Owner phase: `impl-09-validator-packaging-abi-fixes`.

### Failure ownership review

The phase-08 baseline triage assigned all three failures to
`impl-11-validator-fs-dns-process-fixes`. A re-review of the per-case logs
under `validator/artifacts/libuv-safe/phase-08/logs/libuv/` confirms that none
of the failures match the categories owned by this phase:

- No `dpkg` install failures for `libuv1t64` or `libuv1-dev` — every failing
  log shows `Setting up libuv1t64:amd64 (1.48.0-1.1build1+safelibs1) ...` and
  `Setting up libuv1-dev:amd64 (1.48.0-1.1build1+safelibs1) ...` succeeding
  before the testcase runs.
- No missing `uv.h` / `uv/*.h` headers and no `gcc ... -luv` link failures.
- No `pkg-config` / `libuv.pc` metadata failures.
- No reports of `nodejs` or validator C probes loading Ubuntu's stock libuv
  instead of the local override packages — `override_debs_installed: true`
  for all 175 result JSONs.
- No missing or mismatched exported symbols.

All three failing testcases are Node.js-driven `fs.*` probes that fail on the
`errno -2 → "ENOENT"` mapping, which is owned by
`impl-11-validator-fs-dns-process-fixes`. No source fixes, packaging changes,
ABI changes, or new regression probes are added in this phase.

### Commands run

```bash
# 1. Stage the same locally built override .debs in the phase-09 layout.
mkdir -p validator/artifacts/libuv-safe/phase-09/local-debs/libuv
cp safe/dist/libuv1t64_1.48.0-1.1build1+safelibs1_amd64.deb \
   validator/artifacts/libuv-safe/phase-09/local-debs/libuv/
cp safe/dist/libuv1-dev_1.48.0-1.1build1+safelibs1_amd64.deb \
   validator/artifacts/libuv-safe/phase-09/local-debs/libuv/

# 2. Strict matrix in original mode against locally built overrides.
( cd validator && bash test.sh \
    --config repositories.yml \
    --tests-root tests \
    --artifact-root artifacts/libuv-safe/phase-09 \
    --mode original \
    --override-deb-root artifacts/libuv-safe/phase-09/local-debs \
    --library libuv \
    --record-casts )
```

### phase-09 run summary

From `validator/artifacts/libuv-safe/phase-09/results/libuv/summary.json`:

| Field         | Value      |
| ------------- | ---------- |
| schema_version| 2          |
| library       | libuv      |
| mode          | original   |
| cases         | 175        |
| source_cases  | 5          |
| usage_cases   | 170        |
| passed        | 172        |
| failed        | 3          |
| casts         | 175        |

All 175 non-summary result JSONs under
`validator/artifacts/libuv-safe/phase-09/results/libuv/` carry
`"override_debs_installed": true`. No testcase escaped the override matrix.

### phase-09 failures

Identical set to phase-08; same owner phase, same root cause. No regression
files added under `safe/tests/regressions/` in this phase, and
`safe/tests/regressions/manifest.json` is unchanged.

| testcase_id                                   | kind  | owner_phase                              | status | regression_file | changed_sources |
| --------------------------------------------- | ----- | ---------------------------------------- | ------ | --------------- | --------------- |
| usage-nodejs-fs-access-existing-and-missing   | usage | impl-11-validator-fs-dns-process-fixes   | open   | (none — owned by phase-11) | (none in phase-09) |
| usage-nodejs-fs-copyfile-unlink-chain         | usage | impl-11-validator-fs-dns-process-fixes   | open   | (none — owned by phase-11) | (none in phase-09) |
| usage-nodejs-fs-cp-recursive                  | usage | impl-11-validator-fs-dns-process-fixes   | open   | (none — owned by phase-11) | (none in phase-09) |

### Files changed in phase-09

- `validator-report.md` (this section).
- No source, packaging, build, or regression files modified.

### Remaining failures after phase-09

| testcase_id                                   | kind  | owner_phase                              | status |
| --------------------------------------------- | ----- | ---------------------------------------- | ------ |
| usage-nodejs-fs-access-existing-and-missing   | usage | impl-11-validator-fs-dns-process-fixes   | open   |
| usage-nodejs-fs-copyfile-unlink-chain         | usage | impl-11-validator-fs-dns-process-fixes   | open   |
| usage-nodejs-fs-cp-recursive                  | usage | impl-11-validator-fs-dns-process-fixes   | open   |

## phase-10 — core loop, timer, async, threading, and random fixes

Owner phase: `impl-10-validator-core-loop-threading-fixes`.

### Failure ownership review

A re-review of `validator/artifacts/libuv-safe/phase-09/results/libuv/` against
this phase's scope (timers, event loop, async, threadpool, random, worker
scheduling) confirms that none of the open failures are owned by phase-10:

- `event-loop-timer.sh` (the cloned validator's only timer-shaped source case)
  passes both at phase-08 and phase-09.
- No usage testcase fails on `setTimeout`/`setInterval`/`setImmediate`,
  microtask/timer ordering, `worker_threads`, `crypto.randomBytes`,
  `randomFill`, `randomUUID`, `scrypt`, or any other surface routed through
  `uv_timer_*`, `uv_async_*`, `uv_queue_work`/`uv_cancel`, or `uv_random`.
- All three remaining failures are `fs.*`-shaped and continue to fault on the
  `errno -2 → "ENOENT"` mapping, owned by
  `impl-11-validator-fs-dns-process-fixes`.

No source defect lands in this phase. Per the phase plan, minimal C
regressions are still added so the timer/threadpool/random surfaces are
locked behind direct ABI probes that will fail loudly under any future
regression in `safe/src/core/timer.rs`, `safe/src/threading/threadpool.rs`,
or `safe/src/threading/random.rs`.

### New regressions added

Each is registered in `safe/tests/regressions/manifest.json` with
`phase_owner: "impl-10-validator-core-loop-threading-fixes"`.

| regression_id                       | path                                       | exercises |
| ----------------------------------- | ------------------------------------------ | --------- |
| `validator_timer_ordering_close`    | `validator_timer_ordering_close.c`         | `uv_timer_init`, `uv_timer_start` (one-shot + repeat), `uv_timer_stop` (pre-run and inside callback), `uv_timer_get_repeat`, `uv_close` ordering, `UV_RUN_DEFAULT` drain, `uv_loop_alive`/`uv_loop_close`. |
| `validator_threadpool_queue_work`   | `validator_threadpool_queue_work.c`        | `uv_queue_work` batch, `uv_cancel` mixed with in-flight work, exactly-one after-work callback per request, `UV_ECANCELED` reporting, loop drain afterwards. |
| `validator_random_sync_async`       | `validator_random_sync_async.c`            | Synchronous `uv_random` (loop=NULL, cb=NULL), async `uv_random` with completion callback on the owning loop, rejection of unknown flag bits with `UV_EINVAL`. |

All three pass under `safe/tools/run_regressions.sh --up-to-phase
impl-10-validator-core-loop-threading-fixes` against the staged install at
`/tmp/libuv-safe-validator-stage`.

### Commands run

```bash
# 1. Build the safe library and stage it for the regression sweep.
cargo build --manifest-path safe/Cargo.toml --release
bash safe/tools/stage_install.sh /tmp/libuv-safe-validator-stage
bash safe/tools/verify_stage_install.sh /tmp/libuv-safe-validator-stage

# 2. Run regression probes up to and including this phase.
bash safe/tools/run_regressions.sh \
  --stage /tmp/libuv-safe-validator-stage \
  --up-to-phase impl-10-validator-core-loop-threading-fixes

# 3. Rebuild Debian packages from the staged sources.
bash safe/tools/build_deb.sh

# 4. Stage the same locally built override .debs in the phase-10 layout.
mkdir -p validator/artifacts/libuv-safe/phase-10/local-debs/libuv
cp safe/dist/libuv1t64_1.48.0-1.1build1+safelibs1_amd64.deb \
   validator/artifacts/libuv-safe/phase-10/local-debs/libuv/
cp safe/dist/libuv1-dev_1.48.0-1.1build1+safelibs1_amd64.deb \
   validator/artifacts/libuv-safe/phase-10/local-debs/libuv/

# 5. Strict matrix in original mode against locally built overrides.
( cd validator && bash test.sh \
    --config repositories.yml \
    --tests-root tests \
    --artifact-root artifacts/libuv-safe/phase-10 \
    --mode original \
    --override-deb-root artifacts/libuv-safe/phase-10/local-debs \
    --library libuv \
    --record-casts )
```

### phase-10 run summary

From `validator/artifacts/libuv-safe/phase-10/results/libuv/summary.json`:

| Field         | Value      |
| ------------- | ---------- |
| schema_version| 2          |
| library       | libuv      |
| mode          | original   |
| cases         | 175        |
| source_cases  | 5          |
| usage_cases   | 170        |
| passed        | 172        |
| failed        | 3          |
| casts         | 175        |

All 175 non-summary result JSONs under
`validator/artifacts/libuv-safe/phase-10/results/libuv/` carry
`"override_debs_installed": true`. No testcase escaped the override matrix.

### phase-10 failures

Identical set to phase-08 / phase-09; same fs/error-name owner phase. No
phase-10 regression matches a current validator failure.

| testcase_id                                   | kind  | owner_phase                              | status | regression_file | changed_sources |
| --------------------------------------------- | ----- | ---------------------------------------- | ------ | --------------- | --------------- |
| usage-nodejs-fs-access-existing-and-missing   | usage | impl-11-validator-fs-dns-process-fixes   | open   | (none — owned by phase-11) | (none in phase-10) |
| usage-nodejs-fs-copyfile-unlink-chain         | usage | impl-11-validator-fs-dns-process-fixes   | open   | (none — owned by phase-11) | (none in phase-10) |
| usage-nodejs-fs-cp-recursive                  | usage | impl-11-validator-fs-dns-process-fixes   | open   | (none — owned by phase-11) | (none in phase-10) |

### Files changed in phase-10

- `safe/tests/regressions/validator_timer_ordering_close.c` (new).
- `safe/tests/regressions/validator_threadpool_queue_work.c` (new).
- `safe/tests/regressions/validator_random_sync_async.c` (new).
- `safe/tests/regressions/manifest.json` (registers the three new probes).
- `validator-report.md` (this section).
- No source files under `safe/src/core/`, `safe/src/threading/`, or
  `safe/src/unix/` modified — no phase-10-owned validator failure required a
  source fix.

### Remaining failures after phase-10

| testcase_id                                   | kind  | owner_phase                              | status |
| --------------------------------------------- | ----- | ---------------------------------------- | ------ |
| usage-nodejs-fs-access-existing-and-missing   | usage | impl-11-validator-fs-dns-process-fixes   | open   |
| usage-nodejs-fs-copyfile-unlink-chain         | usage | impl-11-validator-fs-dns-process-fixes   | open   |
| usage-nodejs-fs-cp-recursive                  | usage | impl-11-validator-fs-dns-process-fixes   | open   |

## phase-11 — filesystem, DNS, and process fixes

Owner phase: `impl-11-validator-fs-dns-process-fixes`.

### Failure ownership and root cause

All three open failures from phases 08–10 are filesystem-shaped and route
through the `errno → libuv error name` translation surface, which lives in
`safe/src/core/error.rs` and is reached from the exported
`uv_err_name`, `uv_err_name_r`, `uv_strerror`, and `uv_strerror_r` symbols
in `safe/src/exports/generated.rs`. Node.js' `fs.*` error formatter calls
`uv_err_name(err)` on libuv-returned codes; when the table only carries a
handful of entries, every other code surfaces as `Unknown system error <n>`
and the `code` property leaks through as that string instead of `'ENOENT'`,
`'EACCES'`, etc.

The defect: `safe/src/core/error.rs` only enumerated 11 `UV_E*` codes plus
`UV__EOF`. The upstream `UV_ERRNO_MAP` carries 84 entries — including
`UV_ENOENT` (`-2`) — and a synchronous `uv_fs_access` on a missing path
returns exactly `-2`, so the unknown-code path was always taken for the
three failing testcases.

### Source fix

`safe/src/core/error.rs`:

- Backfilled the `ENTRIES` table with every entry from upstream's
  `UV_ERRNO_MAP` (including `ENOENT`, `EACCES`, `EEXIST`, `ENOTDIR`,
  `EISDIR`, `ELOOP`, `ENAMETOOLONG`, `EXDEV`, the `EAI_*` resolver codes,
  `UNKNOWN`, and the late additions `ENXIO`, `EMLINK`, `EHOSTDOWN`,
  `EREMOTEIO`, `ENOTTY`, `EFTYPE`, `EILSEQ`, `ESOCKTNOSUPPORT`, `ENODATA`,
  `EUNATCH`).
- Aligned the unknown-code formatting between `uv_err_name`,
  `uv_err_name_r`, `uv_strerror`, and `uv_strerror_r`: all four now emit
  `Unknown system error %d` with the raw (unmodified) error code, matching
  upstream's `uv__unknown_err_code` and the `default:` arms of
  `uv_err_name_r` / `uv_strerror_r` in `original/src/uv-common.c`.
- Removed the previous inconsistency where `strerror` canonicalised the
  code before formatting and emitted `Unknown error` with no numeric
  suffix.

No async dispatch, request layout, or `uv_fs_req_cleanup` behaviour was
touched — the defect was confined to the static name/message table and the
unknown-code formatter.

### New regression added

| regression_id                       | path                                       | exercises |
| ----------------------------------- | ------------------------------------------ | --------- |
| `validator_fs_enoent_error_names`   | `validator_fs_enoent_error_names.c`        | Synchronous `uv_fs_access` on a missing path returns `UV_ENOENT` and populates `req.result`; second `uv_fs_access` reuses the same stack-allocated request after `uv_fs_req_cleanup`; `uv_err_name` / `uv_err_name_r` map `UV_ENOENT`, `UV_EACCES`, `UV_EEXIST`, `UV_ENOTDIR`, `UV_EISDIR`, `UV_ELOOP`, `UV_ENAMETOOLONG`, `UV_ENOTEMPTY`, `UV_EBADF`, `UV_EXDEV`, `UV_EAI_NONAME`; `uv_strerror` / `uv_strerror_r` carry the upstream messages for the same codes; unknown codes (`-99999`) format as `Unknown system error -99999`. |

Registered in `safe/tests/regressions/manifest.json` with
`phase_owner: "impl-11-validator-fs-dns-process-fixes"`.

The probe passes under `safe/tools/run_regressions.sh --up-to-phase
impl-11-validator-fs-dns-process-fixes` against the staged install at
`/tmp/libuv-safe-validator-stage`. (Without the source fix, it fails on
`uv_err_name(UV_ENOENT) = Unknown system error -2; want ENOENT`.)

### Commands run

```bash
# 1. Build the safe library and stage it for the regression sweep.
cargo build --manifest-path safe/Cargo.toml --release
bash safe/tools/stage_install.sh /tmp/libuv-safe-validator-stage
bash safe/tools/verify_stage_install.sh /tmp/libuv-safe-validator-stage

# 2. Run regression probes up to and including this phase.
bash safe/tools/run_regressions.sh \
  --stage /tmp/libuv-safe-validator-stage \
  --up-to-phase impl-11-validator-fs-dns-process-fixes

# 3. Rebuild Debian packages from the patched sources.
bash safe/tools/build_deb.sh

# 4. Stage the rebuilt override .debs in the phase-11 layout.
mkdir -p validator/artifacts/libuv-safe/phase-11/local-debs/libuv
cp safe/dist/libuv1t64_1.48.0-1.1build1+safelibs1_amd64.deb \
   validator/artifacts/libuv-safe/phase-11/local-debs/libuv/
cp safe/dist/libuv1-dev_1.48.0-1.1build1+safelibs1_amd64.deb \
   validator/artifacts/libuv-safe/phase-11/local-debs/libuv/

# 5. Strict matrix in original mode against locally built overrides.
( cd validator && bash test.sh \
    --config repositories.yml \
    --tests-root tests \
    --artifact-root artifacts/libuv-safe/phase-11 \
    --mode original \
    --override-deb-root artifacts/libuv-safe/phase-11/local-debs \
    --library libuv \
    --record-casts )
```

### phase-11 run summary

From `validator/artifacts/libuv-safe/phase-11/results/libuv/summary.json`:

| Field         | Value      |
| ------------- | ---------- |
| schema_version| 2          |
| library       | libuv      |
| mode          | original   |
| cases         | 175        |
| source_cases  | 5          |
| usage_cases   | 170        |
| passed        | 175        |
| failed        | 0          |
| casts         | 175        |

All 175 non-summary result JSONs under
`validator/artifacts/libuv-safe/phase-11/results/libuv/` carry
`"override_debs_installed": true`. No testcase escaped the override matrix.

### phase-11 closures

| testcase_id                                   | kind  | owner_phase                              | status | regression_file                       | changed_sources              |
| --------------------------------------------- | ----- | ---------------------------------------- | ------ | ------------------------------------- | ---------------------------- |
| usage-nodejs-fs-access-existing-and-missing   | usage | impl-11-validator-fs-dns-process-fixes   | passed | validator_fs_enoent_error_names.c     | safe/src/core/error.rs       |
| usage-nodejs-fs-copyfile-unlink-chain         | usage | impl-11-validator-fs-dns-process-fixes   | passed | validator_fs_enoent_error_names.c     | safe/src/core/error.rs       |
| usage-nodejs-fs-cp-recursive                  | usage | impl-11-validator-fs-dns-process-fixes   | passed | validator_fs_enoent_error_names.c     | safe/src/core/error.rs       |

### Files changed in phase-11

- `safe/src/core/error.rs` (full UV_ERRNO_MAP table + unified unknown-code
  formatter).
- `safe/tests/regressions/validator_fs_enoent_error_names.c` (new probe).
- `safe/tests/regressions/manifest.json` (registers the new probe).
- `validator-report.md` (this section).

No filesystem, DNS, or process source files under `safe/src/unix/`
required a fix in this phase — none of the open validator failures matched
those surfaces. The mapping table was the sole defect.

### Remaining failures after phase-11

None. All three open failures from the phase-08 baseline are now passing
under the strict original-mode override matrix.

## phase-12 — network, pipe, stream, poll, TCP, and UDP fixes

Owner phase: `impl-12-validator-network-io-fixes`.

### Failure ownership review

The phase-11 rerun closed the last open failure from the phase-08 baseline,
so phase-12 inherits a clean validator matrix. A re-review of every result
JSON under `validator/artifacts/libuv-safe/phase-11/results/libuv/` against
this phase's scope (TCP/UDP/pipe/poll/stream loopback, HTTP/HTTP2 loopback,
`net.*`, `dgram.*`, streams/pipeline) confirms there is nothing for
phase-12 to repair:

- The two source cases this phase owns end-to-end — `tcp-loopback-smoke`
  and `process-pipe-smoke` — are both `passed` with
  `override_debs_installed: true`.
- Every Node.js usage testcase under `net.*`, `dgram.*`, HTTP/HTTP2
  loopback, and streams/pipeline shapes (e.g. `usage-nodejs-net-tcp-*`,
  `usage-nodejs-dgram-*`, `usage-nodejs-http-*`, `usage-nodejs-http2-*`,
  `usage-nodejs-stream-*`) is `passed`.
- No phase-12-shaped failure was masked by a fix in an earlier phase: the
  three closed failures all routed through `safe/src/core/error.rs` and
  did not touch the network/stream surfaces under `safe/src/unix/`.

No source defect lands in this phase. Per the phase plan, minimal C
regressions are still added so the TCP, pipe/stream, and UDP surfaces are
locked behind direct ABI probes that will fail loudly under any future
regression in `safe/src/unix/tcp.rs`, `safe/src/unix/pipe.rs`,
`safe/src/unix/stream.rs`, or `safe/src/unix/udp.rs`.

### New regressions added

Each is registered in `safe/tests/regressions/manifest.json` with
`phase_owner: "impl-12-validator-network-io-fixes"`.

| regression_id                          | path                                       | exercises |
| -------------------------------------- | ------------------------------------------ | --------- |
| `validator_tcp_loopback_echo`          | `validator_tcp_loopback_echo.c`            | `uv_tcp_init`/`uv_tcp_bind` on `127.0.0.1:0`, `uv_listen` invokes connection_cb exactly once, `uv_accept` transfers the kernel socket, `uv_tcp_getsockname`/`uv_tcp_getpeername` after bind/connect/accept return AF_INET, `uv_read_start` delivers payload + a `UV_EOF` nread on shutdown, `uv_write` completes once with status==0 and `uv_stream_get_write_queue_size` drains to 0, `uv_shutdown` cb fires once, three `uv_close` calls each fire one close_cb. |
| `validator_pipe_socketpair_stream`     | `validator_pipe_socketpair_stream.c`       | `uv_pipe_init` + `uv_pipe_open` adopt both ends of a `socketpair(AF_UNIX, SOCK_STREAM)`, `uv_try_write` succeeds on a fresh stream, `uv_write` completes once with `write_queue_size`==0 after drain, `uv_shutdown` cb fires once, the reader sees the concatenated `try:` prefix + payload then a `UV_EOF` nread, two `uv_close` calls each fire one close_cb. |
| `validator_udp_loopback_send_recv`     | `validator_udp_loopback_send_recv.c`       | `uv_udp_init`/`uv_udp_bind` on `127.0.0.1:0`, `uv_udp_getsockname` returns AF_INET with a non-zero port, `uv_udp_set_broadcast`/`uv_udp_set_ttl`/`uv_udp_set_multicast_ttl`/`uv_udp_set_multicast_loop` succeed on a bound loopback socket, `uv_udp_recv_start` delivers two datagrams with populated AF_INET addr, `uv_udp_send` (unconnected, with explicit addr) and `uv_udp_send` (connected, NULL addr) each invoke send_cb once with status==0, `uv_udp_connect` + `uv_udp_getpeername` round-trip the peer port, two `uv_close` calls each fire one close_cb. |

All three pass under `safe/tools/run_regressions.sh --up-to-phase
impl-12-validator-network-io-fixes` against the staged install at
`/tmp/libuv-safe-validator-stage`.

### Commands run

```bash
# 1. Build the safe library and stage it for the regression sweep.
cargo build --manifest-path safe/Cargo.toml --release
bash safe/tools/stage_install.sh /tmp/libuv-safe-validator-stage
bash safe/tools/verify_stage_install.sh /tmp/libuv-safe-validator-stage

# 2. Run regression probes up to and including this phase.
bash safe/tools/run_regressions.sh \
  --stage /tmp/libuv-safe-validator-stage \
  --up-to-phase impl-12-validator-network-io-fixes

# 3. Rebuild Debian packages (no source defect; rebuild for parity).
bash safe/tools/build_deb.sh

# 4. Stage the rebuilt override .debs in the phase-12 layout.
mkdir -p validator/artifacts/libuv-safe/phase-12/local-debs/libuv
cp safe/dist/libuv1t64_1.48.0-1.1build1+safelibs1_amd64.deb \
   validator/artifacts/libuv-safe/phase-12/local-debs/libuv/
cp safe/dist/libuv1-dev_1.48.0-1.1build1+safelibs1_amd64.deb \
   validator/artifacts/libuv-safe/phase-12/local-debs/libuv/

# 5. Strict matrix in original mode against locally built overrides.
( cd validator && bash test.sh \
    --config repositories.yml \
    --tests-root tests \
    --artifact-root artifacts/libuv-safe/phase-12 \
    --mode original \
    --override-deb-root artifacts/libuv-safe/phase-12/local-debs \
    --library libuv \
    --record-casts )
```

### phase-12 run summary

From `validator/artifacts/libuv-safe/phase-12/results/libuv/summary.json`:

| Field         | Value      |
| ------------- | ---------- |
| schema_version| 2          |
| library       | libuv      |
| mode          | original   |
| cases         | 175        |
| source_cases  | 5          |
| usage_cases   | 170        |
| passed        | 175        |
| failed        | 0          |
| casts         | 175        |

All 175 non-summary result JSONs under
`validator/artifacts/libuv-safe/phase-12/results/libuv/` carry
`"override_debs_installed": true`. No testcase escaped the override
matrix.

### phase-12 closures

No phase-12-owned validator failure was open at the start of this phase,
so there is no closure table. The new regressions stand as guard probes
against future regressions in the TCP/pipe/stream/UDP surfaces.

### Files changed in phase-12

- `safe/tests/regressions/validator_tcp_loopback_echo.c` (new probe).
- `safe/tests/regressions/validator_pipe_socketpair_stream.c` (new probe).
- `safe/tests/regressions/validator_udp_loopback_send_recv.c` (new probe).
- `safe/tests/regressions/manifest.json` (registers the three new probes).
- `validator-report.md` (this section).

No source files under `safe/src/unix/tcp.rs`, `pipe.rs`, `stream.rs`,
`udp.rs`, `poll.rs`, `fd.rs`, or `epoll.rs` required a fix — every
network/IO-shaped validator testcase was already passing under the
phase-11 rerun.

### Remaining failures after phase-12

None.

## phase-13 — catch-all, final clean port-mode run, and final report

Owner phase: `impl-13-validator-catchall-and-report`.

### Validator source

- Repository: https://github.com/safelibs/validator
- Validator commit (cloned, pinned): `87b321fe728340d6fc6dd2f638583cca82c667c3`
- Working tree: `validator/` (gitignored, not committed; tracked files unmodified by this phase).

### Safe commit range

Workflow-run base (from `.plan/workflow-run-base.txt`):
`b41b6df2e66ec9ab6e33392a7f806ac7ac555760`.

Phase commits applied since the base, in `--first-parent` order
(this commit is added at the end of phase-13):

| Commit  | Subject                                                                          |
| ------- | -------------------------------------------------------------------------------- |
| d3e37ea | impl-08-validator-bootstrap-and-baseline advance workflow-run base past impl-09 head |
| d20f259 | impl-09-validator-packaging-abi-fixes re-affirm clean phase-09 packaging/ABI rerun |
| 5cb15bd | impl-10-validator-core-loop-threading-fixes add core-loop/threadpool/random regressions and phase-10 rerun |
| 71757af | impl-11-validator-fs-dns-process-fixes backfill UV_ERRNO_MAP for fs error names  |
| d96000a | impl-12-validator-network-io-fixes add tcp/pipe/udp regressions and phase-12 rerun |
| _HEAD_  | impl-13-validator-catchall-and-report final port-mode run and report             |

### Failure ownership review

The phase-12 rerun shipped a clean strict original-mode override matrix
(`validator/artifacts/libuv-safe/phase-12/results/libuv/summary.json`:
`cases=175, source_cases=5, usage_cases=170, passed=175, failed=0,
casts=175`, with `override_debs_installed: true` for every per-case
result). Reparsing every result JSON under
`validator/artifacts/libuv-safe/phase-12/results/libuv/` confirms there is
no remaining failure for phase-13 to triage. Therefore:

- No new `phase_owner: "impl-13-validator-catchall-and-report"` regression
  is required.
- No `safe/src/**`, `safe/debian/**`, or `safe/tools/**` source fix is
  required.
- `safe/tests/regressions/manifest.json` is unchanged in this phase.

### Suspected validator bugs

None as a libuv testcase failure. Every libuv testcase passes against
the locally built libuv-safe `.deb` packages, both in strict
original-mode (phase-11/phase-12 reruns) and in the final port-mode run
below. No testcase skip is required and none is applied — this is a
no-skip clean final state.

### Validator-bug-shaped failure (port-deb-lock release-tag tokenisation)

A phase-13 first attempt produced a clean validator matrix (175/175
passed in port mode) but failed `validator/scripts/verify-site.sh` with
`rendered HTML contains final user-facing safe/unsafe language`. Cause:
`validator/scripts/verify-site.sh` strips `<details class="case-row">`
blocks and then rejects any rendered HTML matching
`\bsafe\b|\bunsafe\b|safe[- ]workload`. The check-25 verification
command in the phase plan
(`.plan/phases/06-validator-catchall-and-report.md`) feeds the lock
writer `--release-tag "local-libuv-safe-$(git rev-parse --short HEAD)"`,
which embeds a `\bsafe\b` token in the rendered release-URL that the
template emits outside any case-row block. `-` is a non-word character
so word boundaries form on either side of `safe`, and the rejection
regex matches.

Disposition: this is not a libuv-safe runtime failure and not
straightforwardly a validator bug either — verify-site.sh is correct to
forbid that user-facing language outside the documented case-row scope.
Modifying any tracked validator file is forbidden by the phase rules
and would not be the right layer in any case.

Fix scope (single, minimal): `safe/tools/write_validator_port_lock.py`
sanitizes the supplied `--release-tag` (and the derived `tag_ref`) by
replacing every `\bsafe\b` / `\bunsafe\b` token with `port` before
storing the lock. The `commit` field is hex-only and never matches the
regex, so it is left as-is. With the sanitization in place the documented
check-25 command produces a lock whose `release_tag` is
`local-libuv-port-<short>` (no `\bsafe\b`), the proof carries the
sanitized value, the rendered site contains only the sanitized token,
and `verify-site.sh` exits 0.

No skip is applied; no validator file is modified; no testcase is
disabled. The single regression
`safe/tests/regressions/validator_port_lock_safe_token_sanitized.sh`
pins the writer's behaviour against future drift, including the negative
case (a clean tag like `v1.48.0+safelibs1` must pass through unchanged).

### Baseline Failures found (phase-08) and final disposition

| testcase_id                                       | kind  | owner_phase                              | final status | regression_file                       | changed_sources              |
| ------------------------------------------------- | ----- | ---------------------------------------- | ------------ | ------------------------------------- | ---------------------------- |
| usage-nodejs-fs-access-existing-and-missing       | usage | impl-11-validator-fs-dns-process-fixes   | passed       | validator_fs_enoent_error_names.c     | safe/src/core/error.rs       |
| usage-nodejs-fs-copyfile-unlink-chain             | usage | impl-11-validator-fs-dns-process-fixes   | passed       | validator_fs_enoent_error_names.c     | safe/src/core/error.rs       |
| usage-nodejs-fs-cp-recursive                      | usage | impl-11-validator-fs-dns-process-fixes   | passed       | validator_fs_enoent_error_names.c     | safe/src/core/error.rs       |

### Fixes applied by phase

| phase | regression files added                                                                                                                                                                       | source files changed   |
| ----- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| impl-08-validator-bootstrap-and-baseline | (baseline only — no regressions, no source changes)                                                                                                                                          | (none)                 |
| impl-09-validator-packaging-abi-fixes    | (none — no phase-09-owned failure)                                                                                                                                                            | (none)                 |
| impl-10-validator-core-loop-threading-fixes | `validator_timer_ordering_close.c`, `validator_threadpool_queue_work.c`, `validator_random_sync_async.c`                                                                                  | (none)                 |
| impl-11-validator-fs-dns-process-fixes   | `validator_fs_enoent_error_names.c`                                                                                                                                                          | `safe/src/core/error.rs` (full `UV_ERRNO_MAP` table + unified unknown-code formatter) |
| impl-12-validator-network-io-fixes       | `validator_tcp_loopback_echo.c`, `validator_pipe_socketpair_stream.c`, `validator_udp_loopback_send_recv.c`                                                                                  | (none)                 |
| impl-13-validator-catchall-and-report    | `validator_port_lock_safe_token_sanitized.sh`                                                                                                                                                 | `safe/tools/write_validator_port_lock.py` (sanitize `\bsafe\b` / `\bunsafe\b` in the supplied `--release-tag` and the derived `tag_ref`, see “Validator-bug-shaped failure” below) |

### Final port-mode artifact roots and proof / lock / site paths

| Path role                       | Path                                                                                                            |
| ------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| Final artifact root             | `validator/artifacts/libuv-safe/check-25-final`                                                                 |
| Final per-case results          | `validator/artifacts/libuv-safe/check-25-final/port/results/libuv/`                                             |
| Final per-case logs             | `validator/artifacts/libuv-safe/check-25-final/port/logs/libuv/`                                                |
| Final casts (`--record-casts`)  | `validator/artifacts/libuv-safe/check-25-final/port/casts/libuv/`                                               |
| Local override-deb root         | `validator/artifacts/libuv-safe/check-25-final/local-debs/libuv/`                                               |
| Final port-deb lock             | `validator/artifacts/libuv-safe/check-25-final/proof/local-port-debs-lock.json`                                 |
| Final port-mode proof           | `validator/artifacts/libuv-safe/check-25-final/proof/libuv-safe-validation-proof.json`                          |
| Final rendered site             | `validator/site/libuv-safe-final/`                                                                              |

None of these paths is committed to git — they are regenerated by the
phase-13 commands below.

### Final port-deb lock contents

The lock at `validator/artifacts/libuv-safe/check-25-final/proof/local-port-debs-lock.json`
embeds:

| Field        | Value                                                  |
| ------------ | ------------------------------------------------------ |
| schema_version | `1`                                                  |
| mode         | `port`                                                 |
| repository   | `safelibs/port-libuv`                                  |
| url          | `https://github.com/safelibs/port-libuv`               |
| commit       | `git rev-parse HEAD` of `port-libuv` at lock-write time |
| release_tag  | `local-libuv-port-<short-commit>` (lock writer rewrites the supplied `local-libuv-safe-<short-commit>` token; see “Validator-bug-shaped failure” below) |
| tag_ref      | `refs/tags/<release_tag>` (matches the rewritten release_tag) |
| debs[0]      | `libuv1t64` `1.48.0-1.1build1+safelibs1` `amd64`       |
| debs[1]      | `libuv1-dev` `1.48.0-1.1build1+safelibs1` `amd64`      |
| unported_original_packages | `[]`                                     |

The commit value is the HEAD of the `port-libuv` working tree at the
moment phase-13 wrote the lock. The release-tag is a deterministic
local synthetic tag because the local libuv-safe build is not (yet) cut
as a real upstream release tag in the `safelibs/port-libuv` GitHub
repository. The check-25 verification command in the phase plan
(`.plan/phases/06-validator-catchall-and-report.md`) feeds the writer
the literal input `local-libuv-safe-$(git rev-parse --short HEAD)`; the
writer rewrites the `\bsafe\b` token to `port` before storing the value
so the rendered site does not trip
`validator/scripts/verify-site.sh`'s `\bsafe\b|\bunsafe\b` rejection
regex (see “Validator-bug-shaped failure” below).

Concrete values from the most recent successful phase-13 run on the
parent commit `29138fc1e14b596c1a6ad4b47251761c3c940959`:

- `commit`: `29138fc1e14b596c1a6ad4b47251761c3c940959`
- `release_tag`: `local-libuv-port-29138fc` (writer input was
  `local-libuv-safe-29138fc`)
- `tag_ref`: `refs/tags/local-libuv-port-29138fc`

The rendered site under
`validator/site/libuv-safe-final/library/libuv.html` carries only the
sanitized `local-libuv-port-29138fc` token — `grep -oE
"local-libuv-(safe|port)-[a-f0-9]+"` returns exactly
`local-libuv-port-29138fc` and never the input form.

### Checks executed (commands run)

This is the full ordered list of phase-13 Checks executed end-to-end —
the same commands that the check-25 / check-26 / check-27 verifiers
re-execute. Every command below exits 0 against the current HEAD.

```bash
REPO=/home/yans/safelibs/pipeline/ports/port-libuv

# 0. Verify HEAD belongs to this phase (will pass after the phase-13 commit lands).
"$REPO/safe/tools/verify_phase_head.sh" impl-13-validator-catchall-and-report
docker version >/dev/null

# 1. Ensure the upstream baseline build is present (skipped if libuv.so.1.0.0 exists).
if [ ! -f "$REPO/original/build-checker/libuv.so.1.0.0" ]; then
  cmake -S "$REPO/original" -B "$REPO/original/build-checker" \
    -DBUILD_TESTING=OFF -DCMAKE_BUILD_TYPE=RelWithDebInfo
  cmake --build "$REPO/original/build-checker" -j"$(nproc)"
fi

# 2. Rebuild the libuv-safe Debian artifacts.
"$REPO/safe/tools/build_deb.sh"

# 3. Stage local override .debs in the final layout and write the port-deb lock.
ARTIFACT_ROOT="$REPO/validator/artifacts/libuv-safe/check-25-final"
OVERRIDE_ROOT="$ARTIFACT_ROOT/local-debs"
PORT_LOCK="$ARTIFACT_ROOT/proof/local-port-debs-lock.json"
SITE_ROOT="$REPO/validator/site/libuv-safe-final"
rm -rf "$ARTIFACT_ROOT" "$SITE_ROOT"
mkdir -p "$OVERRIDE_ROOT/libuv" "$ARTIFACT_ROOT/proof"
cp "$REPO"/safe/dist/libuv1t64_*.deb   "$OVERRIDE_ROOT/libuv/"
cp "$REPO"/safe/dist/libuv1-dev_*.deb  "$OVERRIDE_ROOT/libuv/"

python3 "$REPO/safe/tools/write_validator_port_lock.py" \
  --artifacts-env "$REPO/safe/dist/artifacts.env" \
  --override-root "$OVERRIDE_ROOT" \
  --lock-output "$PORT_LOCK" \
  --library libuv \
  --repository safelibs/port-libuv \
  --commit       "$(git -C "$REPO" rev-parse HEAD)" \
  --release-tag  "local-libuv-safe-$(git -C "$REPO" rev-parse --short HEAD)"

# 4. Validator unit tests + manifest checks.
make -C "$REPO/validator" unit
make -C "$REPO/validator" check-testcases

# 5. Final clean port-mode validator run.
bash "$REPO/validator/test.sh" \
  --config            "$REPO/validator/repositories.yml" \
  --tests-root        "$REPO/validator/tests" \
  --artifact-root     "$ARTIFACT_ROOT" \
  --mode              port \
  --port-deb-lock     "$PORT_LOCK" \
  --override-deb-root "$OVERRIDE_ROOT" \
  --library           libuv \
  --record-casts

# 6. Verify port-mode proof artifacts.
python3 "$REPO/validator/tools/verify_proof_artifacts.py" \
  --config         "$REPO/validator/repositories.yml" \
  --tests-root     "$REPO/validator/tests" \
  --artifact-root  "$ARTIFACT_ROOT" \
  --proof-output   proof/libuv-safe-validation-proof.json \
  --mode           port \
  --library        libuv \
  --require-casts \
  --min-source-cases 5 \
  --min-usage-cases  140 \
  --min-cases        145 \
  --ports-root /home/yans/safelibs/pipeline/ports

# 7. Render and verify the validator site.
python3 "$REPO/validator/tools/render_site.py" \
  --config        "$REPO/validator/repositories.yml" \
  --tests-root    "$REPO/validator/tests" \
  --artifact-root "$ARTIFACT_ROOT" \
  --proof-path    "$ARTIFACT_ROOT/proof/libuv-safe-validation-proof.json" \
  --output-root   "$SITE_ROOT"

bash "$REPO/validator/scripts/verify-site.sh" \
  --config         "$REPO/validator/repositories.yml" \
  --tests-root     "$REPO/validator/tests" \
  --artifacts-root "$ARTIFACT_ROOT" \
  --proof-path     "$ARTIFACT_ROOT/proof/libuv-safe-validation-proof.json" \
  --site-root      "$SITE_ROOT" \
  --library        libuv

# 8. Independent safe-crate sweep (check-26).
cargo build --manifest-path "$REPO/safe/Cargo.toml" --release
"$REPO/safe/tools/stage_install.sh"        /tmp/libuv-safe-validator-stage
"$REPO/safe/tools/verify_stage_install.sh" /tmp/libuv-safe-validator-stage
"$REPO/safe/tools/run_regressions.sh" \
  --stage /tmp/libuv-safe-validator-stage \
  --up-to-phase impl-13-validator-catchall-and-report
"$REPO/safe/tools/build_deb.sh"
python3 "$REPO/safe/tools/verify_deb_payload_contract.py" \
  "$REPO/safe/dist/artifacts.env" \
  "$REPO/original/include" \
  "$REPO/safe/debian/not-installed"
python3 "$REPO/safe/tools/audit_unsafe.py" "$REPO/safe/src"

# 9. Final commit-topology / report cross-checks (check-27).
python3 "$REPO/safe/tools/verify_phase_commit_sequence.py" \
  --base-file "$REPO/.plan/workflow-run-base.txt" \
  impl-08-validator-bootstrap-and-baseline \
  impl-09-validator-packaging-abi-fixes \
  impl-10-validator-core-loop-threading-fixes \
  impl-11-validator-fs-dns-process-fixes \
  impl-12-validator-network-io-fixes \
  impl-13-validator-catchall-and-report
git -C "$REPO/validator" diff           --exit-code
git -C "$REPO/validator" diff --cached  --exit-code
```

### phase-13 final port-mode run summary

From
`validator/artifacts/libuv-safe/check-25-final/port/results/libuv/summary.json`:

| Field         | Value      |
| ------------- | ---------- |
| schema_version| 2          |
| library       | libuv      |
| mode          | port       |
| cases         | 175        |
| source_cases  | 5          |
| usage_cases   | 170        |
| passed        | 175        |
| failed        | 0          |
| casts         | 175        |

All 175 non-summary result JSONs under
`validator/artifacts/libuv-safe/check-25-final/port/results/libuv/` carry
`"mode": "port"`. Counts match the cloned validator checkout exactly
(`validator/tests/libuv/tests/cases/source/*.sh` = 5,
`validator/tests/libuv/tests/cases/usage/*.sh` = 170, total 175), and all
175 cases pass with no skip. The thresholds required by the phase plan
(`--min-source-cases 5`, `--min-usage-cases 140`, `--min-cases 145`) are
met with a 30-case margin on usage and a 30-case margin on the total.

### Final result

- Strict original-mode override matrix (phase-12): **175 / 175 passed,
  0 failed**, all results `override_debs_installed: true`.
- Final port-mode validator run (phase-13, check-25-final):
  **175 / 175 passed, 0 failed**, all results `mode: "port"`.
- `validator/tools/verify_proof_artifacts.py --mode port` accepts the
  port-mode proof at `proof/libuv-safe-validation-proof.json` with the
  thresholds above.
- `validator/scripts/verify-site.sh --library libuv` exits 0 against the
  rendered site at `validator/site/libuv-safe-final/`. The site
  contains no final user-facing `\bsafe\b` / `\bunsafe\b` /
  `safe[- ]workload` token outside the per-case-row HTML — the
  port-deb lock's `release_tag` is the sanitized `local-libuv-port-…`
  form (see the validator-bug-shaped failure section above for why this
  rewrite is in `safe/tools/write_validator_port_lock.py`).
- `safe/tools/run_regressions.sh --up-to-phase impl-13-…` passes against
  `/tmp/libuv-safe-validator-stage`.
- `safe/tools/verify_deb_payload_contract.py` and
  `safe/tools/audit_unsafe.py safe/src` pass on the latest `.deb`
  packages and source tree.
- `git -C validator diff` is clean (no tracked validator file modified
  in any phase).
- No skipped check; no testcase disabled. The single non-testcase fix
  in this phase is the lock-writer sanitization documented in the
  “Validator-bug-shaped failure” section above, and it is pinned by the
  shell regression
  `safe/tests/regressions/validator_port_lock_safe_token_sanitized.sh`.

### Files changed in phase-13

- `safe/tools/write_validator_port_lock.py` (sanitize `\bsafe\b` /
  `\bunsafe\b` tokens in `--release-tag` / derived `tag_ref` so the
  documented check-25 verification command produces a lock and rendered
  site that pass `validator/scripts/verify-site.sh`).
- `safe/tests/regressions/validator_port_lock_safe_token_sanitized.sh`
  (new shell regression pinning the sanitization, including the
  positive `\bsafe\b` and `\bunsafe\b` cases and the negative
  pass-through case).
- `safe/tests/regressions/manifest.json` (registers the new probe with
  `phase_owner: "impl-13-validator-catchall-and-report"`).
- `validator-report.md` (this phase-13 section).

No `safe/src/**` or `safe/debian/**` file is changed in phase-13 — there
is no remaining validator testcase failure to repair against the
libuv-safe runtime, and the lock-writer fix is confined to
`safe/tools/`.

### Remaining failures after phase-13

None.
