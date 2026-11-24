## PHASE 9.6: Game Offline dari Repo Debian

**Tujuan:** distro punya game bawaan untuk hiburan offline.
**Output:** 1–4 game ringan terpasang, terjangkau dari menu aplikasi.
**Status:** tambahan (wajib selesai sebelum rc1).

Keputusan: game **di-install dari repo Debian** (karya komunitas, legal, offline), bukan dikoding tim. Fokus koding tetap pada Spider-Sense — game di sini adalah pelengkap hiburan, bukan komponen penilaian.

### 9.6.1 Daftar paket game

```bash
cat > config/package-lists/games.list.chroot <<'EOF'
# --- Terminal game ---
bastet                 # tetris clone yang sangat populer
nethack-console        # roguelike klasik

# --- Game GUI ringan (sudah menyediakan desktop entry sendiri) ---
gnome-mines
gnome-sudoku
EOF
```

Ukurannya wajar (< 10 MB total). Jangan tambah game besar (Supertux, Wesnoth) — membengkakkan ISO tanpa menambah identitas.

### 9.6.2 Desktop entry untuk game terminal

`gnome-mines` dan `gnome-sudoku` otomatis muncul di menu. Game terminal perlu wrapper:

```bash
INC=config/includes.chroot_after_packages
cat > $INC/usr/share/applications/spideros-bastet.desktop <<'EOF'
[Desktop Entry]
Type=Application
Name=Bastet (Tetris)
Comment=Tetris clone dari repo Debian
Exec=xfce4-terminal --hold -e bastet
Icon=org.gnome.SwellFoop
Categories=Game;BlocksGame;
EOF

cat > $INC/usr/share/applications/spideros-nethack.desktop <<'EOF'
[Desktop Entry]
Type=Application
Name=NetHack
Comment=Roguelike klasik
Exec=xfce4-terminal --hold -e nethack
Icon=org.gnome.SwellFoop
Categories=Game;RolePlaying;
EOF
```

`--hold` menjaga terminal tetap terbuka setelah game keluar.

### 9.6.3 Verifikasi di TEST-VM

- [ ] `bastet`, `nethack`, Mines, dan Sudoku terbuka dari menu aplikasi
- [ ] Game terminal playable dengan keyboard, keluar dengan `q`/`Ctrl+C`
- [ ] Highscore/game state tidak menyebabkan error saat ditutup

📸 `P9.6-game-01.png` (menu aplikasi dengan game terlihat), `P9.6-bastet-01.png`.

### 9.6.4 Checklist Phase 9.6

- [ ] `games.list.chroot` ada dan terpasang di ISO
- [ ] Desktop entry game terminal muncul di menu
- [ ] Semua game teruji di TEST-VM (BIOS dan UEFI cukup sekali)
- [ ] Commit dan snapshot (`P9.6-games`)

✅ **Selesai kalau:** pengguna baru bisa main game dari menu aplikasi tanpa instalasi tambahan.

---

