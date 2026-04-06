#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <build-dir>" >&2
  exit 1
fi

BUILD_DIR="$(cd "$1" && pwd)"
WORK_DIR="$(mktemp -d)"
trap 'rm -rf "${WORK_DIR}"' EXIT

cat > "${WORK_DIR}/smoke.c" <<'EOF'
#include <stdio.h>
#include <uv.h>

int main(void) {
  if (uv_version() != UV_VERSION_HEX) {
    fprintf(stderr, "unexpected libuv version: %u\n", uv_version());
    return 1;
  }

  printf("%s\n", uv_version_string());
  return 0;
}
EOF

cc \
  -I"${BUILD_DIR}/include" \
  "${WORK_DIR}/smoke.c" \
  -L"${BUILD_DIR}" \
  -Wl,-rpath,"${BUILD_DIR}" \
  -luv -lpthread -ldl -lrt \
  -o "${WORK_DIR}/smoke"

"${WORK_DIR}/smoke" >/dev/null
