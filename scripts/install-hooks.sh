#!/bin/bash
# Menyalin semua hook dari template ke folder hooks live-build di VM.
# Aman dijalankan berulang; tidak menyentuh symlink hook default.
set -e
cd "$(dirname "$0")/.."

mkdir -p config/hooks/live
cp -f config/hooks-templates/*.hook.chroot config/hooks/live/
sudo chmod +x config/hooks/live/*.hook.chroot

echo "OK: hook terpasang:"
ls -1 config/hooks/live/*.hook.chroot
