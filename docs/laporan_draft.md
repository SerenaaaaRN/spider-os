# SpiderOS — Kerangka Laporan Akhir (draft)

Dokumen ini adalah bahan mentah BAB III–V. Isi yang sudah terisi berasal dari modul, `build.log`, dan screenshot Phase 10. Bagian bertanda **BELUM** harus dilengkapi setelah data tersedia.

## DATA FINAL

| Item | Nilai |
|---|---|
| Nama ISO | `spideros-v1.iso` |
| Ukuran | 1.893.974.016 byte (~1,76 GiB / 1,8 GB) |
| SHA-256 | `559c65f0531e04eeff0992bad0bc800a4acce58835bb089993e004b491692504` |
| Basis | Debian 13 (trixie), kernel `6.12.118+deb13-amd64` |
| Desktop | XFCE 4.20, LightDM |
| Komponen koding | Spider-Sense 1.0.0 — Rust, std-only, 15 unit test |
| Build | live-build, hook `0400` mengompilasi Rust di dalam chroot |
| Screenshot bukti | `P10-boot-bios-1.png` (Plymouth), `P10-boot-bios-2.png` (desktop + terminal) |
| Status uji | Lihat `docs/catatan.md` §10.2 |

---

## BAB III PERANCANGAN

### 3.1 Analisis kebutuhan
- Target: distro live x86-64 turunan Debian, tanpa installer, bisa boot dari flashdisk.
- Kebutuhan fungsional: desktop XFCE, aplikasi produktivitas dan hiburan, identitas visual Spider-Man, monitor sistem Spider-Sense.
- Kebutuhan nonfungsional: ringan, dapat dibangun ulang (reproducible), berbasis file konfigurasi di Git.
- Spesifikasi lingkungan: VM Debian 13 untuk build, VM test untuk uji, Windows untuk transfer dan flashing. **(BELUM: spesifikasi RAM/CPU VM dari Phase 1)**

### 3.2.1 Persiapan remastering
- Instal Debian 13 di VM, pasang `live-build`, `git`, tool pendukung.
- Struktur repo: `auto/`, `config/`, `scripts/`, `tools/`, `docs/`, `assets-final/`.
- Folder kerja native VM: `~/spider-os-build`; shared folder hanya untuk artefak.
- Alur sinkronisasi: Git = jembatan source; `/mnt/hgfs/SpiderOS-Share/spider-os` = jalur ISO/log/screenshot.

### 3.2.2 Perancangan dan kustomisasi tampilan
- Konsep: palet hangat Spider-Sense dari `tools/spider-sense/src/render/ansi.rs` (88 maroon, 174 rose, 223 cream, 180 sand, 138 dusk, 108 sage, 179 amber).
- Enam aset final: logo, logo kecil, wallpaper, login, `grub-bg`, `isolinux-splash` — semuanya PNG dengan ukuran pas.
- Tema: wallpaper + greeter LightDM, Plymouth `spiderman`, terminal XFCE, prompt bash, `os-release`/motd.
- Identitas terminal disatukan: `neofetch` = symlink ke binary Rust, satu sumber ASCII dan palet.

### 3.2.3 Integrasi aplikasi pendukung
- Paket yang terpasang: `firefox-esr`, VLC, XFCE, firmware, `polkitd` + `lxpolkit` (menggantikan `policykit-1-gnome` yang dihapus dari trixie), `ufw`.
- **Ditambahkan, menunggu rebuild:** YACReader, mpv, audacious, dan game. Package list `content.list.chroot`/`games.list.chroot`, 3 desktop entry, serta folder komik sudah dibuat setelah audit akhir. Item uji 18–21 di `docs/catatan.md` ditandai `Menunggu rebuild`.
- `neofetch` tidak tersedia di trixie — digantikan oleh mode snapshot Spider-Sense.
- Spider-Sense: Rust std-only, membaca `/proc` langsung, parser manual, sampler CPU delta, renderer ANSI, status Tenang/Waspada/BAHAYA.

### 3.2.4 Proses remastering
- `lb config` → `lb build` dengan konfigurasi berbasis file.
- Hook: `0050` system config, `0100` theme, `0200` Plymouth, `0300` branding, `0400` kompilasi Spider-Sense.
- Semua hook disimpan sebagai template terlacak di `config/hooks-templates/`, dipasang di VM oleh `scripts/install-hooks.sh`.
- `scripts/build.sh`: `lb clean` (cache tetap) → staging sumber Rust → `lb config` → `lb build` → rename ISO → checksum.

### 3.2.5 Perancangan perangkat lunak Spider-Sense
- Modul: `main`, `cli`, `model`, `parser`, `procfs`, `metrics`, `sampler`, `render`.
- `render/art.rs` = `SPIDER_ART`; `render/ansi.rs` = palet; `render/mod.rs` = frame ala neofetch.
- Mode: `--watch` (default, live tiap 1 detik), `--once` (snapshot), `--no-color`, `--help`, `--version`.
- `cli::default_watch()` memilih mode dari nama pemanggil: `neofetch` → snapshot; nama lain → live.
- 15 unit test lulus (`cargo test`). Hook `0400` mengompilasi di chroot lalu menghapus toolchain.

### 3.3 Hasil yang diharapkan
- ISO hybrid ±1,8 GB, boot langsung ke Live system, desktop bertema, Spider-Sense berfungsi, dapat dibangun ulang dari repo.

---

## BAB IV HASIL DAN PEMBAHASAN

### 4.1.1 Lingkungan pengembangan
- Build VM Debian 13, TEST-VM VMware Workstation, Windows sebagai HOST.
- Struktur akhir repo: lihat `docs/modules/lampiran-02-struktur-repo.md`.
- **(BELUM: screenshot Phase 1/2, versi lengkap tool)**

### 4.1.2 Kustomisasi tampilan
- Plymouth bertema: `P10-boot-bios-1.png`.
- Desktop + wallpaper + terminal bertema: `P10-boot-bios-2.png`.
- Perbedaan BIOS/UEFI tidak dijadikan kriteria: `live-build` menyembunyikan menu dan langsung boot ke "Live system", sehingga keduanya identik.
- **(BELUM: screenshot boot menu GRUB/ISOLINUX, login greeter)**

### 4.1.3 Implementasi dan pengujian Spider-Sense
- Frame `SPIDER_ART` + Status "Tenang" + palet `ansi.rs` tampil otomatis di terminal: `P10-boot-bios-2.png`.
- `neofetch` menghasilkan frame yang sama dengan prompt langsung balik (tanpa Ctrl+C).
- `sense` / `spider-sense` = mode live, dihentikan dengan Ctrl+C.
- 15 unit test lulus; hook `0400` berhasil mengompilasi dan memasang binary ke `/usr/local/bin/spider-sense`.
- **(BELUM: output `file`, `--version`, bukti toolchain tidak ikut)**

### 4.2 Proses pembuatan ISO
- Build sukses: `P: Build completed successfully` di `build.log`.
- Hasil: `spideros-v1.iso`, 1.893.974.016 byte, SHA-256 `559c65…2504` (diverifikasi COCOK di Windows).
- Kendala yang diatasi: `policykit-1-gnome` dihapus dari trixie → `polkitd`+`lxpolkit`; staging sumber Rust belum ada → `stage-spider-sense.sh`; bentrok lock apt dari build ganda → proses dihentikan sebelum rebuild; ISO ter-mount sebagai drive E: → dilepas sebelum dihapus.

### 4.3.1 Pembuatan media live boot
- **BELUM** — Phase 11 (flash USB) belum dijalankan. Perlu Rufus/Etcher + screenshot pengaturan.

### 4.3.2 Pengujian booting dan boot sequence
- **BELUM** — matriks hardware dan foto tombol boot menu belum tersedia.
- Uji di VM sudah: Plymouth, auto-login, wallpaper, terminal, Spider-Sense. Rincian di `docs/catatan.md` §10.2.

### 4.4 Dokumentasi
- Modul dipecah jadi 19 file di `docs/modules/`, `modul_guide.md` dihasilkan oleh `scripts/generate-modul.py`.
- `docs/index.md` sebagai peta file; `docs/catatan.md` sebagai log progres.
- **(BELUM: foto kerja tim)**

### Pembahasan performa
- **BELUM** — data `free -h`, `systemd-analyze`, jumlah proses belum dikumpulkan.

### Pembahasan keterbatasan
- Sistem live tidak persisten secara default.
- Tanpa installer — tidak ada opsi install ke disk.
- Secure Boot perlu dimatikan pada hardware tertentu.
- Tidak ada aset komik berhak cipta di ISO; pengguna mengisi sendiri folder `/usr/share/spideros/comics/`.
- Beberapa item uji fungsional masih "Belum diuji" (lihat §10.2).

---

## BAB V PENUTUP

### 5.1 Kesimpulan
- Remastering Debian 13 berhasil menghasilkan ISO live SpiderOS yang dapat langsung boot ke desktop bertema.
- Identitas visual tersatu dalam satu sumber (Rust), menghilangkan duplikasi ASCII dan palet di shell.
- Spider-Sense membuktikan kontribusi pemrograman: Rust std-only, parser `/proc` manual, 15 test, dikompilasi di dalam chroot saat build.
- ISO final: `spideros-v1.iso`, checksum terverifikasi.

### 5.2 Saran
- Tambah installer (Calamares) untuk opsi instal permanen.
- Dukungan Secure Boot lewat shim yang ditandatangani.
- Opsi persistensi (Lampiran E).
- Lokalisasi bahasa Indonesia, tema tambahan, aplikasi Welcome.
- Pelengkapan uji hardware (Phase 11) dan data performa.

---

## DAFTAR BUKTI

| Bukti | File |
|---|---|
| Log build sukses | `build.log` (VM), tersalin di Windows |
| Checksum ISO | `dist/spideros-v1.iso.sha256` |
| Plymouth | `docs/screenshots/P10-boot-bios-1.png` |
| Desktop + terminal + Spider-Sense | `docs/screenshots/P10-boot-bios-2.png` |
| Hasil uji 21 poin | `docs/catatan.md` §10.2 |
| Source Rust | `tools/spider-sense/src/` |
| Hook kompilasi | `config/hooks-templates/0400-spider-sense-build.hook.chroot` |
| Test otomatis | 15 test lulus (`cargo test`) |

## YANG MASIH PERLU DILENGKAPI

- [ ] Data performa (RAM idle, waktu boot, jumlah proses)
- [ ] Output CLI Spider-Sense lengkap (`file`, `--version`, toolchain bersih)
- [ ] Rebuild ISO sekali lagi supaya Phase 9.5/9.6 masuk
- [ ] Uji aplikasi: Firefox, VLC, YACReader, game, mpv, audacious
- [ ] Phase 11: flash USB dan uji hardware
- [ ] Foto kerja tim, video demo, panduan pengguna
- [ ] Tag Git `v1.0` dan freeze Phase 12 (butuh izin eksplisit)
