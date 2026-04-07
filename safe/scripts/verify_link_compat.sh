#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
safe_dir="$(cd -- "$script_dir/.." && pwd)"
repo_root="$(cd -- "$safe_dir/.." && pwd)"
abi_dir="$safe_dir/abi"

usage() {
  printf 'usage: %s <build-check-dir> <safe-build-dir>\n' "${BASH_SOURCE[0]}" >&2
  exit 1
}

fail() {
  printf 'ERROR: %s\n' "$*" >&2
  exit 1
}

require_file() {
  [[ -e "$1" ]] || fail "missing required file: $1"
}

if [[ $# -ne 2 ]]; then
  usage
fi

build_check_dir="$(readlink -f "$1")"
safe_build_dir="$(readlink -f "$2")"
out_dir="$safe_build_dir/link-compat"

require_file "$build_check_dir/libuv_test_idna_intercept.so"
require_file "$safe_build_dir/libuv.so.1.0.0"
require_file "$safe_build_dir/libuv.a"

mkdir -p "$out_dir"
rm -f "$out_dir"/uv_run_tests "$out_dir"/uv_run_tests_a "$out_dir"/uv_run_benchmarks_a

build_runner() {
  local mode="$1"
  local link_manifest="$2"
  local object_manifest="$3"
  local output_path="$4"
  local cmd_file
  local runner_name
  local -a link_cmd=()

  runner_name="$(basename "$output_path")"
  require_file "$link_manifest"
  require_file "$object_manifest"
  cmd_file="$(mktemp)"

  if ! python3 - "$mode" "$link_manifest" "$object_manifest" "$build_check_dir" "$safe_build_dir" "$output_path" >"$cmd_file" <<'PY'
from collections import Counter
import os
import shlex
import sys

mode, link_manifest, object_manifest, build_check_dir, safe_build_dir, output_path = sys.argv[1:]

with open(link_manifest, encoding="utf-8") as fh:
    line = fh.read().strip()
if not line:
    raise SystemExit(f"empty link manifest: {link_manifest}")

tokens = shlex.split(line)
compiler = tokens[0]

with open(object_manifest, encoding="utf-8") as fh:
    manifest_objects = [line.strip() for line in fh if line.strip()]

link_objects = [token for token in tokens if token.endswith(".o")]
if len(link_objects) != len(manifest_objects):
    raise SystemExit(
        f"{object_manifest}: expected {len(link_objects)} objects from link manifest, "
        f"found {len(manifest_objects)}"
    )

relative_objects = [os.path.relpath(path, build_check_dir) for path in manifest_objects]
if Counter(relative_objects) != Counter(link_objects):
    missing = sorted((Counter(relative_objects) - Counter(link_objects)).elements())
    extra = sorted((Counter(link_objects) - Counter(relative_objects)).elements())
    raise SystemExit(
        f"{object_manifest}: object list mismatch\n"
        f"missing-from-link={missing[:5]}\n"
        f"extra-in-link={extra[:5]}"
    )

ordered_objects = [os.path.join(build_check_dir, relpath) for relpath in link_objects]
for path in ordered_objects:
    if not os.path.exists(path):
        raise SystemExit(f"missing object file: {path}")

extra = []
index = 1
while index < len(tokens):
    token = tokens[index]
    if token == "-o":
        index += 2
        continue
    if token.endswith(".o"):
        index += 1
        continue
    if token in {"libuv.so.1.0.0", "libuv.a"}:
        index += 1
        continue
    if "build-check" in token:
        index += 1
        continue
    extra.append(token)
    index += 1

command = [compiler, *ordered_objects, "-o", output_path]
if mode == "shared":
    command.extend([f"-Wl,-rpath,{safe_build_dir}", os.path.join(safe_build_dir, "libuv.so.1.0.0")])
elif mode == "static":
    command.append(os.path.join(safe_build_dir, "libuv.a"))
else:
    raise SystemExit(f"unsupported mode: {mode}")
command.extend(extra)

for token in command:
    sys.stdout.write(token)
    sys.stdout.write("\0")
PY
  then
    rm -f "$cmd_file"
    fail "failed to reconstruct link command for $runner_name"
  fi

  mapfile -d '' -t link_cmd <"$cmd_file"
  rm -f "$cmd_file"

  printf '==> relinking %s with %s original objects\n' "$runner_name" "$(wc -l < "$object_manifest")"
  printf '%q ' "${link_cmd[@]}" >"$out_dir/$runner_name.link.cmd"
  printf '\n' >>"$out_dir/$runner_name.link.cmd"
  "${link_cmd[@]}"
}

build_runner shared \
  "$abi_dir/original-uv_run_tests.link.txt" \
  "$abi_dir/original-uv_run_tests.objects.txt" \
  "$out_dir/uv_run_tests"
build_runner static \
  "$abi_dir/original-uv_run_tests_a.link.txt" \
  "$abi_dir/original-uv_run_tests_a.objects.txt" \
  "$out_dir/uv_run_tests_a"
build_runner static \
  "$abi_dir/original-uv_run_benchmarks_a.link.txt" \
  "$abi_dir/original-uv_run_benchmarks_a.objects.txt" \
  "$out_dir/uv_run_benchmarks_a"

readelf -d "$out_dir/uv_run_tests" >"$out_dir/uv_run_tests.readelf.txt"
grep -F "$safe_build_dir" "$out_dir/uv_run_tests.readelf.txt" >/dev/null || \
  fail "readelf did not record $safe_build_dir in the shared runner search path"
if grep -F "$build_check_dir" "$out_dir/uv_run_tests.readelf.txt" >/dev/null; then
  fail "shared runner still references build-check in its dynamic search path"
fi

ldd "$out_dir/uv_run_tests" >"$out_dir/uv_run_tests.ldd.txt"
if ! grep -F "libuv.so.1 => $safe_build_dir/libuv.so.1 " "$out_dir/uv_run_tests.ldd.txt" >/dev/null && \
   ! grep -F "libuv.so.1 => $safe_build_dir/libuv.so.1.0.0 " "$out_dir/uv_run_tests.ldd.txt" >/dev/null; then
  fail "ldd did not resolve libuv.so.1 from $safe_build_dir"
fi
if grep -F "$build_check_dir" "$out_dir/uv_run_tests.ldd.txt" >/dev/null; then
  fail "ldd resolved a dependency from build-check"
fi

(
  cd "$safe_dir"

  env LD_LIBRARY_PATH="$safe_build_dir" \
    "$out_dir/uv_run_tests" --list >"$out_dir/uv_run_tests.list.txt"
  diff -u "$abi_dir/linux-test-list.txt" "$out_dir/uv_run_tests.list.txt"

  "$out_dir/uv_run_tests_a" --list >"$out_dir/uv_run_tests_a.list.txt"
  diff -u "$abi_dir/linux-test-list-static.txt" "$out_dir/uv_run_tests_a.list.txt"

  env LD_LIBRARY_PATH="$safe_build_dir" \
    UV_TEST_TIMEOUT_MULTIPLIER=2 \
    "$out_dir/uv_run_tests" >"$out_dir/uv_run_tests.run.log" 2>&1

  UV_TEST_TIMEOUT_MULTIPLIER=2 \
    "$out_dir/uv_run_tests_a" >"$out_dir/uv_run_tests_a.run.log" 2>&1

  "$out_dir/uv_run_benchmarks_a" sizes | grep '^# ' >"$out_dir/uv_run_benchmarks_a.sizes.txt"
  diff -u "$abi_dir/linux-struct-sizes.txt" "$out_dir/uv_run_benchmarks_a.sizes.txt"
)

printf 'link compatibility verified in %s\n' "$out_dir"
