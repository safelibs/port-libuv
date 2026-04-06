#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"
SAFE_DIR="${ROOT_DIR}/safe"
ORIGINAL_DIR="${ROOT_DIR}/original"
BUILD_DIR="${ROOT_DIR}/build-check"

mkdir -p "${SAFE_DIR}/abi" "${SAFE_DIR}/include" "${SAFE_DIR}/perf"

rm -rf "${SAFE_DIR}/include"
mkdir -p "${SAFE_DIR}/include"
cp -a "${ORIGINAL_DIR}/include/." "${SAFE_DIR}/include/"

"${BUILD_DIR}/uv_run_tests" --list > "${SAFE_DIR}/abi/linux-test-list.txt"
"${BUILD_DIR}/uv_run_tests_a" --list > "${SAFE_DIR}/abi/linux-test-list-static.txt"
"${BUILD_DIR}/uv_run_benchmarks_a" --list > "${SAFE_DIR}/perf/linux-benchmark-list.txt"
nm -D --defined-only "${BUILD_DIR}/libuv.so.1.0.0" | awk '{print $3}' | sort > "${SAFE_DIR}/abi/linux-exported-symbols.txt"
"${BUILD_DIR}/uv_run_benchmarks_a" sizes 2>&1 | sed '1d' > "${SAFE_DIR}/abi/linux-struct-sizes.txt"

python3 - <<'PY' "${BUILD_DIR}/compile_commands.json" "${SAFE_DIR}/abi/linux-source-manifest.txt"
import json
import sys
from pathlib import Path

compile_commands = json.loads(Path(sys.argv[1]).read_text())
sources = sorted({
    entry["file"]
    for entry in compile_commands
    if entry.get("output", "").startswith(("CMakeFiles/uv.dir/", "CMakeFiles/uv_a.dir/"))
})
Path(sys.argv[2]).write_text("\n".join(sources) + "\n")
PY

cp "${BUILD_DIR}/libuv.pc" "${SAFE_DIR}/abi/original-libuv.pc"
cp "${BUILD_DIR}/libuv-static.pc" "${SAFE_DIR}/abi/original-libuv-static.pc"
cp "${BUILD_DIR}/CMakeFiles/uv.dir/link.txt" "${SAFE_DIR}/abi/original-uv.link.txt"
cp "${BUILD_DIR}/CMakeFiles/uv_a.dir/link.txt" "${SAFE_DIR}/abi/original-uv_a.link.txt"
cp "${BUILD_DIR}/CMakeFiles/uv_run_tests.dir/link.txt" "${SAFE_DIR}/abi/original-uv_run_tests.link.txt"
cp "${BUILD_DIR}/CMakeFiles/uv_run_tests_a.dir/link.txt" "${SAFE_DIR}/abi/original-uv_run_tests_a.link.txt"
cp "${BUILD_DIR}/CMakeFiles/uv_run_benchmarks_a.dir/link.txt" "${SAFE_DIR}/abi/original-uv_run_benchmarks_a.link.txt"

find "${BUILD_DIR}/CMakeFiles/uv_run_tests.dir/test" -type f -name '*.o' | sort > "${SAFE_DIR}/abi/original-uv_run_tests.objects.txt"
find "${BUILD_DIR}/CMakeFiles/uv_run_tests_a.dir/test" -type f -name '*.o' | sort > "${SAFE_DIR}/abi/original-uv_run_tests_a.objects.txt"
find "${BUILD_DIR}/CMakeFiles/uv_run_benchmarks_a.dir/test" -type f -name '*.o' | sort > "${SAFE_DIR}/abi/original-uv_run_benchmarks_a.objects.txt"

python3 - <<'PY' "${SAFE_DIR}/abi/linux-exported-symbols.txt" "${SAFE_DIR}/abi/libuv.map"
import sys
from pathlib import Path

symbols = [line.strip() for line in Path(sys.argv[1]).read_text().splitlines() if line.strip()]
lines = ["LIBUV_1.0 {", "  global:"]
lines.extend(f"    {symbol};" for symbol in symbols)
lines.extend(["  local:", "    *;", "};", ""])
Path(sys.argv[2]).write_text("\n".join(lines))
PY

python3 - <<'PY' "${BUILD_DIR}/uv_run_tests_a" "${SAFE_DIR}/abi/linux-targeted-static-test-outcomes.json"
import json
import subprocess
import sys
from pathlib import Path

TESTS = sorted({
    "active",
    "async",
    "async_null_cb",
    "async_ref",
    "barrier_1",
    "barrier_2",
    "barrier_3",
    "barrier_serial_thread",
    "barrier_serial_thread_single",
    "callback_stack",
    "check_ref",
    "clock_gettime",
    "close_order",
    "condvar_1",
    "condvar_2",
    "condvar_3",
    "condvar_4",
    "condvar_5",
    "cwd_and_chdir",
    "default_loop_close",
    "dlerror",
    "embed",
    "env_vars",
    "err_name_r",
    "error_message",
    "get_currentexe",
    "get_group",
    "get_loadavg",
    "get_memory",
    "get_passwd",
    "get_passwd2",
    "getaddrinfo_basic",
    "getaddrinfo_basic_sync",
    "getaddrinfo_concurrent",
    "gethostname",
    "getnameinfo_basic_ip4",
    "getnameinfo_basic_ip4_sync",
    "getnameinfo_basic_ip6",
    "getters_setters",
    "gettimeofday",
    "handle_type_name",
    "has_ref",
    "homedir",
    "idle_check",
    "idle_ref",
    "idle_starvation",
    "idna_toascii",
    "ip4_addr",
    "ip6_addr_link_local",
    "ip_name",
    "loop_alive",
    "loop_backend_timeout",
    "loop_close",
    "loop_configure",
    "loop_handles",
    "loop_instant_close",
    "loop_new_delete",
    "loop_stop",
    "loop_stop_before_run",
    "loop_update_time",
    "metrics_idle_time",
    "metrics_idle_time_thread",
    "metrics_idle_time_zero",
    "metrics_info_check",
    "metrics_pool_events",
    "platform_output",
    "prepare_ref",
    "process_title",
    "process_title_big_argv",
    "process_title_threadsafe",
    "queue_foreach_delete",
    "random_async",
    "random_sync",
    "ref",
    "req_accessors",
    "req_type_name",
    "run_nowait",
    "run_once",
    "semaphore_1",
    "semaphore_2",
    "semaphore_3",
    "strscpy",
    "strtok",
    "thread_affinity",
    "thread_create",
    "thread_equal",
    "thread_local_storage",
    "thread_mutex",
    "thread_mutex_recursive",
    "thread_once",
    "thread_priority",
    "thread_rwlock",
    "thread_rwlock_trylock",
    "thread_stack_size",
    "thread_stack_size_explicit",
    "threadpool_cancel_random",
    "threadpool_queue_work_einval",
    "threadpool_queue_work_simple",
    "timer",
    "timer_again",
    "timer_early_check",
    "timer_from_check",
    "timer_huge_repeat",
    "timer_huge_timeout",
    "timer_init",
    "timer_is_closing",
    "timer_no_double_call_nowait",
    "timer_no_double_call_once",
    "timer_no_run_on_unref",
    "timer_null_callback",
    "timer_order",
    "timer_run_once",
    "timer_start_twice",
    "timer_zero_timeout",
    "tmpdir",
    "udp_send_queue_accessors",
    "uname",
    "unref_in_prepare_cb",
    "utf8_decode1",
    "utf8_decode1_overrun",
    "version",
    "walk_handles",
})

runner = Path(sys.argv[1]).resolve()
output_path = Path(sys.argv[2])
results = {}

for test_name in TESTS:
    proc = subprocess.run(
        [str(runner), test_name],
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        check=False,
    )
    lines = proc.stdout.splitlines()
    tap_lines = [line for line in lines if line.startswith(("ok ", "not ok "))]
    if not tap_lines:
        raise SystemExit(f"{test_name}: missing TAP line in runner output")
    results[test_name] = {
        "exit_code": proc.returncode,
        "final_tap_line": tap_lines[-1],
    }

payload = {
    "runner": str(runner),
    "tests": results,
}
output_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n")
PY
