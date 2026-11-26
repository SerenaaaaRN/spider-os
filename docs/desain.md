# Desain Visual SpiderOS

> **Status:** Phase 4 — konsep, palet, dan aset final sudah terpasang.
> **Sumber kebenaran warna:** `tools/spider-sense/src/render/ansi.rs` (palet ANSI Spider-Sense).
> Palet utama: maroon, rose, cream, sand, dusk. Detail di `docs/modules/phase-04-desain-aset.md` §4.1.
> Guide sudah disesuaikan di Phase 4.1, 6.3, 7.1, 7.2, 8.2, 8.3, 8.4.

---

## 1. Identitas dan konsep visual

| Elemen | Keputusan |
|---|---|
| Nama distro | **SpiderOS** (konsisten di ISO, `/etc/os-release`, prompt, Plymouth, dan `neofetch`) |
| Versi | 1.0 (berbasis Debian 13 Trixie) |
| Tagline | "Dengan kekuatan besar datang tanggung jawab besar." |
| Motif | Jaring laba-laba (garis radial + lingkaran konsentris), siluet kota di bawah, garis diagonal tipis |
| Gaya logo | Emblem laba-laba bergaya sendiri + word art **SPIDEROS**, bukan logo karakter resmi |
| Suasa | Hangat, gelap, "web-noir" |
| Font | **Noto Sans** (UI), **Fira Code** (monospace/terminal) |

Alasan palet hangat: Spider-Sense sudah memakai keluarga warna 256-color hangat (maroon, rose, cream, sand, dusk). Supaya wallpaper, tema GTK, boot menu, Plymouth, prompt, dan terminal terasa satu identitas dengan komponen koding inti, seluruh aset visual mengikuti palet yang sama.

## 1.1 Target pengguna dan persona

| Persona | Karakteristik | Kebutuhan |
|---|---|---|
| **Utama — penggemar Spider-Man / pengguna muda** | Mengenal karakter Spider-Man, memakai laptop pribadi, ingin distro yang enak dilihat dan langsung dipakai | Identitas visual kuat, wallpaper/ikon bertema, hiburan ringan (komik, musik, video, game) |
| Sekunder — pengguna yang tertarik sistem operasi | Ingin melihat cara kerja kernel lewat aplikasi nyata | Spider-Sense yang membaca `/proc` langsung sebagai "indra" bertema |

Bukan target: kebutuhan server/enterprise dan instalasi permanen (SpiderOS memang distribusi live).

Pemetaan kebutuhan ke keputusan desain:

| Kebutuhan | Keputusan di SpiderOS |
|---|---|
| Identitas tema yang kuat | Palet merah-hitam, wallpaper & logo Spiderman, Plymouth `spiderman` |
| Tampilan modern & nyaman | Papirus-Dark, Noto Sans / Fira Code, DPI 120 untuk laptop HiDPI |
| Hiburan & komik | `mcomix`, `mpv`, `audacious`, game ringan, folder komik pengguna |
| Kesan "Spider-Sense" nyata | Aplikasi Rust `spider-sense` / `neofetch` yang membaca `/proc` |
| Pakai laptop sehari-hari | Tap-to-click, firmware WiFi/audio/BT, Network Manager |
| Aman untuk pengguna awam | `ufw` aktif; dua akun dengan hak berbeda (`spider` vs `admin`) |

---

## 2. Palet warna (kanonik)

Lima warna pertama diambil **persis** dari konstanta di `tools/spider-sense/src/render/ansi.rs`. Tiga warna terakhir adalah turunan gelap untuk kanvas/wallpaper (Rust tidak mendefinisikan warna latar karena itu urusan terminal).

| Peran | Kode Rust | xterm-256 | HEX | Pemakaian |
|---|---|---|---|---|
| **Maroon** (identitas utama) | `\x1b[38;5;88m` | 88 | `#870000` | Word art, aksen tema GTK, border/boot menu, prompt bracket |
| **Rose** | `\x1b[38;5;174m` | 174 | `#D78787` | Blok warna kedua, kursor terminal |
| **Cream** (teks/info) | `\x1b[38;5;223m` | 223 | `#FFD7AF` | Teks informasi, `ColorForeground`, judul menu |
| **Sand** (label) | `\x1b[38;5;180m` | 180 | `#D7AF87` | Label `CPU`, `RAM`, `OS`, prompt `@` |
| **Dusk** (pemisah) | `\x1b[38;5;138m` | 138 | `#AF8787` | Garis pemisah, tanda `@`, motif jaring |
| **Sage** — status "Tenang" | `\x1b[38;5;108m` | 108 | `#87AF87` | Status CPU/RAM < 40% |
| **Amber** — status "Waspada" | `\x1b[38;5;179m` | 179 | `#D7AF5F` | Status 40–75% |
| **Red** — status "BAHAYA" | `\x1b[1;31m` | bright red | `#FF5555` | Status > 75% |
| Kanvas (turunan) | — | — | `#17100F` | Latar terminal, gradient paling gelap, Plymouth atas |
| Panel (turunan) | — | — | `#2A1A18` | Panel wallpaper, kartu info |
| Maroon dalam (turunan) | — | — | `#4A0B0B` | Gradient bawah Plymouth, bayangan jaring |

**Aturan penggunaan:**
- Gunakan hanya palet di §2 pada seluruh aset dan konfigurasi.
- Maroon adalah aksen; jangan dipakai sebagai latar penuh teks (kontras rendah). Teks memakai cream/sand di atas kanvas.
- Status memakai tiga warna fungsional (sage/amber/red) dan tidak boleh diganti warna dekoratif.

---

## 3. Daftar aset dan spesifikasi

| No | Aset | Ukuran | Format | Dipakai di | Status sekarang |
|---|---|---|---|---|---|
| 1 | `wallpaper.png` | 1920×1080 | PNG | Desktop | **Final** |
| 2 | `login.png` | 1920×1080 | PNG | Layar login LightDM | **Final** |
| 3 | `logo.png` | 400×400, transparan | PNG | Plymouth, neofetch, menu | **Final** |
| 4 | `logo.svg` | vektor | SVG | Sumber logo | **Final** |
| 5 | `grub-bg.png` | 1920×1080 | PNG | Boot menu UEFI (GRUB) | **Final** |
| 6 | `isolinux-splash.png` | 640×480 | PNG | Boot menu BIOS | **Final** |
| 7 | `spideros-logo-small.png` | 128×128 | PNG | Foto profil user, ikon menu | **Final** |
| 8 | `frame-000..N.png` | 400×400 | PNG | Animasi Plymouth frame | Opsional, **tidak dibuat** (default: logo berdenyut via skrip) |

Sumber vektor ada di `assets-src/logo.svg` (satu salinan). PNG final disiapkan dan dinormalisasi dengan Python + Pillow; ukuran dan format dicek sebelum dipakai. `scripts/generate-assets.py` adalah generator dummy lama — menjalankannya akan menimpa PNG final. Total ukuran aset dijaga di bawah ~10 MB.

---

## 4. Aset final — status penggantian

Aset di `assets-final/` sudah diganti dengan file final. Checklist di bawah tercapai:

- [x] `assets-final/wallpaper.png` → cari/ gambar wallpaper 1920×1080 bertema jaring/kota dengan palet di §2.
- [x] `assets-final/login.png` → versi login (boleh sama, lebih gelap di sisi kiri untuk teks greeter).
- [x] `assets-final/logo.png` → logo 400×400 **transparan**; ini yang dipakai Plymouth & `neofetch`.
- [x] `assets-src/logo.svg` → sumber vektor final (satu salinan, tidak ada duplikat di `assets-final/`).
- [x] `assets-final/grub-bg.png` → 1920×1080 untuk GRUB.
- [x] `assets-final/isolinux-splash.png` → **tepat 640×480** untuk ISOLINUX.
- [x] `assets-final/spideros-logo-small.png` → 128×128.

Kriteria penggantian:
1. Ukuran dan format sesuai tabel §3 (cek dengan `identify` di VM atau Pillow di Windows).
2. Warna mengikuti palet §2 — tidak ada biru.
3. Tidak memakai logo/gambar resmi Spider-Man tanpa lisensi.
4. Setelah mengganti, jalankan ulang `scripts/generate-assets.py` **hanya jika** ingin regenerate dari SVG; skrip akan menimpa PNG, jadi simpan dulu file final kamu.

Catatan terkait ada di `docs/catatan.md`, `assets-final/README-GANTI-ASET.txt`, dan langkah salin aset di `docs/modules/phase-06-tema-desktop.md` §6.2 serta `docs/modules/phase-07-branding-boot.md` §7.1/§7.2.

---

## 5. Panduan integrasi ke phase berikutnya

Rincian siap pakai (hook, XML, PS1, `terminalrc`, Plymouth) ada di `docs/modules/phase-06-tema-desktop.md`, `phase-07-branding-boot.md`, dan `phase-08-branding-terminal.md`. Navigasi: `docs/index.md`. Ringkasannya:

| Phase | Ringkasan integrasi warna |
|---|---|
| 6.3 hook `0100-theme` | Aksen maroon `#870000`, `rgba(135, 0, 0)` |
| 6.4 `xsettings.xml` | Tema `SpiderOS`, ikon `Papirus-Dark`, font Noto Sans / Fira Code |
| 6.6 LightDM | `background=login.png`, `default-user-image=spideros-logo.png` |
| 7.1 Plymouth | Atas `#17100F` → `(0.09, 0.06, 0.06)`; bawah `#4A0B0B` → `(0.29, 0.04, 0.04)` |
| 7.2 ISOLINUX/GRUB | Maroon `#870000`, cream `#FFD7AF`, highlight merah tua |
| 8.2 neofetch | `ascii_colors=(1 3)` (pendekatan 8 warna: merah + kuning) |
| 8.3 PS1 | `\e[38;5;88m` maroon, `\e[38;5;180m` sand, `\e[38;5;223m` cream |
| 8.4 `terminalrc` | Latar `#17100F`, teks `#FFD7AF`, kursor `#D78787`, palet 16 warna hangat |

Semua perubahan ini sudah diterapkan di file-file phase di `docs/modules/`. Versi lengkap dalam satu file tersedia di `docs/modules/modul_guide.md` (hasil generate).

---

## 6. Hak cipta dan kredit

Logo, karakter, dan gambar resmi Spider-Man dilindungi hak cipta. Aset di proyek ini dibuat sendiri bergaya Spider-Man (siluet, jaring, palet hangat) untuk **keperluan edukasi non-komersial**. Semua aset dibuat sendiri bergaya Spider-Man; tidak ada aset pihak ketiga di ISO.

---

## 7. Checklist Phase 4

- [x] Konsep dan palet tertulis di `docs/desain.md` (palet mengikuti Spider-Sense).
- [x] Aset nomor 1–7 ada dan sesuai ukuran — lihat §4.
- [x] File sumber `.svg` tersimpan di `assets-src/`.
- [x] Commit Git — `1d39972` sudah dibuat dan disinkronkan ke `origin/main`.

✅ **Selesai:** semua kotak §4 tercentang; aset final siap dipakai Phase 6/7. 🕷️
