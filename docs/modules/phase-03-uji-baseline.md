## PHASE 3: Uji Baseline di VM Test (BIOS dan UEFI)

**Tujuan:** membuktikan ISO baseline bisa boot sebelum ditambah tema.
**Output:** dua VM test (BIOS dan UEFI), screenshot boot, catatan hasil.

Kenapa tidak diuji langsung di VM build? Nested virtualization umumnya tidak jalan dengan mulus. Jadi ISO dipindah ke Windows lalu diuji di VM baru.

### 3.1 Buat VM test (BIOS) — VirtualBox

1. VirtualBox → **New**.
2. Name: `SpiderOS-Test-BIOS`. ISO Image: pilih ISO baseline. Type: Linux, Version: Debian (64-bit). Centang **Skip Unattended Installation**.
3. Hardware: Base Memory 4096 MB (minimal 2048), Processors 2.
4. Hard Disk: pilih **Do not add a Virtual Hard Disk** (live system tidak butuh disk).
5. Finish. Lalu **Settings → Display**: Video Memory 128 MB, Graphics Controller **VMSVGA**.
6. Pastikan **Settings → Storage** ada ISO terpasang di optical drive.

### 3.1-alt Buat VM test (BIOS) — VMware Workstation Pro

1. **File → New Virtual Machine → Custom (advanced) → Next**.
2. Hardware compatibility: default → Next.
3. Installer disc image file (iso): pilih ISO baseline → Next.
4. Guest OS: **Linux** → Version: **Debian 13 64-bit** → Next.
5. Name: `SpiderOS-Test-BIOS` → Next.
6. Firmware type: pilih **BIOS** (bukan UEFI) → Next.
7. Processors: 2, Memory: 4096 MB → Next.
8. Network: **Use NAT** → Next.
9. I/O Controller: default → Next.
10. Disk: **Do not add a virtual disk** (atau Create, lalu di Settings hapus) → Next → Finish.
11. **VM → Settings → Options → Advanced → Firmware type: BIOS**.
12. **VM → Settings → CD/DVD (SATA) → Use ISO image file**, pilih ISO baseline. Centang **Connect at power on**.

### 3.2 Buat VM test (UEFI)

**VirtualBox:** Klik kanan `SpiderOS-Test-BIOS` → **Clone** → nama `SpiderOS-Test-UEFI` → **Settings → System → Motherboard → centang Enable EFI (special OSes only)**.

**VMware:** Klik kanan `SpiderOS-Test-BIOS` → **Manage → Clone** → nama `SpiderOS-Test-UEFI` → **VM → Settings → Options → Advanced → Firmware type: UEFI** → OK.

### 3.3 Jalankan dan uji

Nyalakan tiap VM. Yang akan terlihat:

| Mode | Boot menu | Pilih |
|---|---|---|
| BIOS | Menu ISOLINUX/Syslinux (gaya lama) | Live system |
| UEFI | Menu GRUB (hitam) | Live system |

Setelah masuk desktop (auto-login), cek di terminal:

```bash
# [TEST-VM]
cat /etc/os-release
uname -r
cat /proc/cmdline
free -h
ip a          # ada IP? internet jalan?
whoami        # spider
```

Test fungsional:
- [x] Muncul desktop XFCE tanpa error
- [x] Terminal bisa dibuka
- [x] Internet jalan (`ping -c 3 deb.debian.org`)
- [x] Resolusi layar menyesuaikan jendela VM
- [x] Shutdown/reboot normal

📸 Screenshot: boot menu BIOS, boot menu UEFI, desktop, hasil `cat /proc/cmdline` → `P03-boot-bios-01.png`, `P03-boot-uefi-01.png`, `P03-desktop-01.png`.

### 3.4 Troubleshooting Phase 3

| Gejala | Solusi |
|---|---|
| Layar hitam setelah boot menu | Di menu boot, pilih entri failsafe atau tekan `Tab` (BIOS) / `e` (GRUB) dan tambah `nomodeset` di akhir baris kernel |
| "No bootable medium" | ISO belum terpasang di Storage, atau EFI tidak aktif di VM UEFI |
| Hanya boot di BIOS, tidak di UEFI | Cek hasil `xorriso -report_el_torito` di Phase 2.7 |
| Resolusi kecil | Normal tanpa Guest Additions/VMware Tools, bisa diatur lewat Display settings XFCE |
| Lambat | Naikkan RAM dan Processors VM test |

### 3.5 Checklist Phase 3

- [x] Boot sukses di VM BIOS
- [x] Boot sukses di VM UEFI
- [x] Semua tes fungsional lulus
- [x] Screenshot tersimpan, hasil dicatat di `docs/catatan.md`

✅ **Selesai kalau:** ISO baseline boot di kedua mode. Setelah ini baru boleh menambah tema. Ini juga titik di mana sekitar 70% risiko teknis sudah lewat.

---

