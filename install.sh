#!/usr/bin/env bash
# Installs the latest terminion release for Linux/macOS.
# Usage: curl -fsSL https://raw.githubusercontent.com/hameedibrh/Terminion/main/install.sh | bash
set -euo pipefail

REPO="hameedibrh/Terminion"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Linux) target="x86_64-unknown-linux-gnu" ;;
  Darwin)
    if [ "$arch" = "arm64" ]; then
      target="aarch64-apple-darwin"
    else
      target="x86_64-apple-darwin"
    fi
    ;;
  *) echo "Unsupported OS: $os" >&2; exit 1 ;;
esac

url="https://github.com/${REPO}/releases/latest/download/terminion-${target}.tar.gz"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

echo "Downloading $url"
curl -fsSL "$url" -o "$tmp_dir/terminion.tar.gz"
tar xzf "$tmp_dir/terminion.tar.gz" -C "$tmp_dir"

mkdir -p "$INSTALL_DIR"
mv "$tmp_dir/terminion" "$INSTALL_DIR/terminion"
chmod +x "$INSTALL_DIR/terminion"

echo "Installed terminion to $INSTALL_DIR/terminion"
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *) echo "Add $INSTALL_DIR to your PATH to use 'terminion' directly." ;;
esac
