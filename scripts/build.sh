#!/bin/bash
# Pemakaian: ./scripts/build.sh v0.1
# Dijalankan di VM Debian, dari root repo (~/spider-os-build).
set -e
VER="${1:-dev}"
cd "$(dirname "$0")/.."
sudo lb clean
sudo lb config
sudo lb build 2>&1 | tee build.log
ISO=$(ls -1 live-image-*.iso | head -n1)
OUT="dist/spideros-${VER}.iso"
mv "$ISO" "$OUT"
sha256sum "$OUT" | tee "${OUT}.sha256"
ls -lh "$OUT"
