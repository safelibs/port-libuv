#!/usr/bin/env python3
import argparse
import sys
import tomllib
from pathlib import Path


PHASE_ORDER = [
    "impl_02_legacy_build_and_test_harness",
    "impl_03_helpers_threads_os",
    "impl_04_loop_core",
    "impl_05_network_backend",
    "impl_06_fs_dns_process_threadpool",
    "impl_07_packaging_and_link_compat",
    "impl_08_final_stabilize",
]
PHASE_INDEX = {phase: index for index, phase in enumerate(PHASE_ORDER)}
MIXED_UNITS = {
    "/home/yans/code/safelibs/ported/libuv/original/src/uv-common.c",
    "/home/yans/code/safelibs/ported/libuv/original/src/unix/core.c",
    "/home/yans/code/safelibs/ported/libuv/original/src/unix/linux.c",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source-manifest", required=True)
    parser.add_argument("--plan", required=True)
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--expect-complete-through", required=True)
    return parser.parse_args()


def read_source_manifest(path: Path) -> list[str]:
    sources = [line.strip() for line in path.read_text().splitlines() if line.strip()]
    if sources != sorted(set(sources)):
        raise ValueError("source manifest must be sorted and unique")
    return sources


def main() -> int:
    args = parse_args()
    source_manifest = read_source_manifest(Path(args.source_manifest))
    legacy_manifest = tomllib.loads(Path(args.manifest).read_text())
    plan = tomllib.loads(Path(args.plan).read_text())
    boundary = args.expect_complete_through

    if boundary not in PHASE_INDEX:
        raise ValueError(f"unknown phase boundary {boundary!r}")

    manifest_sources = legacy_manifest.get("sources")
    if manifest_sources is None or not isinstance(manifest_sources, list):
        raise ValueError("legacy manifest must define sources = [ ... ]")
    if manifest_sources != sorted(set(manifest_sources)):
        raise ValueError("legacy manifest sources must be sorted and unique")

    units = plan.get("unit")
    if units is None or not isinstance(units, list):
        raise ValueError("transition plan must define [[unit]] tables")

    plan_sources = []
    expected_legacy_sources = []

    for unit in units:
        source = unit.get("source")
        groups = unit.get("group")
        if not isinstance(source, str):
            raise ValueError("each unit must define a source path")
        if not isinstance(groups, list) or not groups:
            raise ValueError(f"{source}: missing [[unit.group]] entries")
        plan_sources.append(source)

        if len(groups) > 1 and source not in MIXED_UNITS:
            raise ValueError(f"{source}: only approved mixed units may have multiple groups")
        if len(groups) == 1 and source not in MIXED_UNITS:
            if groups[0].get("symbols") != ["*"]:
                raise ValueError(f"{source}: unsplit units must use symbols = [\"*\"]")

        group_names = [group.get("name") for group in groups]
        if group_names != sorted(group_names):
            raise ValueError(f"{source}: groups must be sorted by name")
        if len(group_names) != len(set(group_names)):
            raise ValueError(f"{source}: group names must be unique")

        legacy_present = False
        for group in groups:
            for key in ("name", "owner_phase", "retirement_phase", "status", "symbols"):
                if key not in group:
                    raise ValueError(f"{source}: group missing {key}")
            owner_phase = group["owner_phase"]
            retirement_phase = group["retirement_phase"]
            status = group["status"]
            symbols = group["symbols"]

            if owner_phase not in PHASE_INDEX or retirement_phase not in PHASE_INDEX:
                raise ValueError(f"{source}:{group['name']}: invalid phase assignment")
            if PHASE_INDEX[retirement_phase] < PHASE_INDEX[owner_phase]:
                raise ValueError(f"{source}:{group['name']}: retirement_phase precedes owner_phase")
            if status not in {"legacy", "rust"}:
                raise ValueError(f"{source}:{group['name']}: invalid status {status!r}")
            if not isinstance(symbols, list) or not symbols or not all(isinstance(symbol, str) for symbol in symbols):
                raise ValueError(f"{source}:{group['name']}: symbols must be a non-empty string list")

            should_be_rust = PHASE_INDEX[owner_phase] <= PHASE_INDEX[boundary]
            expected_status = "rust" if should_be_rust else "legacy"
            if status != expected_status:
                raise ValueError(
                    f"{source}:{group['name']}: expected status {expected_status!r} at {boundary}, found {status!r}"
                )
            legacy_present |= status == "legacy"

        if legacy_present:
            expected_legacy_sources.append(source)

    if plan_sources != sorted(plan_sources):
        raise ValueError("transition plan units must be sorted by source")
    if plan_sources != source_manifest:
        raise ValueError("transition plan sources must match the source manifest exactly")
    if manifest_sources != expected_legacy_sources:
        raise ValueError("legacy manifest sources do not match the transition plan status boundary")

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ValueError as error:
        print(error, file=sys.stderr)
        raise SystemExit(1)
