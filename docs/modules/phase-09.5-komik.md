## PHASE 9.5: Kolektor Komik Offline — YACReader

**Tujuan:** distro punya aplikasi kolektor komik: cover, sinopsis, dan status sudah/belum dibaca.
**Output:** `yacreader` terpasang, desktop entry di menu aplikasi, folder koleksi siap pakai.
**Status:** tambahan (wajib selesai sebelum rc1 — feature freeze Phase 10.5).

Keputusan konten: **ISO hanya menyediakan aplikasinya, bukan komiknya.** Tim tidak membuat komik. YACReader adalah aplikasi koleksi (karya komunitas, GPLv3, dari repo Debian): pengguna yang mengisi komiknya sendiri setelah install — dari koleksi pribadi mereka. Aplikasi otomatis menampilkan cover, sinopsis (dari metadata CBZ), dan status dibaca untuk koleksi apapun yang ditambahkan.

### 9.5.1 Daftar paket konten

```bash
cat > config/package-lists/content.list.chroot <<'EOF'
# --- Kolektor komik offline (tracking cover, sinopsis, status dibaca) ---
yacreader

# --- Media offline ---
mpv
audacious
EOF
```

### 9.5.2 Folder koleksi siap pakai

Siapkan folder kosong yang rapi supaya pengguna langsung tahu ke mana menaruh komik:

```bash
INC=config/includes.chroot_after_packages
mkdir -p $INC/usr/share/spideros/comics
cat > $INC/usr/share/spideros/comics/README.txt <<'EOF'
Letakkan file komik (.cbz/.cbr/.pdf) di folder ini,
lalu buka aplikasi "SpiderOS Comics" dan tambahkan
folder ini sebagai library.
EOF
```

### 9.5.3 Desktop entry

```bash
INC=config/includes.chroot_after_packages
cat > $INC/usr/share/applications/spideros-comics.desktop <<'EOF'
[Desktop Entry]
Type=Application
Name=SpiderOS Comics
Comment=Kolektor komik offline: cover, sinopsis, status dibaca
Exec=yacreader
Icon=yacreader
Categories=Graphics;
EOF
```

### 9.5.4 Verifikasi di TEST-VM

- [ ] `command -v yacreader` menemukan binary
- [ ] Buka "SpiderOS Comics" dari menu aplikasi → aplikasi jalan
- [ ] Uji tracking: tambahkan satu file CBZ apa pun dari folder `comics/` (bisa buat dummy: zip berisi beberapa PNG) → cover tampil
- [ ] Tandai "sudah dibaca", tutup, buka lagi → status tersimpan
- [ ] `mpv` dan `audacious` terbuka

📸 `P9.5-comics-01.png` (aplikasi terbuka), `P9.5-comics-tracking-01.png` (status dibaca tersimpan).

### 9.5.5 Checklist Phase 9.5

- [ ] `content.list.chroot` ada (`yacreader`, `mpv`, `audacious`)
- [ ] Folder `/usr/share/spideros/comics/` dengan README ada
- [ ] Desktop entry "SpiderOS Comics" muncul di menu
- [ ] Cover, sinopsis, dan status dibaca teruji dengan file uji di TEST-VM
- [ ] Commit dan snapshot (`P9.5-comics`)

✅ **Selesai kalau:** membuka menu aplikasi → "SpiderOS Comics" → tracking jalan pada komik apapun yang pengguna tambahkan sendiri, tanpa internet.

---

