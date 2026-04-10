#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../common.sh
. "${script_dir}/../common.sh"

main() {
  libuv_note "Testing Knot Resolver"
  (
    set -euo pipefail
    local dir
    dir="$(mktemp -d /tmp/kres-test.XXXXXX)"
    cat >"${dir}/config" <<'CFG'
net.listen("127.0.0.1", 5301, { kind = "dns" })
modules = { "hints > iterate" }
cache.size = 8 * MB
CFG
    /usr/sbin/kresd -n -q -c "${dir}/config" "${dir}" >"${dir}/kres.log" 2>&1 &
    pid=$!
    trap 'kill "${pid}" 2>/dev/null || true; wait "${pid}" 2>/dev/null || true' EXIT
    for _ in $(seq 1 80); do
      if dig +time=1 +tries=1 +short @127.0.0.1 -p 5301 example.com A >"${dir}/dig.out" 2>/dev/null && \
         [ -s "${dir}/dig.out" ]; then
        exit 0
      fi
      sleep 0.25
    done
    cat "${dir}/kres.log"
    exit 1
  )
}

main "$@"
