#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "usage: $0 --stage <prefix> --build <dir>" >&2
  exit 64
}

stage_prefix=""
build_dir=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --stage)
      [[ $# -ge 2 ]] || usage
      stage_prefix="$2"
      shift 2
      ;;
    --build)
      [[ $# -ge 2 ]] || usage
      build_dir="$2"
      shift 2
      ;;
    *)
      usage
      ;;
  esac
done

[[ -n "${stage_prefix}" && -n "${build_dir}" ]] || usage

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
safe_root="$(cd "${script_dir}/.." && pwd)"
repo_root="$(cd "${safe_root}/.." && pwd)"
generated_dir="${build_dir}/generated"

mkdir -p "${generated_dir}"

python3 - "${repo_root}" "${generated_dir}" <<'PY'
import subprocess
import sys
from pathlib import Path

repo_root = Path(sys.argv[1])
generated_dir = Path(sys.argv[2])
original_test_list = repo_root / "original/test/test-list.h"

selected_tests = [
    "version",
    "loop_new_delete",
    "print_handles",
    "handle_type_name",
    "req_type_name",
    "loop_configure",
    "loop_alive",
    "loop_close",
    "loop_stop",
    "run_once",
    "run_nowait",
]

expected_delta = {
    "loop_new_delete",
    "once",
    "print_handles",
    "udp_send_queue_getters",
    "version",
}

def listed(binary: Path) -> list[str]:
    return subprocess.check_output([str(binary), "--list"], text=True).splitlines()

review_tests = listed(repo_root / "original/build-checker-review/uv_run_tests")
review_tests_a = listed(repo_root / "original/build-checker-review/uv_run_tests_a")
review_bench = listed(repo_root / "original/build-checker-review/uv_run_benchmarks_a")
old_tests = listed(repo_root / "original/build-checker/uv_run_tests")
old_tests_a = listed(repo_root / "original/build-checker/uv_run_tests_a")
old_bench = listed(repo_root / "original/build-checker/uv_run_benchmarks_a")

if (len(review_tests), len(review_tests_a), len(review_bench)) != (440, 440, 55):
    raise SystemExit("unexpected original/build-checker-review inventory counts")

if (len(old_tests), len(old_tests_a), len(old_bench)) != (435, 435, 55):
    raise SystemExit("unexpected original/build-checker inventory counts")

if set(review_tests) - set(old_tests) != expected_delta or set(old_tests) - set(review_tests):
    raise SystemExit("shared test inventory delta no longer matches the checked-in contract")

if set(review_tests_a) - set(old_tests_a) != expected_delta or set(old_tests_a) - set(review_tests_a):
    raise SystemExit("static test inventory delta no longer matches the checked-in contract")

if set(review_bench) != set(old_bench):
    raise SystemExit("benchmark inventory changed unexpectedly between baseline trees")

test_list_text = original_test_list.read_text()
for name in selected_tests:
    needle = f"TEST_DECLARE   ({name})"
    if needle not in test_list_text:
        raise SystemExit(f"missing {needle} in current test-list.h")

phase_test_list = "\n".join(
    [*(f"TEST_DECLARE({name})" for name in selected_tests),
     "TASK_LIST_START",
     *(f"  TEST_ENTRY({name})" for name in selected_tests),
     "TASK_LIST_END",
     ""]
)

run_tests_c = """\
#include <stdio.h>
#include <string.h>

#ifndef _WIN32
# include <unistd.h>
#endif

#include "uv.h"
#include "runner.h"
#include "task.h"
#include "phase-test-list.h"

static int maybe_run_test(int argc, char** argv);

int main(int argc, char** argv) {
  platform_init(argc, argv);
  argv = uv_setup_args(argc, argv);

  switch (argc) {
  case 1:
    return run_tests(0);
  case 2:
    return maybe_run_test(argc, argv);
  case 3:
    return run_test_part(argv[1], argv[2]);
  default:
    fprintf(stderr, "Too many arguments.\\n");
    fflush(stderr);
    return 1;
  }
}

static int maybe_run_test(int argc, char** argv) {
  (void) argc;
  if (strcmp(argv[1], "--list") == 0) {
    print_tests(stdout);
    return 0;
  }
  return run_test(argv[1], 0, 1);
}
"""

benchmark_main_c = """\
int run_benchmark_sizes(void);
int main(void) {
  return run_benchmark_sizes();
}
"""

generated_dir.mkdir(parents=True, exist_ok=True)
(generated_dir / "phase-test-list.h").write_text(phase_test_list)
(generated_dir / "uv-safe-run-tests.c").write_text(run_tests_c)
(generated_dir / "benchmark-sizes-main.c").write_text(benchmark_main_c)
PY

cmake -S "${safe_root}/tests/harness" \
  -B "${build_dir}" \
  -DSTAGE_PREFIX="${stage_prefix}" \
  -DGENERATED_DIR="${generated_dir}"
cmake --build "${build_dir}"
