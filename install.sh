#!/usr/bin/env bash
# Installs terminion for Linux/macOS.
# Per-user (default, no sudo required):
#   curl -fsSL https://raw.githubusercontent.com/hameedibrh/Terminion/main/install.sh | bash
# System-wide, for all users (requires sudo):
#   curl -fsSL https://raw.githubusercontent.com/hameedibrh/Terminion/main/install.sh | sudo bash -s -- --global
set -euo pipefail

REPO="hameedibrh/Terminion"

global=0
for arg in "$@"; do
  case "$arg" in
    --global) global=1 ;;
  esac
done

if [ "$global" = "1" ]; then
  if [ "$(id -u)" -ne 0 ]; then
    echo "Global install requires root. Re-run as:" >&2
    echo "  curl -fsSL https://raw.githubusercontent.com/${REPO}/main/install.sh | sudo bash -s -- --global" >&2
    exit 1
  fi
  default_dir="/usr/local/bin"
else
  default_dir="$HOME/.local/bin"
fi
INSTALL_DIR="${INSTALL_DIR:-$default_dir}"

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

# GitHub's "/releases/latest" shortcut only ever resolves to the newest
# *stable* release, so it 404s while every published release is a
# pre-release (e.g. an alpha). Resolve the newest release of any kind
# (including pre-releases) via the API instead.
tag="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases" \
  | grep -m1 '"tag_name"' \
  | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')"

if [ -z "$tag" ]; then
  echo "No releases found for ${REPO} yet." >&2
  exit 1
fi

url="https://github.com/${REPO}/releases/download/${tag}/terminion-${target}.tar.gz"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

echo "Installing terminion ${tag}$([ "$global" = "1" ] && echo ' (system-wide)')"
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
