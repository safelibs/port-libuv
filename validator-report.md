# libuv-safe validator baseline (phase-08)

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
