## PHASE 1: Persiapan Lingkungan, Repo, dan Dokumentasi

**Tujuan:** VM siap, tools terpasang, repo Git berdiri, template dokumentasi siap.
**Output:** VM bersih dengan snapshot, repo `~/spideros`, folder dokumentasi.

### 1.0 Pilih Hypervisor: VirtualBox atau VMware Workstation Pro

Kamu bisa pakai salah satu. Keduanya mendukung BIOS dan UEFI. Panduan lengkap ada di **Lampiran G (VirtualBox)** dan **Lampiran H (VMware Workstation Pro)**.

**Perbandingan singkat:**

| Aspek | VirtualBox | VMware Workstation Pro |
|---|---|---|
| Lisensi | FOSS (GPL v2) | Gratis untuk penggunaan personal (sejak Mei 2024 via Broadcom) |
| Performa | Baik | Sedikit lebih cepat untuk grafis 3D |
| Shared folder | Via Guest Additions | Via VMware Tools / open-vm-tools |
| Snapshot | Mudah | Mudah |
| Ketersediaan | `apt install virtualbox` di Linux, installer `.exe` di Windows | Download dari Broadcom portal |

> 💡 **Rekomendasi**: pakai yang sudah terpasang di laptop kamu. Kalau belum ada keduanya, VirtualBox lebih cepat dipasang. Kalau ingin performa lebih baik dan punya akun Broadcom, pilih VMware.

### 1.1 Cek spesifikasi VM

Matikan VM Debian (kalau sudah ada), buka hypervisor, Settings dan pastikan:

| Pengaturan | Nilai |
|---|---|
| Base Memory | 4096 MB atau lebih (build lebih lancar) |
| Processors | 4 (atau setengah dari core laptop) |
| Video Memory | 128 MB |
| Network | NAT (default) |
| Storage | Disk minimal 40 GB |

Nyalakan VM lalu cek dari dalam:

```bash
df -h /          # sisa disk, idealnya > 25 GB bebas
free -h          # RAM
nproc            # jumlah core
lsb_release -a   # harus Debian 13 (trixie)
```

Kalau `lsb_release` tidak ada: `cat /etc/os-release`.

### 1.2 Pastikan punya sudo

```bash
sudo -v
```

Kalau muncul error "not in the sudoers file":

```bash
su -                              # masuk root pakai password root
apt install -y sudo
usermod -aG sudo NAMA_USER_KAMU
exit
# logout lalu login ulang supaya grup baru aktif
```

### 1.3 Snapshot pertama

- **VirtualBox:** Machine → Take Snapshot → nama `P1-clean-start`.
- **VMware:** VM → Snapshot → Take Snapshot → nama `P1-clean-start`.

### 1.4 Update sistem dan install tools

```bash
sudo apt update
sudo apt full-upgrade -y
sudo apt install -y live-build debootstrap squashfs-tools xorriso isolinux \
  syslinux-utils syslinux-common mtools dosfstools git build-essential \
  imagemagick librsvg2-bin jp2a neofetch htop curl wget rsync tree openssh-server
```

**Install Rust (untuk Phase 9 Spider-Sense):**

```bash
sudo apt install -y cargo rustc
```

Cek versi:

```bash
lb --version        # di Debian 13 biasanya 1:20250xxx
xorriso --version | head -1
git --version
cargo --version     # harus muncul
rustc --version
```

Pastikan jam VM benar (jam salah bikin error "Release file not valid yet"):

```bash
timedatectl         # "System clock synchronized: yes"
```

Kalau `no`: `sudo timedatectl set-ntp true`.

### 1.5 Cara memindahkan file antara VM dan Windows

Kamu akan butuh memindahkan ISO dari VM ke Windows (karena ISO diuji di VM baru dan USB di Windows). Pilih salah satu sesuai hypervisor.

#### Opsi A: Shared Folder — VirtualBox

```bash
# Di VM:
sudo apt install -y build-essential dkms linux-headers-$(uname -r)
```

Menu VirtualBox: **Devices → Insert Guest Additions CD image**, lalu:

```bash
sudo mkdir -p /mnt/cdrom
sudo mount /dev/cdrom /mnt/cdrom
sudo sh /mnt/cdrom/VBoxLinuxAdditions.run
sudo reboot
```

Di Windows buat folder, misal `D:\SpiderOS-Share`. Di VirtualBox: **Settings → Shared Folders → Add**, pilih folder itu, centang **Auto-mount** dan **Make Permanent**.

```bash
sudo usermod -aG vboxsf $USER
# logout lalu login ulang
ls /media/sf_SpiderOS-Share
```

#### Opsi A: Shared Folder — VMware Workstation Pro

```bash
# Di VM (pakai open-vm-tools, lebih stabil daripada VMware Tools legacy):
sudo apt install -y open-vm-tools open-vm-tools-desktop
sudo reboot
```

Di VMware: **VM → Settings → Options → Shared Folders → Always enabled → Add**, pilih folder Windows (misal `D:\SpiderOS-Share`), centang **Enable this share**.

```bash
# Di VM, mount otomatis biasanya di /mnt/hggs/
ls /mnt/hggs/
# Kalau tidak muncul, mount manual:
sudo vmhgfs-fuse .host:/ /mnt/hggs -o allow_other
```

#### Opsi B: HTTP server sementara (universal, tanpa tools tambahan)

- **VirtualBox:** Settings → Network → Advanced → Port Forwarding: tambah aturan Protocol TCP, Host Port `8000`, Guest Port `8000`.
- **VMware:** VM → Settings → Network → Port Forwarding, tambahkan aturan yang sama.

Di VM, masuk ke folder yang berisi ISO lalu:

```bash
python3 -m http.server 8000
```

Di Windows buka `http://127.0.0.1:8000` di browser lalu unduh ISO. Hentikan server dengan `Ctrl+C`.

### 1.6 Buat repo Git dan struktur folder

```bash
git config --global user.name "Nama Kamu"
git config --global user.email "email@kamu.com"
mkdir -p ~/spideros && cd ~/spideros
git init
mkdir -p auto \
  config/package-lists \
  config/hooks/live \
  config/includes.chroot_after_packages \
  assets-src assets-final \
  tools/spider-sense \
  scripts dist \
  docs/screenshots
```

Buat `.gitignore`:

```bash
cat > .gitignore <<'EOF'
# artefak build live-build
/chroot/
/binary/
/cache/
/.build/
/config/binary
/config/bootstrap
/config/chroot
/config/common
/config/source
*.iso
*.log
live-image-*
dist/*
!dist/.gitkeep
# Rust
tools/spider-sense/target/
# staging sumber Rust untuk hook chroot
config/includes.chroot_after_packages/opt/spider-sense-src/
EOF
touch dist/.gitkeep
```

Struktur yang akan terbentuk di akhir:

```
spideros/
├── auto/                      # config, build, clean (reproducible)
├── config/
│   ├── package-lists/         # daftar paket
│   ├── hooks/live/            # skrip yang jalan di dalam chroot
│   ├── includes.chroot_after_packages/   # file yang disalin ke root sistem
│   └── bootloaders/           # template boot menu
├── assets-src/                # file sumber desain (.xcf, .svg)
├── assets-final/              # hasil ekspor (png)
├── tools/spider-sense/        # kode Rust
├── scripts/                   # skrip bantu (build.sh dst)
├── dist/                      # ISO hasil build
└── docs/                      # catatan dan screenshot
```

`tools/spider-sense/` adalah rumah proyek Rust: seluruh source, `Cargo.toml`, `Cargo.lock`, dan unit test berada di sana. Folder `config/` hanya menerima salinan sementara saat build melalui skrip staging.

Commit pertama:

```bash
git add -A
git commit -m "chore: struktur awal repo"
```

### 1.7 Template dokumentasi

Buat `docs/catatan.md`:

```bash
cat > docs/catatan.md <<'EOF'
# Catatan Pengerjaan SpiderOS
Format tiap entri: tanggal, phase, apa yang dilakukan, command, hasil, nama file screenshot.
| Tanggal | Phase | Langkah | Command | Hasil | Screenshot |
| :-- | :-- | :-- | :-- | :-- | :-- |
|  |  |  |  |  |  |
EOF
```

Aturan nama screenshot (simpan di `docs/screenshots/`):
`P{phase}-{topik}-{nomor}.png`
contoh: `P02-lb-config-01.png`, `P03-boot-bios-02.png`, `P11-usb-uefi-01.png`

### 1.8 Checklist Phase 1

- [x] VM jalan, RAM 4 GB atau lebih, disk bebas > 25 GB
- [x] `sudo` berfungsi
- [x] Semua tools terpasang, `lb --version` keluar, `cargo --version` keluar
- [x] Jam VM sinkron
- [x] Cara memindahkan file VM ke Windows sudah diuji (coba salin file kecil)
- [x] Repo Git ada dan ada 1 commit
- [x] Template catatan siap, peran 4 orang sudah dibagi
- [x] Snapshot `P1-clean-start` ada

✅ **Selesai kalau:** semua kotak di atas tercentang.

---

