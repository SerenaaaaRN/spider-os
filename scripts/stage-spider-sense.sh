#!/bin/bash
# Menyalin sumber Spider-Sense ke staging chroot sebelum hook 0400 dijalankan.
# Dipanggil dari scripts/build.sh tepat setelah `cd "$(dirname "$0")/.."`.
set -e

cd "$(dirname "$0")/.."

SRC="tools/spider-sense"
DST="config/includes.chroot_after_packages/opt/spider-sense-src"

if [ ! -f "$SRC/Cargo.toml" ]; then
    echo "ERROR: sumber Rust tidak ditemukan di $SRC" >&2
    exit 1
fi

rm -rf "$DST"
mkdir -p "$DST"
cp -R "$SRC"/. "$DST"/
rm -rf "$DST/target" "$DST/.git"

echo "OK: sumber Spider-Sense disiapkan di $DST"
