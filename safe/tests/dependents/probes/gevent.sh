#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../common.sh
. "${script_dir}/../common.sh"

main() {
  local build_root
  local gevent_src
  local -a gevent_build_env

  libuv_note "Testing gevent source build with libuv"
  build_root="$(mktemp -d /tmp/gevent-build.XXXXXX)"
  (
    set -euo pipefail
    cd "${build_root}"
    apt-get source python-gevent >/tmp/gevent-source.log 2>&1
    gevent_src="$(find . -maxdepth 1 -type d -name 'python-gevent-*' | head -n 1)"
    [[ -n "${gevent_src}" ]] || libuv_fail "could not extract python-gevent source"
    cd "${gevent_src}"
    gevent_build_env=(
      GEVENTSETUP_EMBED_CARES=0
      GEVENTSETUP_EMBED_LIBEV=0
      GEVENTSETUP_EMBED_LIBUV=0
    )
    if [[ -n "${CPPFLAGS:-}" ]]; then
      gevent_build_env+=("CPPFLAGS=${CPPFLAGS}")
    fi
    if [[ -n "${LDFLAGS:-}" ]]; then
      gevent_build_env+=("LDFLAGS=${LDFLAGS}")
    fi
    env "${gevent_build_env[@]}" python3 setup.py build_ext -i >/tmp/gevent-build.log 2>&1
    libuv_record_probe_glob_root "${PWD}"
    [[ -n "$(find src/gevent/libuv -maxdepth 1 -name '_corecffi*.so' -print -quit)" ]] || \
      libuv_fail "gevent libuv backend was not built"
    PYTHONPATH="${PWD}/src" GEVENT_LOOP=libuv-cffi python3 - <<'PY'
import gevent
from gevent import sleep, spawn_later

flag = []
spawn_later(0.01, flag.append, 1)
sleep(0.05)
assert flag == [1]
print(type(gevent.get_hub().loop).__module__)
gevent.get_hub().destroy(destroy_loop=True)
PY
  )
}

main "$@"
