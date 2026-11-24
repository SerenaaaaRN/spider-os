## PHASE 9.5: Kolektor Komik Offline — mcomix

**Tujuan:** distro punya pembaca komik offline: cover/thumbnail, bookmark, dan mengingat halaman terakhir.
**Output:** `mcomix` terpasang, desktop entry di menu aplikasi, folder koleksi siap pakai.
**Status:** tambahan (wajib selesai sebelum rc1 — feature freeze Phase 10.5).


Keputusan konten: **ISO hanya menyediakan aplikasinya, bukan komiknya.** Tim tidak membuat komik. Pengguna mengisi sendiri komiknya dari koleksi pribadi mereka setelah menjalankan sistem.

### 9.5.1 Daftar paket konten

```bash
cat > config/package-lists/content.list.chroot <<'EOF'
# --- Pembaca komik offline (cover, bookmark, posisi halaman terakhir) ---
mcomix

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
Comment=Pembaca komik offline: cover, bookmark, posisi halaman terakhir
Exec=mcomix
Icon=mcomix
Categories=Graphics;
EOF
```

### 9.5.4 Verifikasi di TEST-VM

- [ ] `command -v mcomix` menemukan binary
- [ ] Buka "SpiderOS Comics" dari menu aplikasi → aplikasi jalan
- [ ] Tambahkan satu file CBZ apa pun dari folder `comics/` (bisa buat dummy: zip berisi beberapa PNG) → cover/thumbnail tampil
- [ ] Pasang bookmark, tutup, buka lagi → bookmark dan posisi halaman tersimpan
- [ ] `mpv` dan `audacious` terbuka

📸 `P9.5-comics-01.png` (aplikasi terbuka), `P9.5-comics-tracking-01.png` (bookmark tersimpan).

### 9.5.5 Checklist Phase 9.5

- [ ] `content.list.chroot` ada (`mcomix`, `mpv`, `audacious`)
- [ ] Folder `/usr/share/spideros/comics/` dengan README ada
- [ ] Desktop entry "SpiderOS Comics" muncul di menu
- [ ] Cover, bookmark, dan posisi halaman teruji dengan file uji di TEST-VM
- [ ] Commit dan snapshot (`P9.5-comics`)

✅ **Selesai kalau:** membuka menu aplikasi → "SpiderOS Comics" → komik dapat dibaca lengkap dengan cover dan bookmark, tanpa internet.

---

