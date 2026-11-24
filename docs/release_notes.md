# Catatan Rilis SpiderOS 1.0

| Item | Nilai |
|---|---|
| Versi | 1.0 |
| Tanggal rilis | 2026-11-24 |
| Nama ISO | `spideros-v1.iso` |
| Ukuran | 1.930.952.704 byte (~1,80 GiB) |
| SHA-256 | `8eac0f45a948f01c671dbac81e5654153ed3c4d58c28173cc74df9a4965a74ca` |
| Basis | Debian 13 (trixie), kernel `6.12.118+deb13-amd64` |
| Desktop | XFCE 4.20 dengan LightDM |
| Arsitektur | amd64 (x86-64), ISO hybrid untuk BIOS dan UEFI |

## Isi paket utama

- Browser: `firefox-esr`
- Media: `vlc`, `mpv`, `audacious`
- Komik: `mcomix` (mendukung CBZ/CBR, cover, bookmark, posisi halaman terakhir)
- Game: `gnome-mines`, `gnome-sudoku`, `bastet`, `nethack-console`
- Desktop: plugin Whisker Menu, pulseaudio, power manager, notifikasi, screenshot, task manager, archive plugin, mousepad, ristretto, file-roller
- Sistem: Network Manager, PulseAudio, pavucontrol, LightDM, gvfs, dbus-x11, ufw
- Alat: htop, git, curl, wget, nano, sudo
- Firmware: paket firmware Linux, Intel, Realtek, Atheros, Broadcom, AMD, dan mikrokode
- Tema: Arc, Papirus, Noto Sans, Fira Code

## Fitur tema

- Wallpaper, layar login, dan splash Plymouth bergaya jaring laba-laba
- Boot loader ISOLINUX dan GRUB memakai palet Spider-Sense
- Terminal XFCE, prompt bash, dan `os-release` memakai palet yang sama
- Enam aset visual final: `wallpaper.png`, `login.png`, `logo.png`, `grub-bg.png`, `isolinux-splash.png`, `spideros-logo-small.png`

## Spider-Sense

- Ditulis dalam Rust, std-only, tanpa dependency eksternal
- Membaca dan mem-parsing `/proc` secara manual
- Sampler CPU dengan perhitungan delta, renderer ANSI real-time
- Status Tenang, Waspada, dan BAHAYA berdasarkan pemakaian CPU/RAM
- Opsi: `--once`, `--watch`, `--no-color`, `--help`, `--version`
- 15 unit test lulus
- Dikompilasi di dalam chroot saat build oleh hook `0400`, lalu toolchain dibuang agar tidak ikut ISO

## Akun live

- User: `spider`
- Kata sandi: `live`
- Hostname: `spideros`

## Verifikasi unduhan

```powershell
Get-FileHash spideros-v1.iso -Algorithm SHA256
```

Hasilnya harus sama dengan nilai SHA-256 di tabel atas.

## Keterbatasan

- Secure Boot harus dimatikan saat boot dari flashdisk
- Tidak tersedia installer permanen
- Sistem live tidak menyimpan perubahan setelah reboot
- Driver NVIDIA proprietary tidak disertakan
- ISO tidak berisi komik berhak cipta; pengguna mengisi sendiri folder `/usr/share/spideros/comics/`
- Dukungan WiFi bergantung pada firmware yang tersedia untuk chipset perangkat

## Sumber

- Repo: https://github.com/SerenaaaaRN/spider-os
- Prosedur build: `scripts/build.sh`
- Panduan pengguna: `docs/panduan_pengguna.md`
- Catatan pengerjaan: `docs/catatan.md`
