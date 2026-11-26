## PHASE 12: Perbaikan Bug, ISO v2, dan Freeze

**Tujuan:** memperbaiki bug temuan Phase 10 dan 11 lalu menetapkan ISO final.
**Output:** `spideros-v1.0.iso` final (atau v2), checksum, release notes, backup.

### 12.1 Bug log

Catat bug di `docs/catatan.md` bagian **§12.1** (satu file dengan catatan progres, tidak ada file terpisah):

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

### 12.2.1 Perbaikan yang sudah disiapkan (2026-11-26)

Bug temuan Phase 11 sudah masuk [`docs/catatan.md`](../catatan.md) §12.1. Perubahan yang menunggu rebuild:

| Bug | File perbaikan | Isi |
|---|---|---|
| B01 tap-to-click mati | `config/includes.chroot_after_packages/etc/X11/xorg.conf.d/40-libinput-touchpad.conf` | `Option "Tapping" "on"` untuk semua touchpad libinput |
| B01 (fallback) | `config/includes.chroot_after_packages/usr/local/bin/spideros-touchpad-tap` + `etc/xdg/autostart/spideros-touchpad.desktop` | Aktifkan tapping via `xinput` saat sesi XFCE mulai |
| Pencegahan bug hardware | `config/package-lists/hardware.list.chroot` | Firmware SOF/Intel audio, MediaTek, Cirrus, NVIDIA; `alsa-ucm-conf`; `bluez`; `udisks2`, `exfatprogs`, `ntfs-3g`; `upower`, `thermald`, `brightnessctl`; VA-API; alat diagnostik |

Setelah build, uji ulang B01 dan fungsi hardware lain di ASUS Vivobook, lalu perbarui matriks 11.8.

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

Buat `docs/release_notes.md` berisi: versi, tanggal, isi paket utama, fitur tema, fitur Spider-Sense (`--once`, `--watch`, `--no-color`), keterbatasan (Secure Boot, tanpa installer, driver NVIDIA proprietary tidak ada), username dan password live.

Cadangan ISO final di minimal dua tempat (misalnya laptop dan cloud/flashdisk lain), plus satu flashdisk bootable siap presentasi.

### 12.6 Checklist Phase 12

- [ ] Semua bug prioritas 1 dan 2 selesai
- [ ] ISO final diuji ulang di VM dan perangkat asli
- [ ] Tag `v1.0`, checksum, release notes ada
- [ ] ISO dibackup, flashdisk presentasi siap

✅ **Selesai kalau:** ISO final stabil dan dibekukan.

---

