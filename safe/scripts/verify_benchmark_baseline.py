#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path

from compare_benchmarks import compare_reports, load_report, load_selection, load_thresholds


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.add_argument("--baseline", required=True)
    parser.add_argument("--candidate", required=True)
    parser.add_argument("--selection", required=True)
    parser.add_argument("--thresholds", required=True)
    parser.add_argument("--expected-runner", required=True)
    return parser


def main() -> int:
    args = build_parser().parse_args()
    selection = load_selection(Path(args.selection))
    thresholds = load_thresholds(Path(args.thresholds))
    baseline = load_report(Path(args.baseline))
    candidate = load_report(Path(args.candidate))
    expected_runner = str(Path(args.expected_runner).resolve())

    for name, report in (("baseline", baseline), ("candidate", candidate)):
        if report["runner"] != expected_runner:
            raise SystemExit(f"{name} runner mismatch: {report['runner']} != {expected_runner}")
        if report["selection"] != selection:
            raise SystemExit(f"{name} selection mismatch")

    _, failures = compare_reports(
        baseline,
        candidate,
        selection,
        thresholds,
        baseline_mode=True,
    )
    if failures:
        for failure in failures:
            print(failure, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
