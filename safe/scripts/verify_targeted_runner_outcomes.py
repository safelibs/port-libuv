#!/usr/bin/env python3
import argparse
import json
import subprocess
import sys
from pathlib import Path


def final_tap_line(output: str) -> str:
    tap_lines = [line for line in output.splitlines() if line.startswith(("ok ", "not ok "))]
    if not tap_lines:
        raise ValueError("runner output did not contain a TAP result line")
    return tap_lines[-1]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--runner", required=True)
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--tests", nargs="*")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    manifest = json.loads(Path(args.manifest).read_text())
    runner = Path(args.runner).resolve()
    manifest_tests = manifest["tests"]
    tests = sorted(args.tests if args.tests else manifest_tests.keys())

    for test_name in tests:
        if test_name not in manifest_tests:
            raise SystemExit(f"{test_name}: missing from {args.manifest}")
        expected = manifest_tests[test_name]
        proc = subprocess.run(
            [str(runner), test_name],
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            check=False,
        )
        actual_tap = final_tap_line(proc.stdout)
        if proc.returncode != expected["exit_code"] or actual_tap != expected["final_tap_line"]:
            print(f"{test_name}: expected exit {expected['exit_code']} and {expected['final_tap_line']!r}", file=sys.stderr)
            print(f"{test_name}: observed exit {proc.returncode} and {actual_tap!r}", file=sys.stderr)
            return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
