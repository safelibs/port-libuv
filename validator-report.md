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
