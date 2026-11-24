===============================================================================
PERINGATAN: SEMUA PNG DI FOLDER INI MASIH DUMMY / PLACEHOLDER
===============================================================================

Dibuat otomatis oleh scripts/generate-assets.py untuk menutup Phase 4 lebih
dulu. JANGAN langsung dipakai untuk build final sebelum diganti.

GANTI SEBELUM BUILD PHASE 6/7 DI VM:
  [ ] wallpaper.png              1920x1080  - desktop
  [ ] login.png                  1920x1080  - layar login LightDM
  [ ] logo.png                   400x400 transparan - Plymouth & neofetch
  [ ] grub-bg.png                1920x1080  - boot menu UEFI (GRUB)
  [ ] isolinux-splash.png        TEPAT 640x480 - boot menu BIOS
  [ ] spideros-logo-small.png    128x128    - foto profil & ikon menu
  [ ] ../assets-src/logo.svg     sumber vektor final

SYARAT:
  1. Palet mengikuti docs/desain.md §2 (maroon #870000, rose #D78787,
     cream #FFD7AF, sand #D7AF87, dusk #AF8787, kanvas #17100F).
     TIDAK BOLEH ada biru.
  2. Tidak pakai logo/gambar resmi Spider-Man tanpa lisensi. Kalau pakai
     aset luar (CC0/CC-BY), catat di docs/credits.md.
  3. Cek dimensi dengan `identify *.png` di VM atau Pillow di Windows.
  4. Kalau menjalankan ulang scripts/generate-assets.py, file final kamu
     AKAN DITIMPA. Simpan dulu di tempat lain.

Detail lengkap: docs/desain.md §4 dan docs/catatan.md entri Phase 4.
===============================================================================
