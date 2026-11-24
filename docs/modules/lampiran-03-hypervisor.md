### Lampiran G: Panduan Lengkap VirtualBox

**G.1 Install VirtualBox di Windows**
1. Download dari https://www.virtualbox.org/wiki/Downloads (Windows hosts).
2. Jalankan installer `.exe`, klik Next sampai selesai.
3. Install Extension Pack (opsional, untuk USB 2/3): download dari halaman yang sama, klik dua kali file `.vbox-extpack`.

**G.2 Install Debian 13 di VirtualBox**
1. Download ISO Debian 13 netinst dari https://www.debian.org/download.
2. **Machine → New**:
   - Name: `Debian13-Build`
   - Type: Linux, Version: Debian (64-bit)
   - ISO Image: pilih ISO Debian 13
   - Centang **Skip Unattended Installation**
3. Hardware: Memory 4096 MB, Processors 4.
4. Hard Disk: **Create a Virtual Hard Disk Now**, 40 GB, VDI, Dynamically allocated.
5. Finish → **Start**.
6. Ikuti installer Debian:
   - Language: English
   - Location: Indonesia
   - Keyboard: American English
   - Hostname: `debian`
   - Domain: kosong
   - Root password: buat yang mudah diingat
   - User: nama kamu, password: buat yang mudah diingat
   - Partitioning: **Guided - use entire disk**, pilih satu disk, **All files in one partition**
   - Write changes → Yes
   - Package manager: pilih mirror terdekat
   - Software selection: **hanya centang "SSH server" dan "standard system utilities"** (jangan centang desktop environment, supaya VM ringan)
   - Install GRUB ke `/dev/sda` → Yes
7. Reboot, login dengan user yang dibuat.

**G.3 Konfigurasi pasca-install**

```bash
su -
apt update && apt full-upgrade -y
apt install -y sudo
usermod -aG sudo NAMA_USER_KAMU
exit
# logout lalu login ulang
```

**G.4 Snapshot**
- **Machine → Take Snapshot** → beri nama deskriptif (misal `P1-clean-start`).
- Untuk restore: **Machine → Snapshots → pilih snapshot → Restore**.

**G.5 Shared Folder (untuk pindahkan ISO ke Windows)**
Lihat Phase 1.5 Opsi A VirtualBox.

**G.6 Boot test ISO SpiderOS**
Lihat Phase 3.1 (BIOS) dan 3.2 (UEFI).

### Lampiran H: Panduan Lengkap VMware Workstation Pro

**H.1 Install VMware Workstation Pro di Windows**

Sejak Mei 2024, VMware Workstation Pro **gratis untuk penggunaan personal** setelah diakuisisi Broadcom.

1. Buka https://support.broadcom.com/group/ecx/productdownloads?subfamily=VMware+Workstation+Pro
2. Daftar akun Broadcom (gratis) kalau belum punya.
3. Download installer VMware Workstation Pro (versi terbaru, misal 17.6.x).
4. Jalankan installer `.exe`, pilih **Use VMware Workstation 17 for Personal Use** (jangan pilih "Commercial").
5. Ikuti wizard sampai selesai.
6. Setelah install, buka VMware — seharusnya tidak minta license key karena sudah mode Personal Use.

**H.2 Install Debian 13 di VMware Workstation Pro**

1. Download ISO Debian 13 netinst dari https://www.debian.org/download.
2. Di VMware: **File → New Virtual Machine**.
3. Pilih **Custom (advanced)** → Next.
4. Hardware compatibility: default → Next.
5. Installer disc image file (iso): **Browse**, pilih ISO Debian 13 → Next.
6. Guest OS Installation: pilih **I will install the operating system later** (supaya kita atur manual) → Next.
7. Guest OS: **Linux** → Version: **Debian 13 64-bit** → Next.
8. Name VM: `Debian13-Build` → pilih lokasi folder → Next.
9. Firmware type: **UEFI** (direkomendasikan) → Next.
10. Processors: **Number of processors: 1**, **Number of cores per processor: 4** (total 4 cores) → Next.
11. Memory: **4096 MB** → Next.
12. Network: **Use NAT** → Next.
13. I/O Controller: **LSI Logic SAS** (default) → Next.
14. Disk: **Create a new virtual disk** → Next.
15. Disk size: **40 GB**, pilih **Store virtual disk as a single file** (lebih cepat) → Next.
16. Specify Disk File: default → Next.
17. Finish.

**H.3 Konfigurasi VM sebelum install Debian**

Sebelum nyalakan, atur:
1. **VM → Settings → Options → Advanced → Disable side channel mitigations** (opsional, untuk performa).
2. **VM → Settings → Display**: Graphics memory **128 MB** atau lebih, 3D acceleration **Enabled**.
3. **VM → Settings → CD/DVD (SATA)**: pastikan "Connect at power on" tercentang, ISO Debian terpasang.
4. **VM → Settings → USB Controller**: Share USB input devices **Enabled** (kalau mau pakai USB di VM).

**H.4 Install Debian 13**

1. **Power on** VM.
2. Tekan **Enter** di menu boot Debian untuk graphical install.
3. Ikuti installer (sama seperti VirtualBox di G.2):
   - Language: English
   - Location: Indonesia
   - Keyboard: American English
   - Hostname: `debian`
   - Root password: buat yang mudah diingat
   - User: nama kamu, password: buat yang mudah diingat
   - Partitioning: **Guided - use entire disk** → **All files in one partition** → Finish → Yes
   - Package manager: pilih mirror terdekat
   - Software selection: **HANYA centang "SSH server" dan "standard system utilities"** (jangan centang desktop environment)
   - Install GRUB ke `/dev/sda` → Yes
4. Reboot, login.

**H.5 Install open-vm-tools (PENTING)**

Di VMware, **jangan pakai "Install VMware Tools" dari menu** (legacy). Pakai `open-vm-tools` dari repositori Debian — lebih stabil dan terintegrasi.

```bash
sudo apt update
sudo apt install -y open-vm-tools open-vm-tools-desktop
sudo reboot
```

Setelah reboot:
- Resolusi layar otomatis menyesuaikan jendela VMware.
- Copy-paste antara host dan VM jalan (kalau VMware Tools terdeteksi).
- Shared folder bisa diakses.

**H.6 Shared Folder (untuk pindahkan ISO ke Windows)**

Lihat Phase 1.5 Opsi A VMware.

**H.7 Snapshot di VMware**

- **VM → Snapshot → Take Snapshot** → beri nama deskriptif.
- Untuk restore: **VM → Snapshot → Snapshot Manager → pilih snapshot → Go To**.
- Untuk hapus: Snapshot Manager → Delete.

**H.8 Boot test ISO SpiderOS di VMware**

Lihat Phase 3.1-alt (BIOS) dan 3.2 (UEFI via clone).

**H.9 Tips khusus VMware**

| Masalah | Solusi |
|---|---|
| Layar hitam saat boot | Matikan 3D acceleration di Settings → Display |
| Mouse "terperangkap" di VM | Tekan `Ctrl+Alt` untuk lepas |
| Shared folder tidak muncul | `sudo vmhgfs-fuse .host:/ /mnt/hggs -o allow_other` |
| Performa lambat | Naikkan RAM ke 6–8 GB, enable 3D acceleration, pastikan host tidak low disk |
| VM tidak bisa boot UEFI | Cek Settings → Options → Advanced → Firmware type: UEFI |
| ISO tidak terdeteksi | Settings → CD/DVD → Use ISO image file, centang "Connect at power on" |

**H.10 Perbedaan penting VirtualBox vs VMware**

| Aspek | VirtualBox | VMware Workstation Pro |
|---|---|---|
| Guest tools | `virtualbox-guest-*` | `open-vm-tools` (dari repo Debian) |
| Shared folder path | `/media/sf_NAMAFOLDER` | `/mnt/hggs/NAMAFOLDER` |
| Mount shared folder manual | `mount -t vboxsf ...` | `vmhgfs-fuse .host:/ /mnt/hggs -o allow_other` |
| Snapshot location | Machine menu | VM → Snapshot menu |
| Hotkey lepas mouse | Right Ctrl | Ctrl+Alt |

---

**Akhir modul. Mulai dari Phase 1, jangan loncat. Baseline harus boot dulu sebelum tema ditambahkan.** 🕷️