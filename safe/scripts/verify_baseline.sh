#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"

bash "${SCRIPT_DIR}/capture_baseline.sh"
diff -ru "${ROOT_DIR}/original/include" "${ROOT_DIR}/safe/include"
diff -u "${ROOT_DIR}/safe/abi/linux-exported-symbols.txt" <(nm -D --defined-only "${ROOT_DIR}/build-check/libuv.so.1.0.0" | awk '{print $3}' | sort)
diff -u "${ROOT_DIR}/safe/abi/linux-test-list.txt" <("${ROOT_DIR}/build-check/uv_run_tests" --list)
diff -u "${ROOT_DIR}/safe/abi/linux-test-list-static.txt" <("${ROOT_DIR}/build-check/uv_run_tests_a" --list)
python3 "${SCRIPT_DIR}/verify_targeted_runner_outcomes.py" \
  --runner "${ROOT_DIR}/build-check/uv_run_tests_a" \
  --manifest "${ROOT_DIR}/safe/abi/linux-targeted-static-test-outcomes.json"
diff -u "${ROOT_DIR}/safe/abi/linux-struct-sizes.txt" <("${ROOT_DIR}/build-check/uv_run_benchmarks_a" sizes 2>&1 | sed '1d')
diff -u "${ROOT_DIR}/safe/abi/linux-source-manifest.txt" <(python3 - <<'PY' "${ROOT_DIR}/build-check/compile_commands.json"
import json
import sys
from pathlib import Path

data = json.loads(Path(sys.argv[1]).read_text())
files = sorted({
    entry["file"]
    for entry in data
    if entry.get("output", "").startswith(("CMakeFiles/uv.dir/", "CMakeFiles/uv_a.dir/"))
})
print("\n".join(files))
PY
)
diff -u "${ROOT_DIR}/safe/abi/original-libuv.pc" "${ROOT_DIR}/build-check/libuv.pc"
diff -u "${ROOT_DIR}/safe/abi/original-libuv-static.pc" "${ROOT_DIR}/build-check/libuv-static.pc"
diff -u "${ROOT_DIR}/safe/abi/original-uv.link.txt" "${ROOT_DIR}/build-check/CMakeFiles/uv.dir/link.txt"
diff -u "${ROOT_DIR}/safe/abi/original-uv_a.link.txt" "${ROOT_DIR}/build-check/CMakeFiles/uv_a.dir/link.txt"
diff -u "${ROOT_DIR}/safe/abi/original-uv_run_tests.link.txt" "${ROOT_DIR}/build-check/CMakeFiles/uv_run_tests.dir/link.txt"
diff -u "${ROOT_DIR}/safe/abi/original-uv_run_tests_a.link.txt" "${ROOT_DIR}/build-check/CMakeFiles/uv_run_tests_a.dir/link.txt"
diff -u "${ROOT_DIR}/safe/abi/original-uv_run_benchmarks_a.link.txt" "${ROOT_DIR}/build-check/CMakeFiles/uv_run_benchmarks_a.dir/link.txt"
diff -u "${ROOT_DIR}/safe/abi/original-uv_run_tests.objects.txt" <(find "${ROOT_DIR}/build-check/CMakeFiles/uv_run_tests.dir/test" -type f -name '*.o' | sort)
diff -u "${ROOT_DIR}/safe/abi/original-uv_run_tests_a.objects.txt" <(find "${ROOT_DIR}/build-check/CMakeFiles/uv_run_tests_a.dir/test" -type f -name '*.o' | sort)
diff -u "${ROOT_DIR}/safe/abi/original-uv_run_benchmarks_a.objects.txt" <(find "${ROOT_DIR}/build-check/CMakeFiles/uv_run_benchmarks_a.dir/test" -type f -name '*.o' | sort)
python3 "${SCRIPT_DIR}/verify_transition_plan.py" \
  --source-manifest "${ROOT_DIR}/safe/abi/linux-source-manifest.txt" \
  --plan "${ROOT_DIR}/safe/legacy/linux-transition-plan.toml" \
  --manifest "${ROOT_DIR}/safe/legacy/linux-manifest.toml" \
  --expect-complete-through impl_02_legacy_build_and_test_harness
