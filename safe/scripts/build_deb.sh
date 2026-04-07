#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
safe_dir="$(cd -- "$script_dir/.." && pwd)"
repo_root="$(cd -- "$safe_dir/.." && pwd)"
dist_dir="$safe_dir/dist"

export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable-x86_64-unknown-linux-gnu}"
export PATH="$CARGO_HOME/bin:$HOME/.local/bin:/usr/bin:/usr/sbin:/bin:/sbin:$PATH"
export DEB_BUILD_OPTIONS="${DEB_BUILD_OPTIONS:+$DEB_BUILD_OPTIONS }parallel=$(nproc)"

mkdir -p "$dist_dir"

(
  cd "$safe_dir"
  if [[ -x debian/rules ]]; then
    debian/rules clean >/dev/null || true
  fi
)

rm -f "$dist_dir"/libuv1t64_*.deb "$dist_dir"/libuv1-dev_*.deb
rm -f "$repo_root"/libuv1t64_*.deb "$repo_root"/libuv1-dev_*.deb
rm -f "$repo_root"/libuv1t64-dbgsym_*.ddeb
rm -f "$repo_root"/libuv1_*.buildinfo "$repo_root"/libuv1_*.changes

(
  cd "$safe_dir"
  dpkg-buildpackage -us -uc -b -d
)

runtime_debs=("$repo_root"/libuv1t64_*.deb)
dev_debs=("$repo_root"/libuv1-dev_*.deb)

if [[ ! -e "${runtime_debs[0]}" || ! -e "${dev_debs[0]}" ]]; then
  printf 'expected runtime and development .deb outputs under %s\n' "$repo_root" >&2
  exit 1
fi

cp "${runtime_debs[@]}" "$dist_dir/"
cp "${dev_debs[@]}" "$dist_dir/"

(
  cd "$safe_dir"
  debian/rules clean >/dev/null
)

rm -f "$repo_root"/libuv1t64_*.deb "$repo_root"/libuv1-dev_*.deb
rm -f "$repo_root"/libuv1t64-dbgsym_*.ddeb
rm -f "$repo_root"/libuv1_*.buildinfo "$repo_root"/libuv1_*.changes
