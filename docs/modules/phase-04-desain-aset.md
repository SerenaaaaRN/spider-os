## PHASE 4: Desain Aset Visual

**Tujuan:** menyiapkan semua gambar yang dibutuhkan Phase 6, 7, dan 8.
**Output:** folder `assets-final/` berisi PNG siap pakai.
**Dikerjakan oleh:** Desainer (bisa mulai sejak Phase 1).

### 4.1 Konsep visual

Putuskan dulu dan tulis di `docs/desain.md`:

| Elemen | Contoh keputusan |
|---|---|
| Nama distro | SpiderOS (boleh diganti, asal konsisten) |
| Palet utama | Maroon `#870000` (identitas), Rose `#D78787`, Cream `#FFD7AF`, Sand `#D7AF87`, Dusk `#AF8787`; kanvas gelap `#17100F`. Sumber kebenaran: palet ANSI Spider-Sense di `tools/spider-sense/src/render/ansi.rs` |
| Motif | Jaring laba-laba, siluet kota, garis diagonal |
| Gaya logo | Emblem laba-laba bergaya sendiri (bukan logo resmi) |
| Font | Satu sans-serif, misal Noto Sans |

Hak cipta: logo, karakter, dan gambar resmi Spider-Man dilindungi hak cipta. Buat desain sendiri bergaya Spider-Man (siluet, motif jaring, palet hangat maroon–cream yang sama dengan Spider-Sense). Jika memakai aset luar, pilih yang lisensinya bebas (CC0 atau CC-BY). Tulis di laporan bahwa proyek ini untuk keperluan edukasi non-komersial.

### 4.2 Daftar aset dan spesifikasi

| No | Aset | Ukuran | Format | Dipakai di |
|---|---|---|---|---|
| 1 | `wallpaper.png` | 1920x1080 | PNG/JPG | Desktop |
| 2 | `login.png` | 1920x1080 | PNG/JPG | Layar login LightDM |
| 3 | `logo.png` | 400x400 (transparan) | PNG | Plymouth, neofetch, menu |
| 4 | `logo.svg` | vektor | SVG | Sumber, ikon |
| 5 | `grub-bg.png` | 1920x1080 | PNG | Boot menu UEFI (GRUB) |
| 6 | `isolinux-splash.png` | 640x480 | PNG | Boot menu BIOS |
| 7 | `spideros-logo-small.png` | 128x128 | PNG | Foto profil user, ikon menu |
| 8 | (opsional) `frame-000..N.png` | 400x400 | PNG | Animasi Plymouth berbasis frame |

### 4.3 Alat dan cara membuat

- **Inkscape** untuk logo dan motif vektor (simpan sumber `.svg` di `assets-src/`).
- **GIMP** untuk wallpaper dan efek (simpan `.xcf` di `assets-src/`).
- Ekspor final ke `assets-final/`.

Tips wallpaper yang aman: gradient maroon gelap ke dusk/sand hangat di atas kanvas `#17100F`, motif jaring dari garis-garis radial dan lingkaran konsentris, siluet gedung di bawah, logo di tengah atau pojok. Hindari biru: palet ini mengikuti keluarga hangat Spider-Sense.

### 4.4 Konversi dan optimasi dengan ImageMagick

Contoh dari satu master 1920x1080:

```bash
cd ~/spideros/assets-final
# Ukuran persis untuk boot menu BIOS (640x480)
convert wallpaper.png -resize 640x480^ -gravity center -extent 640x480 isolinux-splash.png
# Versi login dan GRUB (boleh sama dengan wallpaper)
cp wallpaper.png login.png
cp wallpaper.png grub-bg.png
# Logo kecil
convert logo.png -resize 128x128 spideros-logo-small.png
# Cek ukuran dan dimensi
identify *.png
```

Jaga total ukuran aset di bawah sekitar 10 MB supaya ISO tetap ringan. Kalau wallpaper terlalu besar, simpan sebagai JPG kualitas 85: `convert wallpaper.png -quality 85 wallpaper.jpg`.

### 4.5 Aset animasi Plymouth (opsional, terakhir)

Default di modul ini: logo statis dengan efek berdenyut (dibuat lewat skrip, tidak perlu frame). Kalau ingin animasi frame:
- 30–60 frame, nama `frame-000.png` sampai `frame-059.png`.
- Ukuran 400x400, latar transparan.
- Diputar berulang. Ini opsional dan hanya dikerjakan kalau semua phase lain sudah aman.

### 4.6 Serah terima ke Builder

Desainer commit ke Git:

```bash
git add assets-src assets-final docs/desain.md
git commit -m "assets: wallpaper, logo, boot splash"
```

### 4.7 Checklist Phase 4

- [ ] Konsep dan palet tertulis di `docs/desain.md`
- [ ] Aset nomor 1–7 selesai dan sesuai ukuran
- [ ] File sumber (.svg/.xcf) tersimpan di `assets-src/`
- [ ] Sudah di-commit

✅ **Selesai kalau:** Builder bisa mengambil semua aset dari `assets-final/` tanpa bertanya.

---

