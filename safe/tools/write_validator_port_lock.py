#!/usr/bin/env python3
"""Write a validator port-mode .deb lock for the local libuv-safe build."""
from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
from pathlib import Path


LIBRARY_CANONICAL_PACKAGES = {
    "libuv": ["libuv1t64", "libuv1-dev"],
}
LOCK_GENERATED_AT = "1970-01-01T00:00:00Z"


def fail(message: str) -> "NoReturn":
    print(message, file=sys.stderr)
    raise SystemExit(1)


def parse_artifacts_env(path: Path) -> dict[str, str]:
    if not path.is_file():
        fail(f"missing artifacts.env: {path}")
    values: dict[str, str] = {}
    for raw_line in path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        if "=" not in line:
            fail(f"invalid artifacts.env line: {raw_line}")
        key, value = line.split("=", 1)
        values[key.strip()] = value.strip()
    return values


def dpkg_field(deb_path: Path, field: str) -> str:
    result = subprocess.run(
        ["dpkg-deb", "-f", str(deb_path), field],
        check=True,
        capture_output=True,
        text=True,
    )
    value = result.stdout.strip()
    if not value:
        fail(f"empty {field} field in {deb_path}")
    return value


def sha256_hex(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def build_deb_entry(deb_path: Path, expected_package: str) -> dict[str, object]:
    if not deb_path.is_file():
        fail(f"missing override deb: {deb_path}")
    package = dpkg_field(deb_path, "Package")
    architecture = dpkg_field(deb_path, "Architecture")
    if package != expected_package:
        fail(
            f"override deb {deb_path} has package {package!r}, expected {expected_package!r}"
        )
    if architecture not in {"amd64", "all"}:
        fail(f"override deb {deb_path} has unsupported architecture {architecture!r}")
    return {
        "package": package,
        "filename": deb_path.name,
        "architecture": architecture,
        "sha256": sha256_hex(deb_path),
        "size": deb_path.stat().st_size,
    }


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Write a validator port-mode lock describing locally built libuv-safe debs.",
    )
    parser.add_argument("--artifacts-env", required=True, type=Path)
    parser.add_argument("--override-root", required=True, type=Path)
    parser.add_argument("--lock-output", required=True, type=Path)
    parser.add_argument("--library", required=True)
    parser.add_argument("--repository", required=True)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--release-tag", required=True)
    parser.add_argument("--source-config", default="repositories.yml")
    parser.add_argument("--source-inventory", default="local-safe-dist")
    args = parser.parse_args()

    if args.library not in LIBRARY_CANONICAL_PACKAGES:
        fail(f"unsupported library: {args.library}")

    canonical_packages = LIBRARY_CANONICAL_PACKAGES[args.library]

    env = parse_artifacts_env(args.artifacts_env)
    runtime_src = env.get("LIBUV_SAFE_RUNTIME_DEB")
    dev_src = env.get("LIBUV_SAFE_DEV_DEB")
    if not runtime_src:
        fail("artifacts.env missing LIBUV_SAFE_RUNTIME_DEB")
    if not dev_src:
        fail("artifacts.env missing LIBUV_SAFE_DEV_DEB")

    library_override_dir = args.override_root / args.library
    if not library_override_dir.is_dir():
        fail(f"missing override root leaf: {library_override_dir}")

    runtime_override = library_override_dir / Path(runtime_src).name
    dev_override = library_override_dir / Path(dev_src).name

    debs_by_package = {
        canonical_packages[0]: build_deb_entry(runtime_override, canonical_packages[0]),
        canonical_packages[1]: build_deb_entry(dev_override, canonical_packages[1]),
    }
    ordered_debs = [debs_by_package[pkg] for pkg in canonical_packages]

    if not args.commit:
        fail("--commit must be a non-empty string")
    if not args.release_tag:
        fail("--release-tag must be a non-empty string")

    library_entry = {
        "library": args.library,
        "repository": args.repository,
        "url": f"https://github.com/{args.repository}",
        "tag_ref": f"refs/tags/{args.release_tag}",
        "commit": args.commit,
        "release_tag": args.release_tag,
        "debs": ordered_debs,
        "unported_original_packages": [],
    }

    lock = {
        "schema_version": 1,
        "mode": "port",
        "generated_at": LOCK_GENERATED_AT,
        "source_config": args.source_config,
        "source_inventory": args.source_inventory,
        "libraries": [library_entry],
    }

    args.lock_output.parent.mkdir(parents=True, exist_ok=True)
    args.lock_output.write_text(json.dumps(lock, indent=2, sort_keys=False) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
