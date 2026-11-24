#!/bin/bash
# Menyalin artefak build dari repo native VM ke shared folder Windows.
# Dijalankan di VM setelah build, BUKAN di Windows.
# Build tetap harus di ~/spider-os-build (bukan /mnt/hgfs).
set -e

SRC="$HOME/spider-os-build"
DST="/mnt/hgfs/SpiderOS-Share/spider-os"

if [ ! -d "$DST" ]; then
    echo "ERROR: shared folder tidak ditemukan: $DST" >&2
    exit 1
fi

mkdir -p "$DST/dist" "$DST/docs/screenshots"

# Pakai cp -f, bukan cp -a: vmhgfs tidak mendukung ownership POSIX.
cp -f "$SRC"/dist/*.iso         "$DST/dist/" 2>/dev/null || true
cp -f "$SRC"/dist/*.sha256      "$DST/dist/" 2>/dev/null || true
cp -f "$SRC"/build.log          "$DST/"       2>/dev/null || true
cp -f "$SRC"/docs/screenshots/* "$DST/docs/screenshots/" 2>/dev/null || true

echo "OK: artefak disalin ke $DST"
