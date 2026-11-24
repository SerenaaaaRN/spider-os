## PHASE 12: Perbaikan Bug, ISO v2, dan Freeze

**Tujuan:** memperbaiki bug temuan Phase 10 dan 11 lalu menetapkan ISO final.
**Output:** `spideros-v1.0.iso` final (atau v2), checksum, release notes, backup.

### 12.1 Bug log

Buat `docs/bug-log.md`:

| ID | Ditemukan di | Deskripsi | Penyebab | Perbaikan | Status |
|---|---|---|---|---|---|
| B01 | Phase 11 laptop X | WiFi tidak terdeteksi | Firmware tidak ada | Tambah `firmware-xxx` | Selesai |

Urutkan prioritas: (1) tidak bisa boot, (2) tidak bisa dipakai, (3) tampilan, (4) kosmetik. Kerjakan dari atas, kosmetik opsional.

### 12.2 Perbaikan umum yang sering dibutuhkan

| Masalah | Perbaikan |
|---|---|
| WiFi tidak ada | Tambah paket firmware di `spideros.list.chroot` (Intel: `firmware-iwlwifi`, Realtek: `firmware-realtek`, Broadcom: `firmware-brcm80211`, Atheros: `firmware-atheros`) |
| Layar hitam di GPU tertentu | Tambah `firmware-amd-graphics` atau gunakan menu failsafe, catat sebagai keterbatasan |
| Touchpad tidak ada tap-to-click | Tambah `xserver-xorg-input-libinput`, atau atur lewat Settings |
| Bahasa/keyboard | Ubah parameter `keyboard-layouts` dan `locales` |
| ISO terlalu besar | Hapus paket berat (lihat 5.2), pastikan toolchain Rust sudah dipurge oleh hook 0400 |
| Spider-Sense gagal dikompilasi di chroot | Cek `Cargo.lock`, paket build-only, dan log hook 0400; jalankan `cargo build --release --locked` manual di `tools/spider-sense` |

### 12.3 Build v2

Satu perbaikan satu commit, lalu build:

```bash
git add -A && git commit -m "fix: B01 tambah firmware wifi"
sudo lb clean --purge
./scripts/build.sh v1.0
```

Ulangi uji: ringkasan 21 poin Phase 10 di VM, lalu uji ulang di perangkat bermasalah (Phase 11).

### 12.4 Freeze

Aturan setelah freeze:
- Tidak ada fitur baru.
- ISO final tidak dibuild ulang kecuali ada bug fatal.
- Semua screenshot laporan diambil dari ISO final (supaya konsisten).

### 12.5 Rilis dan cadangan

```bash
git add -A
git commit -m "release: v1.0 final"
git tag v1.0
sha256sum dist/spideros-v1.0.iso
```

Buat `docs/RELEASE_NOTES.md` berisi: versi, tanggal, isi paket utama, fitur tema, fitur Spider-Sense (`--once`, `--watch`, `--no-color`), keterbatasan (Secure Boot, tanpa installer, driver NVIDIA proprietary tidak ada), username dan password live.

Cadangan ISO final di minimal dua tempat (misalnya laptop dan cloud/flashdisk lain), plus satu flashdisk bootable siap presentasi.

### 12.6 Checklist Phase 12

- [ ] Semua bug prioritas 1 dan 2 selesai
- [ ] ISO final diuji ulang di VM dan perangkat asli
- [ ] Tag `v1.0`, checksum, release notes ada
- [ ] ISO dibackup, flashdisk presentasi siap

✅ **Selesai kalau:** ISO final stabil dan dibekukan.

---

