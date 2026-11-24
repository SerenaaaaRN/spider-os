#!/bin/bash
# Menyiapkan splash, judul, dan warna boot loader di VM. TANPA lb build.
# Dijalankan setelah lb config membuat config/bootloaders.
set -e
cd "$(dirname "$0")/.."

if [ ! -d config/bootloaders ]; then
    cp -r /usr/share/live/build/bootloaders config/bootloaders
fi

# 1. Splash: ISOLINUX 640x480, GRUB 1920x1080.
while IFS= read -r -d '' f; do
    case "$f" in
        *isolinux*|*syslinux*)
            cp -f assets-final/isolinux-splash.png "$f"
            ;;
        *grub*)
            cp -f assets-final/grub-bg.png "$f"
            ;;
    esac
done < <(find config/bootloaders -type f -iname 'splash.png' -print0)

# 2. Judul menu.
find config/bootloaders -type f \( -name '*.cfg' -o -name '*.cfg.in' -o -name '*.inc' \) \
    -exec sed -i 's/Debian GNU\/Linux/SpiderOS/g' {} + 2>/dev/null || true

# 3. Warna menu ISOLINUX (ditambahkan kalau belum ada).
MENU=$(find config/bootloaders -type f -iname 'stdmenu.cfg*' | head -n 1 || true)
if [ -n "$MENU" ] && ! grep -q '#ff870000' "$MENU"; then
    cat >> "$MENU" <<'EOF'

menu color border   0 #ff870000 #00000000 none
menu color title    1;37;40 #ffd7af #00000000 none
menu color sel      7;37;40 #ffffffff #ff870000 all
menu color unsel    37;40 #ffd7af #00000000 none
EOF
fi

# 4. Warna menu GRUB (GRUB hanya mengenal nama warna).
find config/bootloaders -path '*grub*' -type f \( -name '*.cfg' -o -name '*.cfg.in' \) \
    -exec sed -i \
      -e 's/^set menu_color_normal=.*/set menu_color_normal=white\/black/' \
      -e 's/^set menu_color_highlight=.*/set menu_color_highlight=white\/red/' {} + 2>/dev/null || true

echo "OK: boot loader siap."
find config/bootloaders -type f -iname 'splash*' -print
