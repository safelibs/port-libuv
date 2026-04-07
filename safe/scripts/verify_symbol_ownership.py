#!/usr/bin/env python3
import argparse
import sys
import tomllib
from pathlib import Path


PHASE_ORDER = [
    "impl_01_audit_existing_safe_workspace",
    "impl_02_remove_network_backend_c_support",
    "impl_03_remove_runtime_c_support",
    "impl_04_constrain_unsafe_boundaries",
    "impl_05_restore_link_compat_and_package_surface",
    "impl_06_add_dependents_image_and_run_matrix",
    "impl_07_final_stabilize",
]
PHASE_INDEX = {phase: index for index, phase in enumerate(PHASE_ORDER)}
SPLIT_UNITS = {
    "/home/yans/code/safelibs/ported/libuv/original/src/uv-common.c": {
        "fs_scandir_shutdown": "impl_03_remove_runtime_c_support",
        "helpers_accessors": "impl_03_remove_runtime_c_support",
        "loop_default_metrics": "impl_03_remove_runtime_c_support",
        "stream_io_buffers": "impl_02_remove_network_backend_c_support",
    },
    "/home/yans/code/safelibs/ported/libuv/original/src/unix/core.c": {
        "close_run_backend_watchers": "impl_03_remove_runtime_c_support",
        "os_helpers_and_env": "impl_03_remove_runtime_c_support",
        "socket_fileno_helpers": "impl_02_remove_network_backend_c_support",
    },
    "/home/yans/code/safelibs/ported/libuv/original/src/unix/linux.c": {
        "epoll_fs_event_socket_watchers": "impl_02_remove_network_backend_c_support",
        "io_uring_fs_random_process_system_info": "impl_03_remove_runtime_c_support",
    },
}
PHASE1_TO_PHASE3 = {
    "impl_01_audit_existing_safe_workspace",
    "impl_02_remove_network_backend_c_support",
    "impl_03_remove_runtime_c_support",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source-manifest", required=True)
    parser.add_argument("--ownership", required=True)
    parser.add_argument("--legacy-manifest", required=True)
    parser.add_argument("--expect-complete-through", required=True)
    return parser.parse_args()


def require_sorted_unique_absolute(values: list[str], label: str) -> list[str]:
    if values != sorted(set(values)):
        raise ValueError(f"{label} must be sorted and unique")
    if any(not Path(value).is_absolute() for value in values):
        raise ValueError(f"{label} must contain absolute paths")
    return values


def read_sorted_lines(path: Path) -> list[str]:
    return require_sorted_unique_absolute(
        [line.strip() for line in path.read_text().splitlines() if line.strip()],
        "source manifest",
    )


def expected_status_for_phase(owner_phase: str, boundary: str) -> str:
    return "rust" if PHASE_INDEX[owner_phase] <= PHASE_INDEX[boundary] else "legacy"


def main() -> int:
    args = parse_args()
    boundary = args.expect_complete_through
    if boundary not in PHASE_INDEX:
        raise ValueError(f"unknown phase boundary {boundary!r}")

    source_manifest = read_sorted_lines(Path(args.source_manifest))
    legacy_manifest = tomllib.loads(Path(args.legacy_manifest).read_text())
    ownership = tomllib.loads(Path(args.ownership).read_text())

    manifest_sources = legacy_manifest.get("sources")
    if not isinstance(manifest_sources, list):
        raise ValueError("legacy manifest must define sources = [ ... ]")
    require_sorted_unique_absolute(manifest_sources, "legacy manifest sources")

    units = ownership.get("unit")
    if not isinstance(units, list) or not units:
        raise ValueError("ownership file must define [[unit]] tables")

    expected_legacy_sources = []
    ownership_sources = []

    for unit in units:
        if set(unit) != {"source", "group"}:
            raise ValueError("each [[unit]] must only define source and group")

        source = unit.get("source")
        groups = unit.get("group")
        if not isinstance(source, str):
            raise ValueError("each [[unit]] must define source as a string")
        if not Path(source).is_absolute():
            raise ValueError(f"{source!r}: source must be absolute")
        if not isinstance(groups, list) or not groups:
            raise ValueError(f"{source}: missing [[unit.group]] entries")

        ownership_sources.append(source)

        split_expectations = SPLIT_UNITS.get(source)
        if split_expectations is None and len(groups) != 1:
            raise ValueError(f"{source}: only approved split units may have multiple groups")
        if split_expectations is not None and len(groups) <= 1:
            raise ValueError(f"{source}: split units must keep explicit per-group ownership")

        group_names = []
        source_is_legacy = False
        for group in groups:
            if set(group) != {"name", "owner_phase", "retirement_phase", "status", "symbols"}:
                raise ValueError(
                    f"{source}: each [[unit.group]] must only define "
                    "name, owner_phase, retirement_phase, status, and symbols"
                )

            name = group["name"]
            owner_phase = group["owner_phase"]
            retirement_phase = group["retirement_phase"]
            status = group["status"]
            symbols = group["symbols"]

            if not isinstance(name, str) or not name:
                raise ValueError(f"{source}: group name must be a non-empty string")
            if owner_phase not in PHASE_INDEX:
                raise ValueError(f"{source}:{name}: invalid owner_phase {owner_phase!r}")
            if retirement_phase not in PHASE_INDEX:
                raise ValueError(f"{source}:{name}: invalid retirement_phase {retirement_phase!r}")
            if PHASE_INDEX[retirement_phase] < PHASE_INDEX[owner_phase]:
                raise ValueError(f"{source}:{name}: retirement_phase precedes owner_phase")
            if owner_phase not in PHASE1_TO_PHASE3 or retirement_phase not in PHASE1_TO_PHASE3:
                raise ValueError(
                    f"{source}:{name}: phase-1 ownership must only point at implement phases 01 through 03"
                )
            if retirement_phase != owner_phase:
                raise ValueError(f"{source}:{name}: retirement_phase must match owner_phase in phase 1")
            if status not in {"legacy", "rust"}:
                raise ValueError(f"{source}:{name}: invalid status {status!r}")
            if not isinstance(symbols, list) or not symbols or not all(isinstance(symbol, str) for symbol in symbols):
                raise ValueError(f"{source}:{name}: symbols must be a non-empty string list")
            if symbols != sorted(set(symbols)):
                raise ValueError(f"{source}:{name}: symbols must be sorted and unique")

            if split_expectations is None:
                if symbols != ["*"]:
                    raise ValueError(f"{source}: unsplit units must use symbols = [\"*\"]")
            else:
                expected_owner = split_expectations.get(name)
                if expected_owner is None:
                    raise ValueError(f"{source}:{name}: unexpected split-unit group")
                if owner_phase != expected_owner:
                    raise ValueError(
                        f"{source}:{name}: owner_phase must be {expected_owner!r}, found {owner_phase!r}"
                    )
                if symbols == ["*"]:
                    raise ValueError(f"{source}:{name}: split-unit groups must enumerate symbol patterns")

            expected_status = expected_status_for_phase(owner_phase, boundary)
            if status != expected_status:
                raise ValueError(
                    f"{source}:{name}: expected status {expected_status!r} at {boundary}, found {status!r}"
                )
            source_is_legacy |= status == "legacy"
            group_names.append(name)

        if group_names != sorted(group_names):
            raise ValueError(f"{source}: groups must be sorted by name")
        if len(group_names) != len(set(group_names)):
            raise ValueError(f"{source}: group names must be unique")
        if split_expectations is not None and group_names != sorted(split_expectations):
            raise ValueError(f"{source}: split-unit groups do not match the checked phase boundaries")

        if source_is_legacy:
            expected_legacy_sources.append(source)

    if ownership_sources != source_manifest:
        raise ValueError("ownership sources must match safe/abi/linux-source-manifest.txt exactly")
    if manifest_sources != expected_legacy_sources:
        raise ValueError("legacy manifest sources do not match the ownership status boundary")

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ValueError as error:
        print(error, file=sys.stderr)
        raise SystemExit(1)
