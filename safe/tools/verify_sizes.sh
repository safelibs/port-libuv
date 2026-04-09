#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <stage-prefix>" >&2
  exit 64
fi

stage_prefix="$1"
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
safe_root="$(cd "${script_dir}/.." && pwd)"
tmp_build="$(mktemp -d /tmp/libuv-safe-sizes.XXXXXX)"
trap 'rm -rf "${tmp_build}"' EXIT

"${script_dir}/build_upstream_harness.sh" --stage "${stage_prefix}" --build "${tmp_build}"

python3 - "${tmp_build}" <<'PY'
import re
import subprocess
import sys
from pathlib import Path

build_dir = Path(sys.argv[1])
expected = {
    "uv_shutdown_t": 80,
    "uv_write_t": 192,
    "uv_connect_t": 96,
    "uv_udp_send_t": 320,
    "uv_tcp_t": 248,
    "uv_pipe_t": 264,
    "uv_tty_t": 312,
    "uv_prepare_t": 120,
    "uv_check_t": 120,
    "uv_idle_t": 120,
    "uv_async_t": 128,
    "uv_timer_t": 152,
    "uv_fs_poll_t": 104,
    "uv_fs_event_t": 136,
    "uv_process_t": 136,
    "uv_poll_t": 160,
    "uv_loop_t": 848,
}

pattern = re.compile(r"^(uv_[A-Za-z0-9_]+): (\d+) bytes$")

for binary_name in ("uv_safe_benchmark_sizes_shared", "uv_safe_benchmark_sizes_static"):
    output = subprocess.check_output(
        [str(build_dir / binary_name)],
        text=True,
        stderr=subprocess.STDOUT,
    )
    seen = {}
    for line in output.splitlines():
        match = pattern.match(line.strip())
        if match:
            seen[match.group(1)] = int(match.group(2))

    if seen != expected:
        missing = sorted(set(expected) - set(seen))
        mismatch = sorted(k for k in expected if k in seen and seen[k] != expected[k])
        extra = sorted(set(seen) - set(expected))
        if missing:
            raise SystemExit(f"{binary_name}: missing size lines for {', '.join(missing)}")
        if mismatch:
            details = ", ".join(f"{name}={seen[name]} expected {expected[name]}" for name in mismatch)
            raise SystemExit(f"{binary_name}: size mismatches: {details}")
        if extra:
            raise SystemExit(f"{binary_name}: unexpected extra size lines: {', '.join(extra)}")

print("verified benchmark size probe output for shared and static harnesses")
PY
