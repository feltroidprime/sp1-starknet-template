#!/usr/bin/env bash
set -euo pipefail
version=6.10.0
case "$(uname -s)-$(uname -m)" in
    Linux-x86_64) platform=linux_amd64 ;;
    Linux-aarch64) platform=linux_arm64 ;;
    Darwin-arm64) platform=darwin_arm64 ;;
    Darwin-x86_64) platform=darwin_amd64 ;;
    *) printf 'Unsupported SP1 platform\n' >&2; exit 1 ;;
esac
install_dir="${SP1_INSTALL_DIR:-$HOME/.sp1/bin}"
mkdir -p "$install_dir"
archive=$(mktemp)
trap 'rm -f "$archive"' EXIT
curl --fail --location --retry 3 "https://github.com/succinctlabs/sp1/releases/download/v${version}/cargo_prove_v${version}_${platform}.tar.gz" --output "$archive"
tar -xzf "$archive" -C "$install_dir" cargo-prove
"$install_dir/cargo-prove" prove install-toolchain
"$install_dir/cargo-prove" prove --version
