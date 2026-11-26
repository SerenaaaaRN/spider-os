#!/bin/bash
# Pemakaian: ./scripts/build.sh v0.1
# Dijalankan di VM Debian / container Debian trixie, dari root repo.
set -euo pipefail
VER="${1:-dev}"
cd "$(dirname "$0")/.."
./scripts/stage-spider-sense.sh
sudo lb clean
sudo lb config
sudo lb build 2>&1 | tee build.log

# Pastikan ISO benar-benar ada sebelum dipindah. Tanpa pemeriksaan ini,
# kegagalan lb build hanya muncul sebagai error "mv: cannot stat ''".
ISO=$(find . -maxdepth 1 -type f -name 'live-image-*.iso' -print -quit)
if [ -z "$ISO" ]; then
    echo "ERROR: lb build selesai tetapi tidak ada live-image-*.iso. Lihat build.log." >&2
    tail -n 80 build.log >&2 2>/dev/null || true
    exit 1
fi

OUT="dist/spideros-${VER}.iso"
mkdir -p dist
mv "$ISO" "$OUT"
sha256sum "$OUT" | tee "${OUT}.sha256"
ls -lh "$OUT"
