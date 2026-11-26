## PHASE 11: Flash USB dan Uji Hardware (UEFI dan Legacy)

**Tujuan:** membuktikan ISO benar-benar bisa boot dari flashdisk di laptop/PC asli, sesuai rumusan masalah ke-4.
**Output:** flashdisk bootable, matriks hasil uji hardware, catatan boot sequence dan cara masuk boot menu.

### 11.1 Persiapan

Flashdisk minimal 4 GB (8 GB lebih aman). Semua isinya akan terhapus, backup dulu.
ISO final di Windows, misal `D:\SpiderOS-Share\spideros-v1.0-rc1.iso`.
Tools: Rufus atau Balena Etcher.

### 11.2 Verifikasi checksum `[HOST]`

PowerShell:

```powershell
Get-FileHash D:\SpiderOS-Share\spideros-v1.0-rc1.iso -Algorithm SHA256
```

Bandingkan dengan isi file `.sha256` di build-VM. Harus sama persis.

### 11.3 Flash dengan Rufus

1. Colok flashdisk, buka Rufus.
2. **Device:** pilih flashdisk yang benar (hati-hati, jangan salah drive).
3. **Boot selection:** klik **SELECT**, pilih ISO.
4. **Partition scheme dan Target system:**
   - Untuk laptop UEFI: **GPT** dan **UEFI (non CSM)**.
   - Untuk laptop Legacy/BIOS: **MBR** dan **BIOS (or UEFI-CSM)**.
5. Karena ISO hybrid, hasilnya biasanya bisa boot di keduanya.
6. Klik **START**. Jika muncul pilihan **ISO Image mode** atau **DD Image mode**, pilih **DD Image mode** (paling aman untuk ISO hybrid live-build).
7. Tunggu sampai status **READY**.

### 11.4 Flash dengan Balena Etcher (alternatif)

1. **Flash from file**, pilih ISO.
2. **Select target**, pilih flashdisk.
3. **Flash!** lalu tunggu validasi selesai.

📸 `P11-rufus-01.png` atau `P11-etcher-01.png`.

### 11.5 Cara masuk boot menu (umum)

Saat laptop baru dinyalakan, tekan tombol berikut berulang sampai muncul menu.

| Merek | Boot menu | Masuk BIOS/UEFI setup |
|---|---|---|
| Acer | F12 (kadang harus diaktifkan di BIOS) | F2 |
| ASUS | Esc (laptop) / F8 (motherboard) | F2 atau Del |
| Lenovo | F12 (atau tombol Novo kecil) | F2 (kadang Fn+F2) |
| HP | Esc lalu F9 | F10 atau Esc lalu F10 |
| Dell | F12 | F2 |
| MSI | F11 | Del |
| Samsung | Esc atau F10 | F2 |
| Toshiba/Dynabook | F12 | F2 |
| Gigabyte (motherboard) | F12 | Del |
| Mac Intel | Tahan Option (Alt) | (tidak ada BIOS biasa) |

Lewat Windows (jika tombol sulit): Windows 10/11 → Settings → Update and Security → Recovery → Advanced startup → Restart now → Use a device → pilih USB. Atau tahan Shift sambil klik Restart.

### 11.6 Secure Boot dan BitLocker (penting)

ISO live-build tidak memakai bootloader bertanda tangan Microsoft, jadi **Secure Boot harus dimatikan sementara** saat uji (di BIOS: Security atau Boot → Secure Boot → Disabled). Tulis ini sebagai keterbatasan di laporan.

Kalau Windows di laptop memakai BitLocker, mengubah pengaturan Secure Boot bisa meminta recovery key saat kembali ke Windows. Pastikan kamu punya key itu, atau uji di laptop teman/lab yang tidak memakai BitLocker.

Setelah uji, aktifkan kembali Secure Boot.

Fast Startup Windows kadang mengganggu: matikan dulu (Control Panel → Power Options → Choose what the power buttons do) atau pakai **Restart**, bukan Shut down.

### 11.7 Prosedur uji di tiap perangkat

1. Matikan laptop, colok flashdisk.
2. Nyalakan, tekan tombol boot menu (11.5).
3. Pilih flashdisk. Perhatikan ada dua entri: bernama biasa (Legacy/BIOS) dan berawalan `UEFI:` (UEFI). Uji keduanya kalau bisa.
4. Boot menu SpiderOS muncul, pilih `Live system`.
5. Catat apa yang terjadi, lalu ambil foto layar atau screenshot.
6. Uji fungsi dasar: WiFi, suara, touchpad, resolusi layar, `spider-sense`.

### 11.8 Matriks hasil uji hardware

Isi untuk tiap perangkat yang diuji:

| Perangkat (merek/model) | Mode boot | Secure Boot | Boot menu muncul | Desktop muncul | WiFi | Audio | Touchpad | Spider-Sense | Catatan |
|---|---|---|---|---|---|---|---|---|---|
|  | UEFI | Off |  |  |  |  |  |  |  |
|  | Legacy | n/a |  |  |  |  |  |  |  |

Targetkan minimal 2 perangkat berbeda.

**Hasil sejauh ini (2026-11-26):**

| Perangkat (merek/model) | Mode boot | Secure Boot | Boot menu muncul | Desktop muncul | WiFi | Audio | Touchpad | Spider-Sense | Catatan |
|---|---|---|---|---|---|---|---|---|---|
| ASUS Vivobook | belum dicatat | Off | ya | ya (live system) | belum diuji | belum diuji | tap mati → B01 | belum diuji | boot berhasil; lanjut uji fungsi lain |

Bug touchpad dicatat sebagai **B01** di [`docs/catatan.md`](../catatan.md) §12.1 dan sudah diperbaiki di repo (menunggu rebuild).

### 11.9 Catat boot sequence (untuk BAB IV 4.3.2)

Urutan proses yang terjadi saat boot dari flashdisk:

1. Power on → firmware (BIOS atau UEFI) melakukan POST
2. Firmware memilih perangkat boot (USB), BIOS membaca MBR, UEFI membaca partisi EFI
3. Bootloader dimuat (ISOLINUX untuk BIOS, GRUB untuk UEFI) dan menampilkan menu
4. Kernel Linux dan initramfs dimuat ke RAM
5. `live-boot` di initramfs mencari media live, memasang `filesystem.squashfs` (read-only) dan menumpuknya dengan overlay (tmpfs) agar sistem bisa ditulis di RAM
6. Plymouth menampilkan splash, systemd menjalankan layanan
7. `live-config` membuat user, hostname, locale, zona waktu sesuai parameter boot
8. LightDM auto-login ke XFCE

Bukti di `[TEST-VM]` atau laptop asli:

```bash
cat /proc/cmdline                 # parameter boot (boot=live, dll.)
findmnt /                         # root berupa overlay
mount | grep -E "squashfs|overlay"
lsblk                             # flashdisk dan loop device squashfs
systemd-analyze                   # waktu firmware, loader, kernel, userspace
```

Cek mode boot yang dipakai:

```bash
[ -d /sys/firmware/efi ] && echo "Booted in UEFI mode" || echo "Booted in Legacy BIOS mode"
```

📸 Foto/screenshot hasil perintah di atas untuk tiap mode.

### 11.10 Troubleshooting Phase 11

| Gejala | Penyebab | Solusi |
|---|---|---|
| Flashdisk tidak muncul di boot menu | Mode boot/skema partisi tidak cocok, atau port USB | Coba port USB 2.0, flash ulang dengan skema berbeda (GPT atau MBR), aktifkan "USB boot" di BIOS |
| Muncul tapi langsung kembali ke menu | Secure Boot aktif | Matikan Secure Boot |
| Hanya muncul satu entri (Legacy atau UEFI) | Pengaturan Boot Mode di BIOS membatasi | Ubah Boot Mode ke "UEFI and Legacy" atau "CSM enabled" |
| Layar hitam setelah pilih Live | Driver grafis | Pilih failsafe atau tambah `nomodeset` di baris kernel (Tab atau `e`) |
| WiFi tidak muncul | Firmware belum ada | Catat chipset-nya (`lspci -nnk \| grep -A3 -i network`), tambah paket firmware yang sesuai di Phase 12 |
| "Verification failed" di Rufus/Etcher | USB rusak atau ISO rusak | Cek checksum, coba flashdisk lain |
| Lambat sekali | Flashdisk murah (USB 2.0) | Normal, catat sebagai keterbatasan |

### 11.11 Checklist Phase 11

- [x] Checksum cocok, USB berhasil dibuat
- [ ] Boot sukses di minimal 2 perangkat, mode UEFI dan Legacy tercatat (baru 1 perangkat: ASUS Vivobook)
- [ ] Boot menu per merek, matriks hasil uji, dan bukti boot sequence terisi
- [x] Bug temuan masuk `docs/catatan.md` §12.1

✅ **Selesai kalau:** ISO terbukti boot dari flashdisk di perangkat asli (rumusan masalah ke-4 terjawab).

---

