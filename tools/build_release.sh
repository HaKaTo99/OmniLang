#!/usr/bin/env bash
# OmniLang Linux/macOS Release Build Script
# Usage: ./tools/build_release.sh

set -e

VERSION="2.2.0"
TARGET_DIR="target/release"
RELEASE_DIR="omnilang-v$VERSION-linux-x64"
OS_NAME=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

echo "🚀 Membangun Standalone Binary OmniLang v$VERSION untuk $OS_NAME ($ARCH)..."

# 1. Build The Release Binary
cargo build --release

# 2. Setup Release Directory
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# 3. Copy Output Binary & Essential Files
echo "📦 Menyalin bundel biner dan dokumen pelengkap..."
cp "$TARGET_DIR/omnilang" "$RELEASE_DIR/"
cp README.md "$RELEASE_DIR/"
cp docs/QUICKSTART.md "$RELEASE_DIR/"
cp -r examples "$RELEASE_DIR/"

# 4. Create Archive
echo "🗜️ Membuat arsip rilis..."
tar -czvf "omnilang-v$VERSION-$OS_NAME-$ARCH.tar.gz" "$RELEASE_DIR"

echo "✅ Berhasil! Rilis standalone siap di: omnilang-v$VERSION-$OS_NAME-$ARCH.tar.gz"
