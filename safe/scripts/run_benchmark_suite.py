#!/usr/bin/env python3
import argparse
import json
import re
import statistics
import subprocess
import sys
from pathlib import Path


def load_selection(path: Path) -> list[str]:
    return [line.strip() for line in path.read_text().splitlines() if line.strip()]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--runner", required=True)
    parser.add_argument("--selection", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--runs", type=int, default=5)
    parser.add_argument("--warmup", type=int, default=1)
    return parser.parse_args()


def parse_output(benchmark: str, lines: list[str]) -> dict[str, float]:
    text = "\n".join(lines)

    def match(pattern: str) -> re.Match[str]:
        result = re.search(pattern, text, re.MULTILINE)
        if result is None:
            raise ValueError(f"{benchmark}: output did not match {pattern!r}")
        return result

    def number(value: str) -> float:
        return float(value.replace(",", ""))

    if benchmark == "loop_count":
        return {
            "ticks_per_sec": number(match(r"loop_count:\s+[\d,]+ ticks in [0-9.]+s \(([\d,]+)/s\)").group(1)),
        }
    if benchmark == "async1":
        return {
            "ops_per_sec": number(match(r"async1:\s+[0-9.]+ sec \(([\d,]+)/sec\)").group(1)),
        }
    if benchmark == "queue_work":
        return {
            "jobs_per_sec": number(match(r"[\d,]+ async jobs in [0-9.]+ seconds \(([\d,]+)/s\)").group(1)),
        }
    if benchmark == "getaddrinfo":
        return {
            "requests_per_sec": number(match(r"getaddrinfo:\s+([\d,]+) req/s").group(1)),
        }
    if benchmark == "ping_pongs":
        return {
            "roundtrips_per_sec": number(match(r"ping_pongs:\s+([\d,]+) roundtrips/s").group(1)),
        }
    if benchmark == "ping_udp1":
        return {
            "roundtrips_per_sec": number(match(r"ping_pongs:\s+\d+ pingers, ~\s*([\d,]+) roundtrips/s").group(1)),
        }
    if benchmark == "fs_stat":
        metrics = {}
        sync_match = match(r"stats \(sync\): [0-9.]+s \(([\d,]+)/s\)")
        metrics["sync_ops_per_sec"] = number(sync_match.group(1))
        for concurrency in (4, 16):
            concurrency_match = match(
                rf"stats \({concurrency} concurrent\): [0-9.]+s \(([\d,]+)/s\)"
            )
            metrics[f"concurrency_{concurrency}_ops_per_sec"] = number(concurrency_match.group(1))
        return metrics
    if benchmark == "tcp_write_batch":
        matched = match(r"([\d,]+) write requests in ([0-9.]+)s\.")
        writes = number(matched.group(1))
        seconds = number(matched.group(2))
        return {
            "duration_sec": seconds,
            "writes_per_sec": writes / seconds,
        }

    raise ValueError(f"unsupported benchmark {benchmark!r}")


def run_benchmark(runner: Path, benchmark: str) -> dict:
    last_error = None
    for attempt in (1, 2):
        proc = subprocess.run(
            [str(runner), benchmark],
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            check=False,
            timeout=180,
        )
        lines = proc.stdout.splitlines()
        if proc.returncode == 0:
            try:
                metrics = parse_output(benchmark, lines)
            except ValueError as error:
                last_error = f"{benchmark}: {error}"
            else:
                return {
                    "attempt": attempt,
                    "output_lines": lines,
                    "metrics": metrics,
                }
        else:
            last_error = f"{benchmark}: attempt {attempt} exited with {proc.returncode}"

    raise RuntimeError(f"{last_error}; benchmark marked unstable")


def compute_medians(samples: list[dict]) -> dict[str, float]:
    metrics = sorted(samples[0]["metrics"])
    medians = {}
    for metric in metrics:
        medians[metric] = statistics.median(sample["metrics"][metric] for sample in samples)
    return medians


def main() -> int:
    args = parse_args()
    runner = Path(args.runner).resolve()
    selection_path = Path(args.selection)
    output_path = Path(args.output)
    selection = load_selection(selection_path)

    report = {
        "runner": str(runner),
        "selection": selection,
        "runs": args.runs,
        "warmup": args.warmup,
        "benchmarks": {},
    }

    for benchmark in selection:
        for _ in range(args.warmup):
            run_benchmark(runner, benchmark)

        samples = [run_benchmark(runner, benchmark) for _ in range(args.runs)]
        report["benchmarks"][benchmark] = {
            "samples": samples,
            "medians": compute_medians(samples),
        }

    output_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    return 0


if __name__ == "__main__":
    sys.exit(main())
