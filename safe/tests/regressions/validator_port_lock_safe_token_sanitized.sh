#!/usr/bin/env bash
set -euo pipefail

# Regression for impl-13-validator-catchall-and-report.
#
# safe/tools/write_validator_port_lock.py must coerce any "\bsafe\b" /
# "\bunsafe\b" token in the supplied --release-tag (and the derived
# tag_ref) to "port" before writing the lock. The validator's
# verify-site.sh rejects rendered HTML containing those tokens outside
# per-case rows, and the documented check-25 verification command in
# .plan/phases/06-validator-catchall-and-report.md feeds the writer
# release-tags of the form "local-libuv-safe-<short-commit>" — the
# embedded "-safe-" trips word boundaries on either side of "safe".
#
# Without sanitization the rendered site under
# validator/site/libuv-safe-final/library/libuv.html carries the raw
# release-tag, fails the verifier, and the documented final clean
# port-mode validator run cannot be reproduced.

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/../../.." && pwd)"
writer="${repo_root}/safe/tools/write_validator_port_lock.py"

artifacts_env="${repo_root}/safe/dist/artifacts.env"
if [[ ! -f "${artifacts_env}" ]]; then
  echo "missing artifacts.env: ${artifacts_env} (build the libuv-safe debs first)" >&2
  exit 1
fi

work_dir="$(mktemp -d -t libuv-safe-port-lock-regression-XXXXXXXX)"
trap 'rm -rf "${work_dir}"' EXIT

override_root="${work_dir}/local-debs"
mkdir -p "${override_root}/libuv"

runtime_deb="$(awk -F= '$1=="LIBUV_SAFE_RUNTIME_DEB"{print $2}' "${artifacts_env}")"
dev_deb="$(awk -F= '$1=="LIBUV_SAFE_DEV_DEB"{print $2}' "${artifacts_env}")"

cp "${runtime_deb}" "${override_root}/libuv/"
cp "${dev_deb}"     "${override_root}/libuv/"

lock_path="${work_dir}/local-port-debs-lock.json"
release_tag_input="local-libuv-safe-deadbeef"
release_tag_expected="local-libuv-port-deadbeef"

python3 "${writer}" \
  --artifacts-env "${artifacts_env}" \
  --override-root "${override_root}" \
  --lock-output   "${lock_path}" \
  --library       libuv \
  --repository    safelibs/port-libuv \
  --commit        "0000000000000000000000000000000000000000" \
  --release-tag   "${release_tag_input}"

actual_release_tag="$(python3 -c '
import json, sys
print(json.loads(open(sys.argv[1]).read())["libraries"][0]["release_tag"])
' "${lock_path}")"

actual_tag_ref="$(python3 -c '
import json, sys
print(json.loads(open(sys.argv[1]).read())["libraries"][0]["tag_ref"])
' "${lock_path}")"

if [[ "${actual_release_tag}" != "${release_tag_expected}" ]]; then
  echo "release_tag = ${actual_release_tag}; want ${release_tag_expected}" >&2
  exit 1
fi

if [[ "${actual_tag_ref}" != "refs/tags/${release_tag_expected}" ]]; then
  echo "tag_ref = ${actual_tag_ref}; want refs/tags/${release_tag_expected}" >&2
  exit 1
fi

# Hard-fail the rejection regex from validator/scripts/verify-site.sh
# against the rendered text fields on the lock entry.
python3 - "${lock_path}" <<'PY'
import json
import re
import sys

REJECT = re.compile(r"\bsafe\b|\bunsafe\b|safe[- ]workload", flags=re.IGNORECASE)

lock = json.loads(open(sys.argv[1]).read())
entry = lock["libraries"][0]
for field in ("release_tag", "tag_ref"):
    value = entry[field]
    if REJECT.search(value):
        print(
            f"sanitized lock field {field}={value!r} still matches verify-site.sh "
            f"rejection regex",
            file=sys.stderr,
        )
        sys.exit(1)

# Also exercise an unsafe-shaped tag.
PY

# A second pass with an "unsafe" token must also be coerced.
release_tag_input_unsafe="local-libuv-unsafe-cafebabe"
release_tag_expected_unsafe="local-libuv-port-cafebabe"

python3 "${writer}" \
  --artifacts-env "${artifacts_env}" \
  --override-root "${override_root}" \
  --lock-output   "${lock_path}" \
  --library       libuv \
  --repository    safelibs/port-libuv \
  --commit        "0000000000000000000000000000000000000000" \
  --release-tag   "${release_tag_input_unsafe}"

actual_release_tag="$(python3 -c '
import json, sys
print(json.loads(open(sys.argv[1]).read())["libraries"][0]["release_tag"])
' "${lock_path}")"

if [[ "${actual_release_tag}" != "${release_tag_expected_unsafe}" ]]; then
  echo "release_tag = ${actual_release_tag}; want ${release_tag_expected_unsafe}" >&2
  exit 1
fi

# A clean tag (no rejected token) must pass through unchanged so the
# sanitization is scoped to the offending tokens only.
release_tag_clean="v1.48.0+safelibs1"
python3 "${writer}" \
  --artifacts-env "${artifacts_env}" \
  --override-root "${override_root}" \
  --lock-output   "${lock_path}" \
  --library       libuv \
  --repository    safelibs/port-libuv \
  --commit        "0000000000000000000000000000000000000000" \
  --release-tag   "${release_tag_clean}"

actual_release_tag="$(python3 -c '
import json, sys
print(json.loads(open(sys.argv[1]).read())["libraries"][0]["release_tag"])
' "${lock_path}")"

if [[ "${actual_release_tag}" != "${release_tag_clean}" ]]; then
  echo "release_tag = ${actual_release_tag}; want ${release_tag_clean} (unchanged)" >&2
  exit 1
fi

echo "validator_port_lock_safe_token_sanitized OK"
