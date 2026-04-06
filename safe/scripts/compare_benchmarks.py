#!/usr/bin/env python3
import argparse
import json
import math
import statistics
import sys
from pathlib import Path


def load_selection(path: Path) -> list[str]:
    return [line.strip() for line in path.read_text().splitlines() if line.strip()]


def load_thresholds(path: Path) -> dict:
    return json.loads(path.read_text())["benchmarks"]


def load_report(path: Path) -> dict:
    return json.loads(path.read_text())


def recompute_medians(report: dict) -> dict[str, dict[str, float]]:
    medians = {}
    for benchmark, payload in report["benchmarks"].items():
        benchmark_medians = {}
        sample_metrics = payload["samples"][0]["metrics"]
        for metric in sample_metrics:
            values = [sample["metrics"][metric] for sample in payload["samples"]]
            benchmark_medians[metric] = statistics.median(values)
        medians[benchmark] = benchmark_medians
    return medians


def ensure_recorded_medians(report: dict, recomputed: dict[str, dict[str, float]]) -> None:
    for benchmark, metrics in recomputed.items():
        recorded = report["benchmarks"][benchmark]["medians"]
        for metric, value in metrics.items():
            if not math.isclose(recorded[metric], value, rel_tol=1e-9, abs_tol=1e-9):
                raise ValueError(
                    f"{benchmark}:{metric} recorded median {recorded[metric]} != recomputed {value}"
                )


def regression_pct(baseline: float, candidate: float, lower_is_better: bool = False) -> float:
    if baseline == 0:
        return 0.0
    if lower_is_better:
        return max(0.0, ((candidate - baseline) / baseline) * 100.0)
    return max(0.0, ((baseline - candidate) / baseline) * 100.0)


def compare_reports(
    baseline: dict,
    candidate: dict,
    selection: list[str],
    thresholds: dict,
    baseline_mode: bool,
) -> tuple[list[dict], list[str]]:
    baseline_medians = recompute_medians(baseline)
    candidate_medians = recompute_medians(candidate)
    ensure_recorded_medians(baseline, baseline_medians)
    ensure_recorded_medians(candidate, candidate_medians)

    summary = []
    failures = []
    for benchmark in selection:
        config = thresholds[benchmark]
        for metric in config["metrics"]:
            base_value = baseline_medians[benchmark][metric]
            candidate_value = candidate_medians[benchmark][metric]
            delta = regression_pct(base_value, candidate_value, metric == "duration_sec")
            if baseline_mode:
                limit = config["baseline_verify_pct"]
                status = "fail" if delta > limit else "pass"
                if status == "fail":
                    failures.append(
                        f"{benchmark}:{metric} regressed {delta:.2f}% (> {limit:.2f}% baseline_verify_pct)"
                    )
            else:
                warn = config["warn_regression_pct"]
                fail = config["fail_regression_pct"]
                status = "fail" if delta > fail else "warn" if delta > warn else "pass"
                if status == "fail":
                    failures.append(
                        f"{benchmark}:{metric} regressed {delta:.2f}% (> {fail:.2f}% fail_regression_pct)"
                    )
            summary.append(
                {
                    "benchmark": benchmark,
                    "metric": metric,
                    "baseline": base_value,
                    "candidate": candidate_value,
                    "regression_pct": delta,
                    "status": status,
                }
            )
    return summary, failures


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.add_argument("--baseline", required=True)
    parser.add_argument("--candidate", required=True)
    selection_group = parser.add_mutually_exclusive_group(required=True)
    selection_group.add_argument("--selection")
    selection_group.add_argument("--bench", action="append")
    parser.add_argument("--thresholds", required=True)
    parser.add_argument("--mode", choices=("baseline-verify", "runtime"), default="runtime")
    parser.add_argument("--output")
    return parser


def resolve_selection(args: argparse.Namespace) -> list[str]:
    if args.selection is not None:
        return load_selection(Path(args.selection))
    return args.bench


def main() -> int:
    args = build_parser().parse_args()
    selection = resolve_selection(args)
    thresholds = load_thresholds(Path(args.thresholds))
    baseline = load_report(Path(args.baseline))
    candidate = load_report(Path(args.candidate))

    summary, failures = compare_reports(
        baseline,
        candidate,
        selection,
        thresholds,
        baseline_mode=args.mode == "baseline-verify",
    )
    payload = {"mode": args.mode, "comparisons": summary}
    rendered = json.dumps(payload, indent=2, sort_keys=True) + "\n"

    if args.output:
        Path(args.output).write_text(rendered)
    else:
        sys.stdout.write(rendered)

    if failures:
        for failure in failures:
            print(failure, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
