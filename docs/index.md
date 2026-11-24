# SpiderOS — Indeks Dokumentasi Modul

## Tentang proyek

SpiderOS adalah distro live turunan **Debian 13 (trixie)** yang dibangun dengan **live-build**, memakai desktop **XFCE 4.20** dan tema Spider-Man. Hasil akhirnya satu file ISO hybrid yang bisa boot dari flashdisk di **BIOS dan UEFI**, tanpa installer ke disk.

Tujuannya:

- ISO ringan dan bertema: wallpaper, GTK, ikon, login screen, boot menu, Plymouth, dan terminal.
- Aplikasi dasar plus hiburan offline: YACReader sebagai kolektor komik dan game dari repo Debian.
- **Spider-Sense**, tool Rust std-only yang membaca `/proc`, mem-parsing data kernel secara manual, dan merender antarmuka ANSI real-time — komponen koding inti proyek.
- Dokumentasi yang langsung dipakai sebagai bahan BAB III, IV, dan V laporan.

Build dilakukan di VM Debian 13 (`[BUILD-VM]`), uji boot di VM test BIOS/UEFI (`[TEST-VM]`), sedangkan transfer ISO dan flashing USB dilakukan dari Windows (`[HOST]`). Folder kerja: `~/spideros`. Phase 4 selesai; pengerjaan berikutnya Phase 5.

Untuk pengerjaan, buka file phase yang relevan di `modules/`. File `modules/modul_guide.md` berisi versi gabungan, dipakai bila seluruh modul dibutuhkan dalam satu file.

## Peta file

| Kebutuhan | File | Status |
|---|---|---|
| Pengantar, konvensi, peta phase, peran, aturan emas | [`modules/00-pengantar.md`](modules/00-pengantar.md) | — |
| Phase 1 — Persiapan lingkungan, repo, dokumentasi | [`modules/phase-01-persiapan.md`](modules/phase-01-persiapan.md) | ✅ Selesai |
| Phase 2 — Build ISO baseline | [`modules/phase-02-baseline.md`](modules/phase-02-baseline.md) | ✅ Selesai |
| Phase 3 — Uji baseline di VM (BIOS/UEFI) | [`modules/phase-03-uji-baseline.md`](modules/phase-03-uji-baseline.md) | ✅ Selesai |
| Phase 4 — Desain aset visual | [`modules/phase-04-desain-aset.md`](modules/phase-04-desain-aset.md) | ✅ Selesai |
| Phase 5 — Paket, user live, keamanan dasar | [`modules/phase-05-paket-sistem.md`](modules/phase-05-paket-sistem.md) | ⬜ Belum |
| Phase 6 — Tema desktop | [`modules/phase-06-tema-desktop.md`](modules/phase-06-tema-desktop.md) | ⬜ Belum |
| Phase 7 — Branding boot (Plymouth, boot menu) | [`modules/phase-07-branding-boot.md`](modules/phase-07-branding-boot.md) | ⬜ Belum |
| Phase 8 — Branding terminal dan identitas | [`modules/phase-08-branding-terminal.md`](modules/phase-08-branding-terminal.md) | ⬜ Belum |
| Phase 9 — Spider-Sense (Rust) | [`modules/phase-09-spider-sense.md`](modules/phase-09-spider-sense.md) | ⚠️ Kode + file integrasi ✅, build chroot belum |
| Phase 9.5 — Kolektor komik (YACReader) | [`modules/phase-09.5-komik.md`](modules/phase-09.5-komik.md) | ⬜ Belum |
| Phase 9.6 — Game offline dari repo Debian | [`modules/phase-09.6-game.md`](modules/phase-09.6-game.md) | ⬜ Belum |
| Phase 10 — ISO v1 rc dan uji menyeluruh | [`modules/phase-10-rc1.md`](modules/phase-10-rc1.md) | ⬜ Belum |
| Phase 11 — Flash USB dan uji hardware | [`modules/phase-11-usb-hardware.md`](modules/phase-11-usb-hardware.md) | ⬜ Belum |
| Phase 12 — Perbaikan bug, ISO v2, freeze | [`modules/phase-12-freeze.md`](modules/phase-12-freeze.md) | ⬜ Belum |
| Phase 13 — Laporan, panduan, demo | [`modules/phase-13-laporan.md`](modules/phase-13-laporan.md) | ⬜ Belum |
| Lampiran A, B, D, E, F | [`modules/lampiran-01-referensi.md`](modules/lampiran-01-referensi.md) | — |
| Lampiran C — Struktur akhir repo | [`modules/lampiran-02-struktur-repo.md`](modules/lampiran-02-struktur-repo.md) | — |
| Lampiran G, H — VirtualBox dan VMware | [`modules/lampiran-03-hypervisor.md`](modules/lampiran-03-hypervisor.md) | — |
| Versi lengkap satu file (hasil generate) | [`modules/modul_guide.md`](modules/modul_guide.md) | — |

## Cara pakai

- Baca indeks ini dulu, lalu buka hanya file phase yang sedang dikerjakan.
- `modules/modul_guide.md` adalah versi gabungan dari semua file di atas. Dihasilkan oleh `scripts/generate-modul.py`; tidak diedit langsung.
- Setelah mengubah file phase, jalankan `python scripts/generate-modul.py` untuk memperbarui versi gabungan.

## Alur sinkronisasi Windows ↔ VM

Folder Windows dan folder build di VM (`~/spider-os-build`) sekarang terpisah. Build wajib dijalankan di path native VM, bukan di `/mnt/hgfs`.

- **Source (docs, config, scripts, aset):** lewat Git. Edit di Windows → `git commit` + `git push` → `git pull` di VM.
- **Artefak (ISO, `.sha256`, `build.log`, screenshot):** dari VM jalankan `./scripts/sync-artifacts.sh` supaya tersalin ke `/mnt/hgfs/SpiderOS-Share/spider-os` (folder Windows yang sama).
- `scripts/build.sh` sudah ada di repo, jadi tidak perlu dibuat manual di VM.
- `config/hooks/` di-ignore Git. Template hook ada di `config/hooks-templates/`; di VM tinggal disalin ke `config/hooks/live/` lalu `chmod +x`.

## Sumber kebenaran

- Prosedur dan konfigurasi: file-file phase di `modules/`.
- Palet warna: `tools/spider-sense/src/render/ansi.rs` → [`desain.md`](desain.md).
- Catatan progres dan screenshot: [`catatan.md`](catatan.md), `screenshots/`.
- Atribusi aset: [`credits.md`](credits.md).


