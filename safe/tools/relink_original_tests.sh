#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "usage: $0 (--shared|--static) [--no-run] [--run-smoke <test>] --build-dir <dir> --stage <prefix>" >&2
  exit 64
}

mode=""
build_dir=""
stage_prefix=""
no_run=0
run_smoke=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --shared)
      mode="shared"
      shift
      ;;
    --static)
      mode="static"
      shift
      ;;
    --no-run)
      no_run=1
      shift
      ;;
    --run-smoke)
      [[ $# -ge 2 ]] || usage
      run_smoke="$2"
      shift 2
      ;;
    --build-dir)
      [[ $# -ge 2 ]] || usage
      build_dir="$2"
      shift 2
      ;;
    --stage)
      [[ $# -ge 2 ]] || usage
      stage_prefix="$2"
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done

[[ -n "${mode}" && -n "${build_dir}" && -n "${stage_prefix}" ]] || usage

python3 - "${mode}" "${build_dir}" "${stage_prefix}" "${no_run}" "${run_smoke}" <<'PY'
import pathlib
import shlex
import subprocess
import sys

mode = sys.argv[1]
build_dir = pathlib.Path(sys.argv[2]).resolve()
stage_prefix = pathlib.Path(sys.argv[3]).resolve()
no_run = bool(int(sys.argv[4]))
run_smoke = sys.argv[5]
out_dir = build_dir / ".safe-relink" / mode
out_dir.mkdir(parents=True, exist_ok=True)

if mode == "shared":
    targets = [build_dir / "CMakeFiles/uv_run_tests.dir/link.txt"]
else:
    targets = [
        build_dir / "CMakeFiles/uv_run_tests_a.dir/link.txt",
        build_dir / "CMakeFiles/uv_run_benchmarks_a.dir/link.txt",
    ]

for link_txt in targets:
    if not link_txt.exists():
        raise SystemExit(f"missing linker recipe: {link_txt}")

    command = shlex.split(link_txt.read_text().strip())
    out_index = command.index("-o") + 1
    binary_name = pathlib.Path(command[out_index]).name
    output_path = out_dir / binary_name
    command[out_index] = str(output_path)

    if mode == "shared":
        command = [
            str(stage_prefix / "lib/libuv.so") if token.endswith("libuv.so.1.0.0") else token
            for token in command
        ]
        command = [
            token
            for token in command
            if not token.startswith("-Wl,-rpath,")
        ]
        command.append(f"-Wl,-rpath,{stage_prefix / 'lib'}")
    else:
        command = [
            str(stage_prefix / "lib/libuv.a") if token == "libuv.a" else token
            for token in command
        ]

    subprocess.check_call(command, cwd=build_dir)

    if not no_run:
        if run_smoke and output_path.name.startswith("uv_run_tests"):
            probe = [run_smoke]
        else:
            probe = ["--list"]
        subprocess.check_call([str(output_path), *probe], cwd=build_dir)

    print(output_path)
PY
