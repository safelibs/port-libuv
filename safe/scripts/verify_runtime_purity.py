#!/usr/bin/env python3
import argparse
import json
import sys
import tomllib
from pathlib import Path


NON_RUST_EXTENSIONS = {".c", ".cc", ".cpp", ".s", ".S"}
BUILD_MANIFEST_SCHEMA_VERSION = 2
MERGED_LIBRARY_TARGET_MARKERS = (
    "CMakeFiles/uv.dir/",
    "CMakeFiles/uv_a.dir/",
    "CMakeFiles/uv_test_legacy_support.dir/",
    "CMakeFiles/uv_test_phase5_private_support.dir/",
    "CMakeFiles/uv_test_phase5_forwarders.dir/",
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--build-dir", required=True)
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--mode", choices=("allow-legacy", "rust-only"), required=True)
    return parser.parse_args()


def require_sorted_unique_absolute(values: list[str], label: str) -> set[str]:
    if values != sorted(set(values)):
        raise ValueError(f"{label} must be sorted and unique")
    if any(not Path(value).is_absolute() for value in values):
        raise ValueError(f"{label} must contain absolute paths")
    return set(values)


def read_merged_library_compile_entries(compile_commands_path: Path) -> list[str]:
    entries = json.loads(compile_commands_path.read_text())
    result = []
    for entry in entries:
        output = entry.get("output", "")
        source = entry.get("file")
        if not isinstance(output, str) or not isinstance(source, str):
            continue
        if not output.startswith(MERGED_LIBRARY_TARGET_MARKERS):
            continue
        if Path(source).suffix in NON_RUST_EXTENSIONS:
            result.append(source)
    return sorted(set(result))


def main() -> int:
    args = parse_args()
    build_dir = Path(args.build_dir)
    manifest_path = Path(args.manifest)
    compile_commands_path = build_dir / "compile_commands.json"
    build_manifest_path = build_dir / "libuv-build-manifest.json"

    legacy_manifest = tomllib.loads(manifest_path.read_text())
    build_manifest = json.loads(build_manifest_path.read_text())
    compile_sources = read_merged_library_compile_entries(compile_commands_path)

    legacy_sources = legacy_manifest.get("sources")
    if not isinstance(legacy_sources, list):
        raise ValueError("legacy manifest must define sources = [ ... ]")
    legacy_source_set = require_sorted_unique_absolute(legacy_sources, "legacy manifest sources")

    schema_version = build_manifest.get("schema_version")
    if schema_version != BUILD_MANIFEST_SCHEMA_VERSION:
        raise ValueError(
            f"build manifest schema_version must be {BUILD_MANIFEST_SCHEMA_VERSION}, found {schema_version!r}"
        )

    manifest_sources = build_manifest.get("legacy_manifest_sources")
    production_sources = build_manifest.get("production_non_rust_sources")
    generated_sources = build_manifest.get("generated_production_non_rust_sources")
    if not isinstance(manifest_sources, list):
        raise ValueError("build manifest missing legacy_manifest_sources")
    if not isinstance(production_sources, list):
        raise ValueError("build manifest missing production_non_rust_sources")
    if not isinstance(generated_sources, list):
        raise ValueError("build manifest missing generated_production_non_rust_sources")

    manifest_source_set = require_sorted_unique_absolute(manifest_sources, "build manifest legacy_manifest_sources")
    production_source_set = require_sorted_unique_absolute(
        production_sources, "build manifest production_non_rust_sources"
    )
    generated_source_set = require_sorted_unique_absolute(
        generated_sources, "build manifest generated_production_non_rust_sources"
    )

    compile_source_set = set(compile_sources)
    declared_non_rust_sources = production_source_set | generated_source_set

    if args.mode == "allow-legacy":
        if manifest_source_set != legacy_source_set:
            raise ValueError("build manifest legacy_manifest_sources must match the checked-in manifest")
        if production_source_set != legacy_source_set:
            raise ValueError(
                "build manifest production_non_rust_sources must match the checked-in legacy manifest"
            )
        missing_compile_entries = declared_non_rust_sources - compile_source_set
        unexpected_compile_entries = compile_source_set - declared_non_rust_sources
        if missing_compile_entries or unexpected_compile_entries:
            details = []
            if missing_compile_entries:
                details.append(
                    "missing-from-compile-commands=" + ", ".join(sorted(missing_compile_entries))
                )
            if unexpected_compile_entries:
                details.append(
                    "unexpected-in-compile-commands=" + ", ".join(sorted(unexpected_compile_entries))
                )
            raise ValueError("merged non-Rust library inputs drifted from the build manifest: " + "; ".join(details))
        return 0

    if legacy_sources:
        raise ValueError("rust-only mode requires safe/legacy/linux-manifest.toml to have sources = []")
    if manifest_sources:
        raise ValueError("rust-only mode requires an empty legacy_manifest_sources array")
    if production_sources:
        raise ValueError("rust-only mode requires an empty production_non_rust_sources array")
    if generated_sources:
        raise ValueError("rust-only mode requires an empty generated_production_non_rust_sources array")
    if compile_sources:
        raise ValueError(
            "rust-only mode forbids library-target C/C++/assembly compile entries: "
            + ", ".join(compile_sources)
        )

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ValueError as error:
        print(error, file=sys.stderr)
        raise SystemExit(1)
