# SpiderOS — Indeks Dokumentasi Modul

## Tentang proyek

SpiderOS adalah distro live turunan **Debian 13 (trixie)** yang dibangun dengan **live-build**, memakai desktop **XFCE 4.20** dan tema Spider-Man. Hasil akhirnya satu file ISO hybrid yang bisa boot dari flashdisk di **BIOS dan UEFI**, tanpa installer ke disk.

Tujuannya:

- ISO ringan dan bertema: wallpaper, GTK, ikon, login screen, boot menu, Plymouth, dan terminal.
- Aplikasi dasar plus hiburan offline: mcomix sebagai kolektor komik dan game dari repo Debian.
- **Spider-Sense**, tool Rust std-only yang membaca `/proc`, mem-parsing data kernel secara manual, dan merender antarmuka ANSI real-time — komponen koding inti proyek.
- Dokumentasi yang langsung dipakai sebagai bahan BAB III, IV, dan V laporan.

Build dilakukan di VM Debian 13 (`[BUILD-VM]`) atau melalui GitHub Actions (`.github/workflows/build-iso.yml`, dipicu manual atau tag `v*`/`ci-*`), uji boot di VM test (`[TEST-VM]`) dan pada laptop fisik, sedangkan transfer ISO dan flashing USB dilakukan dari Windows (`[HOST]`). Folder kerja: `~/spideros`. ISO terbaru yang terverifikasi adalah `spideros-v2.iso` (`9ba567…199d`). Phase 1–11 selesai; Phase 12–13 berjalan menuju build final.

Untuk pengerjaan, buka file phase yang relevan di `modules/`. File `modules/modul_guide.md` berisi versi gabungan, dipakai bila seluruh modul dibutuhkan dalam satu file.

## Peta file

| Kebutuhan | File | Status |
|---|---|---|
| Pengantar, konvensi, peta phase, peran, aturan emas | [`modules/00-pengantar.md`](modules/00-pengantar.md) | — |
| Phase 1 — Persiapan lingkungan, repo, dokumentasi | [`modules/phase-01-persiapan.md`](modules/phase-01-persiapan.md) | ✅ Selesai |
| Phase 2 — Build ISO baseline | [`modules/phase-02-baseline.md`](modules/phase-02-baseline.md) | ✅ Selesai |
| Phase 3 — Uji baseline di VM (BIOS/UEFI) | [`modules/phase-03-uji-baseline.md`](modules/phase-03-uji-baseline.md) | ✅ Selesai |
| Phase 4 — Desain aset visual | [`modules/phase-04-desain-aset.md`](modules/phase-04-desain-aset.md) | ✅ Selesai |
| Phase 5 — Paket, user live, keamanan dasar | [`modules/phase-05-paket-sistem.md`](modules/phase-05-paket-sistem.md) | ✅ Selesai |
| Phase 6 — Tema desktop | [`modules/phase-06-tema-desktop.md`](modules/phase-06-tema-desktop.md) | ✅ Selesai |
| Phase 7 — Branding boot (Plymouth, boot menu) | [`modules/phase-07-branding-boot.md`](modules/phase-07-branding-boot.md) | ✅ Selesai |
| Phase 8 — Branding terminal dan identitas | [`modules/phase-08-branding-terminal.md`](modules/phase-08-branding-terminal.md) | ✅ Selesai |
| Phase 9 — Spider-Sense (Rust) | [`modules/phase-09-spider-sense.md`](modules/phase-09-spider-sense.md) | ✅ Selesai (2 item checklist menyusul) |
| Phase 9.5 — Kolektor komik (mcomix) | [`modules/phase-09.5-komik.md`](modules/phase-09.5-komik.md) | ✅ Selesai |
| Phase 9.6 — Game offline dari repo Debian | [`modules/phase-09.6-game.md`](modules/phase-09.6-game.md) | ✅ Selesai |
| Phase 10 — ISO v1 dan uji menyeluruh | [`modules/phase-10-rc1.md`](modules/phase-10-rc1.md) | ✅ Selesai |
| Phase 11 — Flash USB dan uji hardware | [`modules/phase-11-usb-hardware.md`](modules/phase-11-usb-hardware.md) | ✅ Selesai (2 perangkat: ASUS Vivobook + Lenovo IdeaPad 82KC; Bluetooth belum diverifikasi) |
| Phase 12 — Perbaikan bug, ISO v2, freeze | [`modules/phase-12-freeze.md`](modules/phase-12-freeze.md) | 🟡 Siap — menunggu rebuild lokal v3 (touchpad, DPI, paket hardware, akun) |
| Phase 13 — Laporan, panduan, demo | [`modules/phase-13-laporan.md`](modules/phase-13-laporan.md) | 🟡 Berjalan — dokumentasi selesai; demo/presentasi dan verifikasi ISO final tertunda |
| Lampiran A, B, D, E, F | [`modules/lampiran-01-referensi.md`](modules/lampiran-01-referensi.md) | — |
| Lampiran C — Struktur akhir repo | [`modules/lampiran-02-struktur-repo.md`](modules/lampiran-02-struktur-repo.md) | — |
| Lampiran G, H — VirtualBox dan VMware | [`modules/lampiran-03-hypervisor.md`](modules/lampiran-03-hypervisor.md) | — |
| Versi lengkap satu file (hasil generate) | [`modules/modul_guide.md`](modules/modul_guide.md) | — |
| Laporan tugas besar (lengkap) | [`laporan.md`](laporan.md) | ✅ Identitas diganti placeholder |
| Panduan pengguna | [`panduan_pengguna.md`](panduan_pengguna.md) | ✅ Siap |
| Catatan rilis | [`release_notes.md`](release_notes.md) | ✅ Siap |

## Cara pakai

- Baca indeks ini dulu, lalu buka hanya file phase yang sedang dikerjakan.
- `modules/modul_guide.md` adalah versi gabungan dari semua file di atas. Dihasilkan oleh `scripts/generate-modul.py`; tidak diedit langsung.
- Setelah mengubah file phase, jalankan `python scripts/generate-modul.py` untuk memperbarui versi gabungan.
- Strategi build: **satu build per tonggak** — `v1.0-rc1` (Phase 10), `v2` (Phase 12), dan `v3` (Phase 13) menunggu build lokal. Phase 5–9 hanya menyiapkan dan memvalidasi konfigurasi, tanpa build per phase.

## Alur sinkronisasi Windows ↔ VM

Folder Windows dan folder build di VM (`~/spider-os-build`) sekarang terpisah. Build wajib dijalankan di path native VM, bukan di `/mnt/hgfs`.

- **Source (docs, config, scripts, aset):** lewat Git. Edit di Windows → `git commit` + `git push` → `git pull` di VM.
- **Artefak (ISO, `.sha256`, `build.log`, screenshot):** dari VM jalankan `./scripts/sync-artifacts.sh` supaya tersalin ke `/mnt/hgfs/SpiderOS-Share/spider-os` (folder Windows yang sama).
- `scripts/build.sh` sudah ada di repo, jadi tidak perlu dibuat manual di VM.
- `config/hooks/` di-ignore Git. Semua hook tersedia sebagai template di `config/hooks-templates/`; di VM jalankan `./scripts/install-hooks.sh`.
- File tema (wallpaper, greeter, Plymouth, neofetch, prompt, `terminalrc`) sudah ada di `config/includes.chroot_after_packages/`.
- Untuk boot menu, jalankan `./scripts/setup-bootloaders.sh` di VM setelah `lb config`. Skrip ini mengisi splash, judul, dan warna ISOLINUX/GRUB.

## Sumber kebenaran

- Prosedur dan konfigurasi: file-file phase di `modules/`.
- Palet warna: `tools/spider-sense/src/render/ansi.rs` → [`desain.md`](desain.md).
- Catatan progres, screenshot, dan daftar bug (termasuk status perbaikan): [`catatan.md`](catatan.md), `screenshots/`.


