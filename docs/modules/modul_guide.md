# 🕷️ SpiderOS: Modul End-to-End Remastering Debian 13 Trixie dengan live-build

Modul praktik lengkap dari nol sampai ISO jadi, diuji di USB asli, dan siap dilaporkan. Tema: Spider-Man. Basis: **Debian 13 (trixie)**. Tool utama: **live-build**. Desktop: **XFCE 4.20**. Tool bonus: **Rust (Spider-Sense)**.

---

## 0. Pengantar

### 0.1 Hasil akhir yang dituju
- Satu file ISO hybrid (`spideros-vX.Y.iso`) yang bisa boot di BIOS/Legacy dan UEFI, dari flashdisk.
- Tampilan bertema Spider-Man: wallpaper, tema GTK, ikon, login screen, boot menu, animasi boot (Plymouth), terminal.
- Branding sistem: `neofetch`, prompt bash, `/etc/os-release`.
- Aplikasi dasar yang ringan dan berguna.
- **(Bonus)** tool Spider-Sense (**Rust**) yang membaca `/proc`.
- Dokumentasi lengkap yang langsung jadi bahan BAB III, IV, dan V.

### 0.2 Konvensi di modul ini

| Penanda | Arti |
|---|---|
| `[BUILD-VM]` | Dijalankan di VM Debian 13 tempat kamu membangun ISO (default kalau tidak ditulis) |
| `[HOST]` | Dijalankan di Windows (komputer fisik) |
| `[TEST-VM]` | Dijalankan di VM baru yang boot dari ISO hasil build |
| `$` | Perintah sebagai user biasa (pakai `sudo` kalau perlu) |
| 📸 | Ambil screenshot untuk BAB IV |
| ✅ Selesai kalau | Kriteria lulus phase, jangan lanjut sebelum terpenuhi |

Folder kerja: `~/spideros`. Semua perintah dijalankan dari situ kecuali disebut lain.

### 0.3 Peta Phase

| Phase | Isi | Fase RUP | Estimasi effort |
|---|---|---|---|
| 1 | Persiapan lingkungan, repo, dokumentasi, pembagian tugas | Inception | 2–3 jam |
| 2 | Build ISO baseline (tanpa tema) | Elaboration | 3–4 jam (banyak menunggu build) |
| 3 | Uji baseline di VM test (BIOS dan UEFI) | Elaboration | 1–2 jam |
| 4 | Desain aset visual | Construction | 4–8 jam (paralel, oleh desainer) |
| 5 | Paket, user live, konfigurasi sistem, keamanan dasar | Construction | 3–4 jam |
| 6 | Tema desktop (wallpaper, GTK, ikon, login screen) | Construction | 3–5 jam |
| 7 | Branding boot (boot menu, Plymouth) | Construction | 3–4 jam |
| 8 | Branding terminal dan sistem | Construction | 2–3 jam |
| 9 | Spider-Sense (Rust, bonus) | Construction | 3–6 jam |
| 10 | Build ISO v1 dan uji menyeluruh di VM | Construction | 2–3 jam |
| 11 | Flash USB dan uji hardware (UEFI dan Legacy) | Transition | 3–5 jam |
| 12 | Perbaikan bug, ISO v2, freeze | Transition | 2–4 jam |
| 13 | Laporan, panduan pengguna, video demo, presentasi | Transition | 6–10 jam |

Phase 4 bisa jalan paralel dengan Phase 2 dan 3 (desainer tidak perlu menunggu build).

### 0.4 Pembagian peran (4 orang)

| Peran | Tanggung jawab utama | Phase utama |
|---|---|---|
| Builder | Pipeline live-build, hook, konfigurasi, build ISO | 1, 2, 5, 6, 7, 8, 10, 12 |
| Desainer | Semua aset visual | 4 (lalu bantu 6, 7) |
| Tester | VM test, USB, matriks hardware, catat boot sequence | 3, 10, 11 |
| Dokumentator | Catatan, screenshot, laporan, panduan, demo (dan Spider-Sense kalau sempat) | 1, 9, 13 |

Semua anggota tetap mencoba menjalankan ISO minimal sekali supaya bisa menjawab pertanyaan dosen.

### 0.5 Aturan emas
- Snapshot VM sebelum setiap phase besar.
- Commit Git setiap satu perubahan yang berhasil.
- Satu perubahan per build. Jangan mengubah 10 hal lalu build. Kalau gagal, tidak tahu penyebabnya.
- Screenshot dan catat command saat mengerjakan, bukan nanti.
- Baseline harus boot dulu sebelum menambah tema.
- Setelah ISO final di-freeze, tidak ada fitur baru.

---

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

Buat juga `docs/credits.md` untuk mencatat sumber setiap aset (lisensi dan atribusi). Isi tiap kali ada aset dari luar.

### 1.8 Checklist Phase 1

- [ ] VM jalan, RAM 4 GB atau lebih, disk bebas > 25 GB
- [ ] `sudo` berfungsi
- [ ] Semua tools terpasang, `lb --version` keluar, `cargo --version` keluar
- [ ] Jam VM sinkron
- [ ] Cara memindahkan file VM ke Windows sudah diuji (coba salin file kecil)
- [ ] Repo Git ada dan ada 1 commit
- [ ] Template catatan siap, peran 4 orang sudah dibagi
- [ ] Snapshot `P1-clean-start` ada

✅ **Selesai kalau:** semua kotak di atas tercentang.

---

## PHASE 2: Build ISO Baseline (Remaster Pertama)

**Tujuan:** menghasilkan ISO Debian 13 + XFCE polos dari live-build. Belum ada tema. Ini membuktikan pipeline berjalan.
**Output:** satu ISO baseline, `build.log`, repo berisi konfigurasi.

### 2.1 Konsep singkat: bagaimana live-build bekerja

live-build membangun ISO lewat beberapa tahap. Ini juga bahan penjelasan di laporan.

| Tahap | Perintah internal | Apa yang terjadi | Hasil |
|---|---|---|---|
| Config | `lb config` | Membaca opsi dan membuat folder `config/` | File konfigurasi |
| Bootstrap | `lb bootstrap` | `debootstrap` mengunduh sistem Debian dasar | Folder `chroot/` |
| Chroot | `lb chroot` | Memasang paket, menyalin file, menjalankan hook | `chroot/` terisi sistem lengkap |
| Binary | `lb binary` | Memadatkan `chroot/` jadi `filesystem.squashfs`, memasang bootloader, membuat ISO | `live-image-amd64.hybrid.iso` |

Semua tahap dijalankan sekaligus oleh `lb build`.

### 2.2 Siapkan skrip `auto/`

Skrip `auto/` membuat konfigurasi tersimpan di Git dan bisa dijalankan ulang.

```bash
cd ~/spideros
ls /usr/share/doc/live-build/examples/auto/
```

Kalau ada isinya (`build`, `clean`, `config`):

```bash
cp /usr/share/doc/live-build/examples/auto/* auto/
chmod +x auto/*
```

Kalau tidak ada, buat manual:

```bash
cat > auto/build <<'EOF'
#!/bin/sh
set -e
lb build noauto "${@}"
EOF
cat > auto/clean <<'EOF'
#!/bin/sh
set -e
lb clean noauto "${@}"
rm -f config/binary config/bootstrap config/chroot config/common config/source
EOF
chmod +x auto/build auto/clean
```

### 2.3 Tulis `auto/config`

```bash
cat > auto/config <<'EOF'
#!/bin/sh
set -e
lb config noauto \
    --mode debian \
    --distribution trixie \
    --architectures amd64 \
    --linux-flavours amd64 \
    --archive-areas "main contrib non-free non-free-firmware" \
    --binary-images iso-hybrid \
    --debian-installer none \
    --memtest none \
    --bootappend-live "boot=live components quiet splash username=spider hostname=spideros locales=en_US.UTF-8 keyboard-layouts=us timezone=Asia/Jakarta" \
    --iso-application "SpiderOS" \
    --iso-publisher "Tim Sistem Operasi UNSRI" \
    --iso-volume "SPIDEROS" \
    "${@}"
EOF
chmod +x auto/config
```

Arti opsi penting (catat untuk BAB III):

| Opsi | Arti |
|---|---|
| `--distribution trixie` | Basis Debian 13 |
| `--archive-areas` | Repositori yang dipakai. `non-free-firmware` perlu untuk driver WiFi dan GPU |
| `--binary-images iso-hybrid` | ISO yang bisa dibakar ke USB dan boot di BIOS dan UEFI |
| `--debian-installer none` | ISO hanya live (tanpa installer ke disk), sesuai batasan masalah |
| `--bootappend-live` | Parameter kernel: nama user `spider`, hostname `spideros`, zona waktu Jakarta |
| `splash` | Mengaktifkan animasi boot Plymouth nanti |

Login default live: user `spider`, password `live`.

### 2.4 Daftar paket baseline

```bash
cat > config/package-lists/spideros.list.chroot <<'EOF'
xorg
lightdm
lightdm-gtk-greeter
xfce4
xfce4-terminal
network-manager
network-manager-gnome
EOF
```

`live-boot` dan `live-config` ditambahkan otomatis oleh live-build, tidak perlu ditulis.

### 2.5 Generate konfigurasi

```bash
sudo lb clean --purge 2>/dev/null || true
sudo lb config
ls config/
```

📸 Screenshot output `lb config` dan isi folder `config/` → `P02-lb-config-01.png`.

Cek bahwa opsi terbaca:

```bash
grep -E "LB_DISTRIBUTION|LB_ARCHIVE_AREAS|LB_BINARY_IMAGES|LB_BOOTAPPEND_LIVE" config/common config/binary 2>/dev/null
```

### 2.6 Build

Build harus dengan root:

```bash
sudo lb build 2>&1 | tee build.log
```

Waktu: sekitar 10–40 menit tergantung internet dan CPU.
Ukuran unduhan: sekitar 700 MB–1 GB.
Selama build, biarkan VM jalan, jangan sleep.

📸 Screenshot bagian akhir build (tulisan sukses) → `P02-build-selesai-01.png`.

#### 2.6.1 Kalau mau iterasi lebih cepat nanti

| Perintah | Efek |
|---|---|
| `sudo lb clean` | Hapus hasil build tapi **simpan cache** unduhan (build ulang jauh lebih cepat) |
| `sudo lb clean --binary` | Hapus tahap binary saja (kalau hanya ubah boot menu) |
| `sudo lb clean --purge` | Hapus semua termasuk cache dan konfigurasi hasil generate (mulai dari nol) |

### 2.7 Verifikasi hasil

```bash
ls -lh *.iso
file live-image-amd64.hybrid.iso
sha256sum live-image-amd64.hybrid.iso
xorriso -indev live-image-amd64.hybrid.iso -report_el_torito plain 2>&1 | head -30
```

Yang diharapkan:
- Ukuran sekitar 700 MB–1.2 GB.
- `file` menampilkan "DOS/MBR boot sector" (tanda ISO hybrid).
- Laporan El Torito menampilkan entri boot BIOS dan UEFI (ada entri EFI). Ini bukti ISO mendukung dua mode boot, bagus untuk rumusan masalah ke-4.

### 2.8 Skrip build bantu

Supaya build rapi dan ISO diberi nama versi:

```bash
cat > scripts/build.sh <<'EOF'
#!/bin/bash
# Pemakaian: ./scripts/build.sh v0.1
set -e
VER="${1:-dev}"
cd "$(dirname "$0")/.."
sudo lb clean
sudo lb config
sudo lb build 2>&1 | tee build.log
ISO=$(ls -1 live-image-*.iso | head -n1)
OUT="dist/spideros-${VER}.iso"
mv "$ISO" "$OUT"
sha256sum "$OUT" | tee "${OUT}.sha256"
ls -lh "$OUT"
EOF
chmod +x scripts/build.sh
```

### 2.9 Troubleshooting Phase 2

| Gejala | Penyebab umum | Solusi |
|---|---|---|
| `E: Unable to locate package ...` | Nama paket salah atau area repositori kurang | Cek ejaan, cek `--archive-areas` |
| `Failed to fetch` / timeout | Internet atau mirror lambat | Ulangi `sudo lb build`, atau ganti mirror (lihat Lampiran E) |
| `Release file ... is not valid yet` | Jam VM salah | `sudo timedatectl set-ntp true` lalu build ulang |
| `No space left on device` | Disk penuh | `df -h`, hapus `chroot/` & `cache/` lama dengan `sudo lb clean --purge`, perbesar disk |
| Build berhenti di `mksquashfs` | RAM kurang | Naikkan RAM VM ke 4 GB atau lebih |
| Error `locale` warning | Tidak fatal | Abaikan, selama build selesai |
| `lb: command not found` | live-build belum terpasang | `sudo apt install live-build` |
| Hasil ISO tidak ada | Build gagal sebelum tahap binary | Baca 50 baris terakhir `build.log` |

Cari error di log:

```bash
grep -n -i -E "error|^E:|failed" build.log | head -30
```

### 2.10 Commit dan snapshot

```bash
git add -A
git commit -m "feat: baseline live-build debian 13 xfce"
```

Snapshot: `P2-baseline-built`. Salin ISO ke Windows (lihat 1.5).

### 2.11 Checklist Phase 2

- [ ] `auto/config`, paket list, `lb config` sukses
- [ ] `sudo lb build` selesai tanpa error fatal
- [ ] ISO ada, `file` menunjukkan hybrid, El Torito menunjukkan BIOS dan UEFI
- [ ] `build.log` tersimpan, screenshot diambil
- [ ] Commit Git dan snapshot dibuat
- [ ] ISO sudah dipindah ke Windows

✅ **Selesai kalau:** ISO baseline ada dan sudah ada di Windows siap diuji.

---

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
| BIOS | Menu ISOLINUX/Syslinux (biru, gaya lama) | Live system |
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
- [ ] Muncul desktop XFCE tanpa error
- [ ] Terminal bisa dibuka
- [ ] Internet jalan (`ping -c 3 deb.debian.org`)
- [ ] Resolusi layar menyesuaikan jendela VM
- [ ] Shutdown/reboot normal

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

- [ ] Boot sukses di VM BIOS
- [ ] Boot sukses di VM UEFI
- [ ] Semua tes fungsional lulus
- [ ] Screenshot tersimpan, hasil dicatat di `docs/catatan.md`

✅ **Selesai kalau:** ISO baseline boot di kedua mode. Setelah ini baru boleh menambah tema. Ini juga titik di mana sekitar 70% risiko teknis sudah lewat.

---

## PHASE 4: Desain Aset Visual

**Tujuan:** menyiapkan semua gambar yang dibutuhkan Phase 6, 7, dan 8.
**Output:** folder `assets-final/` berisi PNG siap pakai, `docs/credits.md` terisi.
**Dikerjakan oleh:** Desainer (bisa mulai sejak Phase 1).

### 4.1 Konsep visual

Putuskan dulu dan tulis di `docs/desain.md`:

| Elemen | Contoh keputusan |
|---|---|
| Nama distro | SpiderOS (boleh diganti, asal konsisten) |
| Palet utama | Merah `#E23636`, Biru tua `#1D3E91`, Biru sangat tua `#0A1E5E`, Hitam `#0B0C10`, Putih `#F5F5F5` |
| Motif | Jaring laba-laba, siluet kota, garis diagonal |
| Gaya logo | Emblem laba-laba bergaya sendiri (bukan logo resmi) |
| Font | Satu sans-serif, misal Noto Sans |

Hak cipta: logo, karakter, dan gambar resmi Spider-Man dilindungi hak cipta. Buat desain sendiri bergaya Spider-Man (siluet, motif jaring, palet merah-biru). Jika memakai aset luar, pilih yang lisensinya bebas (CC0 atau CC-BY), lalu catat sumber dan lisensinya di `docs/credits.md`. Tulis di laporan bahwa proyek ini untuk keperluan edukasi non-komersial.

### 4.2 Daftar aset dan spesifikasi

| No | Aset | Ukuran | Format | Dipakai di |
|---|---|---|---|---|
| 1 | `wallpaper.png` | 1920x1080 | PNG/JPG | Desktop |
| 2 | `login.png` | 1920x1080 | PNG/JPG | Layar login LightDM |
| 3 | `logo.png` | 400x400 (transparan) | PNG | Plymouth, neofetch, menu |
| 4 | `logo.svg` | vektor | SVG | Sumber, ikon |
| 5 | `grub-bg.png` | 1920x1080 | PNG | Boot menu UEFI (GRUB) |
| 6 | `isolinux-splash.png` | 640x480 | PNG | Boot menu BIOS |
| 7 | `spideros-logo-small.png` | 128x128 | PNG | Foto profil user, ikon menu |
| 8 | (opsional) `frame-000..N.png` | 400x400 | PNG | Animasi Plymouth berbasis frame |

### 4.3 Alat dan cara membuat

- **Inkscape** untuk logo dan motif vektor (simpan sumber `.svg` di `assets-src/`).
- **GIMP** untuk wallpaper dan efek (simpan `.xcf` di `assets-src/`).
- Ekspor final ke `assets-final/`.

Tips wallpaper yang aman: gradient merah ke biru gelap, motif jaring dari garis-garis radial dan lingkaran konsentris, siluet gedung di bawah, logo di tengah atau pojok.

### 4.4 Konversi dan optimasi dengan ImageMagick

Contoh dari satu master 1920x1080:

```bash
cd ~/spideros/assets-final
# Ukuran persis untuk boot menu BIOS (640x480)
convert wallpaper.png -resize 640x480^ -gravity center -extent 640x480 isolinux-splash.png
# Versi login dan GRUB (boleh sama dengan wallpaper)
cp wallpaper.png login.png
cp wallpaper.png grub-bg.png
# Logo kecil
convert logo.png -resize 128x128 spideros-logo-small.png
# Cek ukuran dan dimensi
identify *.png
```

Jaga total ukuran aset di bawah sekitar 10 MB supaya ISO tetap ringan. Kalau wallpaper terlalu besar, simpan sebagai JPG kualitas 85: `convert wallpaper.png -quality 85 wallpaper.jpg`.

### 4.5 Aset animasi Plymouth (opsional, terakhir)

Default di modul ini: logo statis dengan efek berdenyut (dibuat lewat skrip, tidak perlu frame). Kalau ingin animasi frame:
- 30–60 frame, nama `frame-000.png` sampai `frame-059.png`.
- Ukuran 400x400, latar transparan.
- Diputar berulang. Ini opsional dan hanya dikerjakan kalau semua phase lain sudah aman.

### 4.6 Serah terima ke Builder

Desainer commit ke Git:

```bash
git add assets-src assets-final docs/credits.md docs/desain.md
git commit -m "assets: wallpaper, logo, boot splash"
```

### 4.7 Checklist Phase 4

- [ ] Konsep dan palet tertulis di `docs/desain.md`
- [ ] Aset nomor 1–7 selesai dan sesuai ukuran
- [ ] File sumber (.svg/.xcf) tersimpan di `assets-src/`
- [ ] `docs/credits.md` terisi (atau tertulis "semua aset buatan sendiri")
- [ ] Sudah di-commit

✅ **Selesai kalau:** Builder bisa mengambil semua aset dari `assets-final/` tanpa bertanya.

---

## PHASE 5: Paket, User Live, Konfigurasi Sistem, dan Keamanan Dasar

**Tujuan:** mengubah ISO polos menjadi sistem yang berguna: aplikasi, user, locale, firewall, driver.
**Output:** ISO dengan paket lengkap dan konfigurasi dasar.

### 5.1 Daftar paket lengkap

Ganti isi `config/package-lists/spideros.list.chroot`:

```bash
cat > config/package-lists/spideros.list.chroot <<'EOF'
# --- Dasar grafis ---
xorg
lightdm
lightdm-gtk-greeter
dbus-x11
policykit-1-gnome
xdg-user-dirs
gvfs
gvfs-backends

# --- Desktop XFCE 4.20 ---
xfce4
xfce4-terminal
xfce4-whiskermenu-plugin
xfce4-pulseaudio-plugin
xfce4-power-manager
xfce4-notifyd
xfce4-screenshooter
xfce4-taskmanager
thunar-archive-plugin
mousepad
ristretto
file-roller

# --- Jaringan ---
network-manager
network-manager-gnome

# --- Audio ---
pulseaudio
pavucontrol

# --- Aplikasi ---
firefox-esr
vlc

# --- Tema dan font ---
arc-theme
papirus-icon-theme
librsvg2-common
fonts-noto-core
fonts-firacode

# --- Branding dan alat terminal ---
plymouth
plymouth-themes
neofetch
htop
git
curl
wget
nano
sudo

# --- Keamanan ---
ufw

# --- Firmware (Trixie: meta-package firmware-linux sudah lengkap) ---
firmware-linux
firmware-linux-nonfree
firmware-misc-nonfree
firmware-iwlwifi
firmware-realtek
firmware-atheros
firmware-brcm80211
firmware-amd-graphics
intel-microcode
amd64-microcode
EOF
```

**Catatan Trixie:**
- Daftar ini sengaja eksplisit (tanpa `task-xfce-desktop`) karena task Debian menarik paket besar seperti LibreOffice. Dengan begini sistem lebih ringan, sesuai klaim di proposal.
- Meta-package `firmware-linux` di Trixie sudah mencakup banyak driver yang di Bookworm harus dipasang manual. Ini bikin ISO lebih kompatibel *out-of-the-box*.
- Driver NVIDIA proprietary tidak dimasukkan (pakai driver bawaan `nouveau`). Ini bagus untuk kompatibilitas, tulis di laporan sebagai batasan.

### 5.2 Mencari paket yang berat (opsional)

Setelah build, cek isi chroot:

```bash
sudo chroot chroot dpkg-query -Wf '${Installed-Size}\t${Package}\n' | sort -n | tail -25
```

Kalau ada paket besar yang tidak perlu, hapus dari daftar atau purge lewat hook.

### 5.3 Hook pertama: konfigurasi dasar sistem

Hook adalah skrip yang berjalan di dalam chroot saat build. Nama harus berakhiran `.hook.chroot` dan harus executable.

```bash
cat > config/hooks/live/0050-system-config.hook.chroot <<'EOF'
#!/bin/bash
set -e
# Browser default
update-alternatives --set x-www-browser /usr/bin/firefox-esr || true
# Firewall: aktif saat boot, tolak koneksi masuk
sed -i 's/^ENABLED=.*/ENABLED=yes/' /etc/ufw/ufw.conf || true
systemctl enable ufw || true
# Bersihkan cache apt agar ISO lebih kecil
apt-get clean
rm -rf /var/lib/apt/lists/*
EOF
chmod +x config/hooks/live/0050-system-config.hook.chroot
```

Catatan: `ufw` secara default menolak koneksi masuk. Kalau tulisan `|| true` bikin penasaran: itu agar build tidak gagal total kalau satu langkah kecil bermasalah. Setelah stabil, kamu boleh menghapusnya agar error terlihat jelas.

### 5.4 Pengaturan user live dan parameter boot

Sudah diatur lewat `--bootappend-live` di `auto/config`. Penjelasan untuk laporan:

| Parameter | Arti |
|---|---|
| `username=spider` | Nama user live (password default `live`) |
| `hostname=spideros` | Nama komputer |
| `timezone=Asia/Jakarta` | Zona waktu WIB |
| `keyboard-layouts=us` | Tata letak keyboard |
| `locales=en_US.UTF-8` | Bahasa sistem (lihat Lampiran E kalau mau bahasa Indonesia) |
| `quiet splash` | Boot tenang dan aktifkan Plymouth |

User `spider` otomatis masuk grup `sudo` tanpa password di sesi live (perilaku standar live-config). Auto-login desktop juga aktif. Kalau ingin layar login tampil (supaya login screen bertema bisa dipamerkan), tambahkan `nox11autologin` ke `--bootappend-live` (lalu login manual `spider` / `live`).

### 5.5 Build dan uji

```bash
cd ~/spideros
./scripts/build.sh v0.2
```

Salin ISO ke Windows, boot di VM test BIOS dan UEFI. Cek:

```bash
# [TEST-VM]
firefox-esr --version
vlc --version | head -1
sudo ufw status          # Status: active
free -h
```

📸 `P05-apps-01.png` (menu aplikasi), `P05-ufw-01.png`.

### 5.6 Commit dan snapshot

```bash
git add -A
git commit -m "feat: paket lengkap, firewall, firmware"
```

Snapshot: `P5-packages-done`.

### 5.7 Checklist Phase 5

- [ ] Paket lengkap terpasang, ISO v0.2 boot di BIOS dan UEFI
- [ ] Firefox, VLC, file manager, terminal jalan
- [ ] `ufw` aktif
- [ ] Ukuran ISO dicatat (target di bawah sekitar 2 GB)
- [ ] Commit dan snapshot dibuat

✅ **Selesai kalau:** sistem bisa dipakai harian dasar (browsing, media, file).

---

## PHASE 6: Tema Desktop (Wallpaper, GTK, Ikon, Login Screen)

**Tujuan:** tampilan desktop benar-benar bertema Spider-Man.
**Output:** desktop dengan wallpaper, tema warna merah-biru, ikon, dan login screen khusus.

### 6.1 Konsep: bagaimana file masuk ke ISO

Folder `config/includes.chroot_after_packages/` meniru struktur root (`/`) sistem. File di dalamnya disalin ke sistem hasil build.

| Letakkan di repo | Menjadi di sistem |
|---|---|
| `.../usr/share/backgrounds/spiderman/wallpaper.png` | `/usr/share/backgrounds/spiderman/wallpaper.png` |
| `.../etc/skel/.config/xfce4/...` | Disalin ke home user live saat user dibuat |
| `.../etc/lightdm/lightdm-gtk-greeter.conf` | Konfigurasi login screen |

Buat struktur:

```bash
cd ~/spideros
INC=config/includes.chroot_after_packages
mkdir -p $INC/usr/share/backgrounds/spiderman
mkdir -p $INC/usr/share/pixmaps
mkdir -p $INC/usr/local/bin
mkdir -p $INC/etc/lightdm
mkdir -p $INC/etc/skel/.config/xfce4/xfconf/xfce-perchannel-xml
mkdir -p $INC/etc/skel/.config/autostart
```

### 6.2 Salin aset

```bash
INC=config/includes.chroot_after_packages
cp assets-final/wallpaper.png $INC/usr/share/backgrounds/spiderman/wallpaper.png
cp assets-final/login.png     $INC/usr/share/backgrounds/spiderman/login.png
cp assets-final/spideros-logo-small.png $INC/usr/share/pixmaps/spideros-logo.png
```

### 6.3 Tema GTK "SpiderOS" lewat hook

Cara: salin tema Arc-Dark lalu ganti warna aksen biru menjadi merah.

```bash
cat > config/hooks/live/0100-theme.hook.chroot <<'EOF'
#!/bin/bash
set -e
SRC=/usr/share/themes/Arc-Dark
DST=/usr/share/themes/SpiderOS
if [ -d "$SRC" ]; then
  rm -rf "$DST"
  cp -r "$SRC" "$DST"
  # Ganti aksen biru Arc (#5294E2) menjadi merah Spider-Man (#E23636)
  find "$DST" -type f \( -name '*.css' -o -name 'gtkrc' -o -name '*.svg' -o -name 'themerc' -o -name '*.scss' \) \
    -exec sed -i -e 's/#5294E2/#E23636/Ig' -e 's/rgba(82, *148, *226/rgba(226, 54, 54/Ig' {} +
  # Ganti nama tema
  sed -i 's/Arc-Dark/SpiderOS/g' "$DST/index.theme" || true
fi
EOF
chmod +x config/hooks/live/0100-theme.hook.chroot
```

Catatan jujur: sebagian elemen kecil (misalnya aset PNG tertentu) mungkin masih berwarna biru bawaan. Itu bisa diterima. Kalau warnanya bermasalah, fallback-nya pakai `Arc-Dark` apa adanya dan ganti nama tema di XML di langkah 6.4 menjadi `Arc-Dark`.

### 6.4 Pengaturan tema XFCE (XML)

Tema GTK dan ikon (`xsettings.xml`):

```bash
INC=config/includes.chroot_after_packages
XML=$INC/etc/skel/.config/xfce4/xfconf/xfce-perchannel-xml
cat > $XML/xsettings.xml <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<channel name="xsettings" version="1.0">
  <property name="Net" type="empty">
    <property name="ThemeName" type="string" value="SpiderOS"/>
    <property name="IconThemeName" type="string" value="Papirus-Dark"/>
  </property>
  <property name="Gtk" type="empty">
    <property name="FontName" type="string" value="Noto Sans 10"/>
    <property name="MonospaceFontName" type="string" value="Fira Code 10"/>
  </property>
</channel>
EOF
```

Tema window manager (`xfwm4.xml`):

```bash
cat > $XML/xfwm4.xml <<'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<channel name="xfwm4" version="1.0">
  <property name="general" type="empty">
    <property name="theme" type="string" value="SpiderOS"/>
    <property name="title_font" type="string" value="Noto Sans Bold 9"/>
  </property>
</channel>
EOF
```

### 6.5 Wallpaper lewat skrip autostart

Nama output monitor beda di tiap komputer (`Virtual-1` di VM, `eDP-1` di laptop), jadi wallpaper di-set lewat skrip yang membaca nama monitor.

```bash
INC=config/includes.chroot_after_packages
cat > $INC/usr/local/bin/spideros-wallpaper <<'EOF'
#!/bin/bash
IMG="/usr/share/backgrounds/spiderman/wallpaper.png"
MARK="$HOME/.cache/spideros-wallpaper-set"
[ -f "$MARK" ] && exit 0
[ -f "$IMG" ] || exit 0
sleep 3
OUTS="$(xrandr --query | awk '/ connected/ {print $1}') 0"
for out in $OUTS; do
  for ws in 0 1 2 3; do
    P="/backdrop/screen0/monitor${out}/workspace${ws}"
    xfconf-query -c xfce4-desktop -p "$P/last-image"  -n -t string -s "$IMG"
    xfconf-query -c xfce4-desktop -p "$P/image-style" -n -t int -s 5
    xfconf-query -c xfce4-desktop -p "$P/color-style" -n -t int -s 0
  done
done
mkdir -p "$(dirname "$MARK")"
touch "$MARK"
EOF
chmod +x $INC/usr/local/bin/spideros-wallpaper

cat > $INC/etc/skel/.config/autostart/spideros-wallpaper.desktop <<'EOF'
[Desktop Entry]
Type=Application
Name=SpiderOS Wallpaper
Exec=/usr/local/bin/spideros-wallpaper
X-GNOME-Autostart-enabled=true
EOF
```

Skrip hanya jalan sekali per user (ada penanda), jadi user tetap bebas mengganti wallpaper sendiri.

### 6.6 Login screen (LightDM)

```bash
INC=config/includes.chroot_after_packages
cat > $INC/etc/lightdm/lightdm-gtk-greeter.conf <<'EOF'
[greeter]
background=/usr/share/backgrounds/spiderman/login.png
theme-name=SpiderOS
icon-theme-name=Papirus-Dark
font-name=Noto Sans 11
user-background=false
default-user-image=/usr/share/pixmaps/spideros-logo.png
indicators=~host;~spacer;~clock;~spacer;~session;~language;~a11y;~power
EOF
```

Login screen hanya terlihat kalau auto-login dimatikan (`nox11autologin`, lihat 5.4). Untuk demo, aktifkan saat ingin memamerkannya.

### 6.7 Panel dan tweak lain (opsional)

Layout panel XFCE disimpan di `xfce4-panel.xml`. Cara paling mudah mengubahnya:
1. Boot ISO di VM test, atur panel lewat GUI (klik kanan panel, Panel Preferences).
2. Lihat file hasilnya: `~/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-panel.xml`.
3. Salin isinya ke `$INC/etc/skel/.config/xfce4/xfconf/xfce-perchannel-xml/xfce4-panel.xml` di repo, lalu build ulang.

Kalau tidak sempat, lewati. Panel default sudah rapi.

### 6.8 Build dan verifikasi

```bash
./scripts/build.sh v0.3
```

Cek di `[TEST-VM]`:
- [ ] Wallpaper Spider-Man tampil
- [ ] Tema jendela dan tombol berwarna merah gelap
- [ ] Ikon Papirus tampil
- [ ] (Jika `nox11autologin`) login screen bertema tampil

Cek tanpa GUI kalau ada masalah:

```bash
ls /usr/share/themes | grep Spider
ls /usr/share/backgrounds/spiderman
xfconf-query -c xsettings -p /Net/ThemeName
xrandr --query | grep " connected"
```

📸 `P06-desktop-01.png`, `P06-login-01.png`, `P06-tema-01.png`.

### 6.9 Troubleshooting Phase 6

| Gejala | Penyebab | Solusi |
|---|---|---|
| Wallpaper tidak berubah | Skrip tidak jalan atau penanda sudah ada | Hapus `~/.cache/spideros-wallpaper-set` lalu jalankan `spideros-wallpaper` manual, lihat errornya |
| Tema tidak berubah, tetap putih | Nama tema salah atau hook gagal | Cek `ls /usr/share/themes`, cek `build.log` untuk hook 0100 |
| File di includes tidak ada di sistem | Nama folder includes tidak dikenali | Pastikan pakai `config/includes.chroot_after_packages` |
| Skrip tidak executable | Lupa `chmod +x` sebelum commit | `chmod +x` lalu commit ulang dan build ulang |
| Ikon tetap bawaan | `IconThemeName` salah | Pastikan `papirus-icon-theme` ada di daftar paket |

### 6.10 Commit dan snapshot

```bash
git add -A
git commit -m "feat: tema desktop spiderman"
```

Snapshot: `P6-theme-done`.

### 6.11 Checklist Phase 6

- [ ] Wallpaper, tema, ikon, login screen sesuai desain
- [ ] Tampil benar di VM BIOS dan UEFI
- [ ] Screenshot lengkap
- [ ] Commit dan snapshot dibuat

✅ **Selesai kalau:** desktop jelas bertema Spider-Man tanpa error.

---

## PHASE 7: Branding Boot (Boot Menu dan Plymouth)

**Tujuan:** layar yang muncul sebelum desktop (menu boot dan animasi loading) bertema Spider-Man.
**Output:** boot menu BIOS dan UEFI bertema, animasi Plymouth.

Urutan tampilan saat boot, agar jelas yang mana yang diubah:

```
Firmware (BIOS/UEFI) → Boot menu (ISOLINUX/GRUB) → Kernel + initramfs → Plymouth (loading) → Login/Desktop
                         ^ diubah di 7.2                                ^ diubah di 7.1
```

### 7.1 Plymouth (animasi loading)

Plymouth memutar animasi dari gambar dan skrip kecil, bukan video.

**Langkah 1: struktur tema**

```bash
cd ~/spideros
INC=config/includes.chroot_after_packages
PLY=$INC/usr/share/plymouth/themes/spiderman
mkdir -p $PLY
cp assets-final/logo.png $PLY/logo.png
```

**Langkah 2: file `spiderman.plymouth`**

```bash
cat > $PLY/spiderman.plymouth <<'EOF'
[Plymouth Theme]
Name=SpiderOS
Description=Boot splash bertema Spider-Man
ModuleName=script

[script]
ImageDir=/usr/share/plymouth/themes/spiderman
ScriptFile=/usr/share/plymouth/themes/spiderman/spiderman.script
EOF
```

**Langkah 3: skrip animasi (logo berdenyut)**

```bash
cat > $PLY/spiderman.script <<'EOF'
Window.SetBackgroundTopColor(0.04, 0.05, 0.09);
Window.SetBackgroundBottomColor(0.00, 0.08, 0.30);

logo.image = Image("logo.png");
logo.sprite = Sprite(logo.image);
logo.sprite.SetX(Window.GetX() + Window.GetWidth() / 2 - logo.image.GetWidth() / 2);
logo.sprite.SetY(Window.GetY() + Window.GetHeight() / 2 - logo.image.GetHeight() / 2);
logo.sprite.SetZ(10);

state.counter = 0;
fun refresh_callback() {
    state.counter = state.counter + 1;
    opacity = 0.65 + 0.35 * Math.Sin(state.counter / 15.0);
    logo.sprite.SetOpacity(opacity);
}
Plymouth.SetRefreshFunction(refresh_callback);
EOF
```

**Langkah 4: aktifkan lewat hook**

```bash
cat > config/hooks/live/0200-plymouth.hook.chroot <<'EOF'
#!/bin/bash
set -e
plymouth-set-default-theme spiderman
update-initramfs -u -k all
EOF
chmod +x config/hooks/live/0200-plymouth.hook.chroot
```

Parameter `splash` sudah ada di `--bootappend-live`, jadi Plymouth akan tampil.

Verifikasi setelah boot:

```bash
# [TEST-VM]
plymouth-set-default-theme          # harus: spiderman
cat /proc/cmdline | grep splash
```

### 7.2 Boot menu BIOS (ISOLINUX/Syslinux) dan UEFI (GRUB)

**Langkah 1: salin template bootloader live-build ke repo**

```bash
cd ~/spideros
cp -r /usr/share/live/build/bootloaders config/bootloaders
ls config/bootloaders/
```

**Langkah 2: cari file gambar latar**

```bash
find config/bootloaders -iname 'splash*'
```

Ganti file gambar yang ditemukan:
- Untuk ISOLINUX: pakai `assets-final/isolinux-splash.png` (640x480).
- Untuk GRUB: pakai `assets-final/grub-bg.png`. Ukuran/format harus sama dengan file aslinya.

Contoh (sesuaikan path hasil `find`):

```bash
cp assets-final/isolinux-splash.png config/bootloaders/isolinux/splash.png
# GRUB, kalau ada splash.png di foldernya:
cp assets-final/grub-bg.png config/bootloaders/grub-pc/splash.png
```

**Langkah 3: ganti teks judul menu**

```bash
grep -rn -i -E "debian|menu title|menu label|title" config/bootloaders | head -40
sed -i 's/Debian GNU\/Linux/SpiderOS/g' config/bootloaders/isolinux/*.cfg
```

**Langkah 4: warna menu**

Untuk ISOLINUX, ubah warna di `stdmenu.cfg`:

```
menu color border   0 #ffe23636 #00000000 none
menu color title    1;37;44 #ffe23636 #00000000 none
menu color sel      7;37;40 #ffffffff #ffe23636 all
menu color unsel    37;44 #ffffffff #00000000 none
```

Untuk GRUB, cari `menu_color` atau `color_normal` di `config/bootloaders/grub-*/`:

```
set menu_color_normal=white/black
set menu_color_highlight=white/light-red
```

**Langkah 5: timeout menu (opsional)**

Cari `timeout` di file cfg dan atur misalnya 5 detik.

### 7.3 Build dan uji

```bash
./scripts/build.sh v0.4
```

Uji di VM BIOS dan UEFI, lalu cek:
- [ ] Menu BIOS bertema SpiderOS
- [ ] Menu UEFI bertema SpiderOS
- [ ] Animasi/logo Plymouth muncul saat loading
- [ ] Sistem tetap boot sampai desktop

📸 `P07-bootmenu-bios-01.png`, `P07-bootmenu-uefi-01.png`, `P07-plymouth-01.png`.

### 7.4 Troubleshooting Phase 7

| Gejala | Penyebab | Solusi |
|---|---|---|
| Menu boot tidak berubah | Template di `config/bootloaders` tidak terpakai | Pastikan folder ada, lakukan `sudo lb clean --binary` lalu build ulang |
| Boot menu rusak/tidak muncul | Gambar salah format/ukuran | Kembalikan splash default, cek `identify`, ulangi |
| Plymouth tidak muncul | Tema belum aktif, `splash` hilang, atau initramfs tidak diperbarui | Cek `plymouth-set-default-theme`, cek `/proc/cmdline`, cek hook 0200 di `build.log` |
| Build gagal di hook 0200 | `plymouth` tidak terpasang atau nama tema salah | Pastikan paket `plymouth` ada dan nama tema `spiderman` |
| Layar hitam setelah Plymouth | Masalah grafis VM | Coba `nomodeset` sementara, uji di hardware asli |

### 7.5 Commit dan snapshot

```bash
git add -A
git commit -m "feat: boot menu dan plymouth bertema"
```

Snapshot: `P7-boot-branding`.

### 7.6 Checklist Phase 7

- [ ] Boot menu BIOS dan UEFI bertema
- [ ] Plymouth aktif (atau keputusan fallback tercatat)
- [ ] Boot tetap sukses di kedua mode
- [ ] Screenshot, commit, snapshot

✅ **Selesai kalau:** alur boot dari menu sampai desktop konsisten bertema dan tidak ada error.

---

## PHASE 8: Branding Terminal dan Identitas Sistem

**Tujuan:** terminal, `neofetch`, dan identitas sistem terlihat sebagai SpiderOS.
**Output:** `neofetch` dengan ASCII laba-laba, prompt bash custom, `os-release` dan banner login.

### 8.1 Identitas sistem (`os-release`, `issue`, `motd`)

Penting: ubah hanya `NAME`, `PRETTY_NAME`, dan URL. Jangan ubah `ID` dan `VERSION_CODENAME` karena beberapa tool bergantung padanya.

```bash
cat > config/hooks/live/0300-branding.hook.chroot <<'EOF'
#!/bin/bash
set -e
# os-release (di Debian 13 /etc/os-release adalah symlink ke /usr/lib/os-release)
OSR=/usr/lib/os-release
sed -i 's/^NAME=.*/NAME="SpiderOS"/'                                        $OSR
sed -i 's/^PRETTY_NAME=.*/PRETTY_NAME="SpiderOS 1.0 (berbasis Debian 13)"/'  $OSR
sed -i 's/^HOME_URL=.*/HOME_URL="https:\/\/www.debian.org\/"/'               $OSR

# Banner login teks
cat > /etc/issue <<'BANNER'
SpiderOS 1.0 (Debian 13 Trixie) \n \l
BANNER
cp /etc/issue /etc/issue.net

cat > /etc/motd <<'BANNER'
  Selamat datang di SpiderOS.
  Dengan kekuatan besar datang tanggung jawab besar.
  Ketik `sense` untuk memeriksa kondisi sistem.
BANNER
EOF
chmod +x config/hooks/live/0300-branding.hook.chroot
```

### 8.2 `neofetch` dengan ASCII laba-laba

```bash
INC=config/includes.chroot_after_packages
NF=$INC/etc/skel/.config/neofetch
mkdir -p $NF
```

ASCII sederhana (`${c1}` merah, `${c2}` biru):

```bash
cat > $NF/spider.txt <<'EOF'
${c1}      \  \     /  /
       \  \   /  /
  ______\  \_/  /______
  \      \ (${c2}O${c1}) /      /
   `-.___/ / \ \___.-'
      _.-'  |  '-._
     /     / \     \
    /  .-'/   \'-.  \
   '__/  /     \  \__'
${c2}       SpiderOS
EOF
```

Alternatif: ubah logo kamu otomatis jadi ASCII lalu simpan ke `spider.txt`:

```bash
jp2a --width=40 --colors assets-final/logo.png > $NF/spider.txt
```

Konfigurasi neofetch:

```bash
cat > $NF/config.conf <<'EOF'
print_info() {
    info title
    info underline
    info "OS" distro
    info "Kernel" kernel
    info "Uptime" uptime
    info "Packages" packages
    info "Shell" shell
    info "DE" de
    info "CPU" cpu
    info "Memory" memory
}
image_backend="ascii"
image_source="/etc/skel/.config/neofetch/spider.txt"
ascii_colors=(1 4)
title_fqdn="off"
memory_percent="on"
EOF
```

### 8.3 Prompt bash dan alias (`.bash_aliases`)

Debian memuat `~/.bash_aliases` otomatis dari `.bashrc`, jadi kita tidak perlu menimpa `.bashrc`.

```bash
cat > $INC/etc/skel/.bash_aliases <<'EOF'
# Prompt SpiderOS: [user@host dir] dengan warna merah dan biru
PS1='\[\e[1;31m\][\u\[\e[1;34m\]@\h \[\e[1;37m\]\W\[\e[1;31m\]]\[\e[0m\]\$ '

# Alias
alias ll='ls -lah --color=auto'
alias update='sudo apt update && sudo apt full-upgrade'
alias sense='spider-sense'

# Tampilkan neofetch sekali di terminal interaktif pertama
if [ -t 1 ] && command -v neofetch >/dev/null 2>&1 && [ -z "$SPIDEROS_GREETED" ]; then
    export SPIDEROS_GREETED=1
    neofetch
fi
EOF
```

### 8.4 Warna terminal XFCE

```bash
mkdir -p $INC/etc/skel/.config/xfce4/terminal
cat > $INC/etc/skel/.config/xfce4/terminal/terminalrc <<'EOF'
[Configuration]
FontName=Fira Code 11
MiscAlwaysShowTabs=FALSE
MiscMenubarDefault=FALSE
ColorForeground=#e6e6e6
ColorBackground=#0b0c10
ColorCursor=#e23636
ColorPalette=#0b0c10;#e23636;#2ecc71;#f1c40f;#3a6df0;#c678dd;#00bcd4;#e6e6e6;#4b4f58;#ff6b6b;#69f0ae;#ffe082;#82a8ff;#e1a3ff;#80deea;#ffffff
EOF
```

### 8.5 Build dan uji

```bash
./scripts/build.sh v0.5
```

Cek di `[TEST-VM]`:

```bash
cat /etc/os-release | head -3
neofetch
echo $PS1
```

- [ ] `neofetch` menampilkan ASCII laba-laba dan nama "SpiderOS"
- [ ] Prompt berwarna merah-biru
- [ ] Warna terminal gelap dengan kursor merah

📸 `P08-neofetch-01.png`, `P08-terminal-01.png`.

### 8.6 Troubleshooting Phase 8

| Gejala | Solusi |
|---|---|
| `neofetch` tidak memakai ASCII kita | Cek `image_source` path, cek `ls -l /etc/skel/.config/neofetch/spider.txt` |
| Prompt tidak berubah | Pastikan `.bash_aliases` ada di `~` (`ls -a ~`), buka terminal baru |
| Nama OS tetap "Debian" | Cek hook 0300 di `build.log`, cek `cat /usr/lib/os-release` |
| Warna ASCII aneh | Sesuaikan `ascii_colors=(1 4)` (1 merah, 4 biru) |

### 8.7 Commit dan snapshot

```bash
git add -A
git commit -m "feat: branding terminal, neofetch, os-release"
```

Snapshot: `P8-terminal-branding`.

### 8.8 Checklist Phase 8

- [ ] `os-release`, banner, neofetch, prompt, warna terminal sesuai
- [ ] Sistem masih normal (apt jalan, tidak ada error aneh)
- [ ] Screenshot, commit, snapshot

✅ **Selesai kalau:** membuka terminal langsung terasa "SpiderOS".

---

## PHASE 9: Spider-Sense (Coding Bonus — Rust)

**Tujuan:** membuat tool CLI kecil bertema Spider-Man yang membaca `/proc`, ditulis dalam **Rust**, lalu memasukkannya ke ISO.
**Status:** bonus. Kalau Phase 1–8 belum aman, lewati dan kembali ke sini kalau ada waktu.

### 9.1 Spesifikasi

Spider-Sense menampilkan kondisi sistem dengan gaya Spider-Man:

| Info | Sumber di `/proc` | Konsep SO |
|---|---|---|
| Model CPU | `/proc/cpuinfo` | Informasi hardware |
| Penggunaan CPU (%) | `/proc/stat` (selisih dua pengambilan sampel) | Penjadwalan CPU, waktu user/system/idle |
| RAM total dan tersedia | `/proc/meminfo` | Manajemen memori (`MemAvailable`) |
| Jumlah proses | folder angka di `/proc` | Manajemen proses (satu folder per PID) |
| Uptime | `/proc/uptime` | Waktu sistem berjalan |
| Beban rata-rata | `/proc/loadavg` | Antrian proses (load) |
| Level "bahaya" | dihitung dari CPU dan RAM | Logika sederhana |

Level bahaya: di bawah 40% = "Tenang", 40–75% = "Waspada", di atas 75% = "BAHAYA! Spider-Sense berdenyut!".

### 9.2 Setup proyek Rust di build-VM

Pastikan Rust sudah terpasang (Phase 1.4):

```bash
cargo --version
rustc --version
```

Buat proyek baru:

```bash
cd ~/spideros/tools
cargo new spider-sense
cd spider-sense
```

### 9.3 Kode Spider-Sense (Rust)

Ganti isi `src/main.rs`:

```bash
cat > src/main.rs <<'EOF'
use std::fs;
use std::thread::sleep;
use std::time::Duration;

const RED: &str = "\x1b[1;31m";
const BLUE: &str = "\x1b[1;34m";
const WHITE: &str = "\x1b[1;37m";
const YELLOW: &str = "\x1b[1;33m";
const RESET: &str = "\x1b[0m";

fn cpu_times() -> Option<(u64, u64)> {
    let s = fs::read_to_string("/proc/stat").ok()?;
    let line = s.lines().next()?;
    let v: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|x| x.parse().ok())
        .collect();
    if v.len() < 5 {
        return None;
    }
    let idle = v[3] + v[4];
    let total: u64 = v.iter().take(8).sum();
    Some((idle, total))
}

fn cpu_usage() -> f64 {
    let (i1, t1) = match cpu_times() {
        Some(x) => x,
        None => return 0.0,
    };
    sleep(Duration::from_millis(400));
    let (i2, t2) = match cpu_times() {
        Some(x) => x,
        None => return 0.0,
    };
    let dt = (t2 - t1) as f64;
    let di = (i2 - i1) as f64;
    if dt <= 0.0 { 0.0 } else { 100.0 * (1.0 - di / dt) }
}

fn mem_kb(key: &str) -> u64 {
    fs::read_to_string("/proc/meminfo")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with(key))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|n| n.parse().ok())
        .unwrap_or(0)
}

fn cpu_model() -> String {
    fs::read_to_string("/proc/cpuinfo")
        .unwrap_or_default()
        .lines()
        .find(|l| l.starts_with("model name"))
        .and_then(|l| l.split(':').nth(1))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "tidak diketahui".to_string())
}

fn process_count() -> usize {
    fs::read_dir("/proc")
        .map(|d| {
            d.filter_map(|e| e.ok())
                .filter(|e| {
                    let n = e.file_name().to_string_lossy().to_string();
                    !n.is_empty() && n.chars().all(|c| c.is_ascii_digit())
                })
                .count()
        })
        .unwrap_or(0)
}

fn uptime_text() -> String {
    let sec: f64 = fs::read_to_string("/proc/uptime")
        .unwrap_or_default()
        .split_whitespace()
        .next()
        .and_then(|x| x.parse().ok())
        .unwrap_or(0.0);
    let s = sec as u64;
    let d = s / 86400;
    let h = (s % 86400) / 3600;
    let m = (s % 3600) / 60;
    if d > 0 {
        format!("{} hari {} jam {} menit", d, h, m)
    } else {
        format!("{} jam {} menit", h, m)
    }
}

fn load_avg() -> String {
    let s = fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let parts: Vec<&str> = s.split_whitespace().take(3).collect();
    parts.join(" ")
}

fn bar(pct: f64, width: usize) -> String {
    let filled = ((pct / 100.0 * width as f64).round() as usize).min(width);
    format!("{}{}", "#".repeat(filled), "-".repeat(width - filled))
}

fn main() {
    let cpu = cpu_usage();
    let total = mem_kb("MemTotal:");
    let avail = mem_kb("MemAvailable:");
    let mem_pct = if total > 0 {
        100.0 * (1.0 - avail as f64 / total as f64)
    } else {
        0.0
    };
    let procs = process_count();
    let load = load_avg();

    println!(
        "{RED}   \\  \\     /  /{RESET}"
    );
    println!(
        "{RED}    \\  \\   /  /{RESET}      {WHITE}S P I D E R - S E N S E{RESET}"
    );
    println!(
        "{RED} ____\\  \\_/  /____{RESET}   ------------------------"
    );
    println!(
        "{RED} \\     \\ (O) /    /{RESET}"
    );
    println!(
        "{RED}  `-.__/ / \\ \\__.-'{RESET}\n"
    );

    println!("{WHITE}Kernel  : {RESET}{}", std::env::consts::OS);
    println!("{WHITE}CPU     : {RESET}{}", cpu_model());
    println!("{WHITE}Uptime  : {RESET}{}", uptime_text());
    println!("{WHITE}Proses  : {RESET}{}", procs);
    println!("{WHITE}Load    : {RESET}{}\n", load);

    println!("{BLUE}CPU  [{}] {:.1}%{RESET}", bar(cpu, 20), cpu);
    println!(
        "{RED}RAM  [{}] {:.1}%  ({} / {} MB){RESET}\n",
        bar(mem_pct, 20),
        mem_pct,
        (total - avail) / 1024,
        total / 1024
    );

    let worst = cpu.max(mem_pct);
    if worst < 40.0 {
        println!("{BLUE}Status: Tenang. Lingkungan sekitar aman.{RESET}");
    } else if worst < 75.0 {
        println!("{YELLOW}Status: Waspada. Ada aktivitas mencurigakan.{RESET}");
    } else {
        println!("{RED}Status: BAHAYA! Spider-Sense berdenyut!{RESET}");
    }
}
EOF
```

### 9.4 Build dan tes di build-VM

```bash
cd ~/spideros/tools/spider-sense
cargo build --release
./target/release/spider-sense
```

Tes beban supaya status berubah:

```bash
# jalankan di terminal lain untuk membebani CPU
yes > /dev/null &
./target/release/spider-sense
kill %1
```

### 9.5 Masukkan ke ISO

```bash
cd ~/spideros
INC=config/includes.chroot_after_packages
mkdir -p $INC/usr/local/bin $INC/usr/share/applications

# Salin binary hasil build Rust
cp tools/spider-sense/target/release/spider-sense $INC/usr/local/bin/spider-sense
chmod +x $INC/usr/local/bin/spider-sense

cat > $INC/usr/share/applications/spider-sense.desktop <<'EOF'
[Desktop Entry]
Type=Application
Name=Spider-Sense
Comment=Monitor kondisi sistem
Exec=xfce4-terminal --hold -e spider-sense
Icon=utilities-system-monitor
Terminal=false
Categories=System;Monitor;
EOF
```

Catatan: binary yang ada di `includes.chroot_after_packages` sengaja ikut di-commit karena ukurannya kecil. Jangan commit folder `target/` milik Rust (sudah ada di `.gitignore`).

Alias `sense` sudah ada di `.bash_aliases` (Phase 8). Build dan uji:

```bash
./scripts/build.sh v0.6
```

Di `[TEST-VM]`: jalankan `spider-sense` dan `sense`, dan buka lewat menu aplikasi (kategori System). 📸 `P09-spidersense-01.png`.

### 9.6 Bahan penjelasan untuk dosen

Siapkan jawaban singkat untuk pertanyaan seperti ini:

| Pertanyaan | Jawaban inti |
|---|---|
| Apa itu `/proc`? | Filesystem virtual yang dibuat kernel di memori; file di dalamnya tidak ada di disk, isinya informasi kernel saat itu juga |
| Kenapa tiap proses punya folder angka? | Nama folder = PID, kernel menyediakan info per proses di sana |
| Kenapa CPU dihitung dari dua sampel? | `/proc/stat` berisi hitungan kumulatif sejak boot; penggunaan didapat dari selisih dalam rentang waktu |
| Kenapa pakai `MemAvailable`, bukan `MemFree`? | `MemFree` tidak menghitung cache yang bisa dilepas; `MemAvailable` perkiraan memori yang benar-benar bisa dipakai aplikasi |
| Apa itu load average? | Rata-rata jumlah proses yang siap jalan atau menunggu I/O dalam 1, 5, 15 menit |
| Kenapa pakai Rust? | Rust menghasilkan binary statis yang aman (memory safety), cepat (comparable dengan C++), dan modern — cocok untuk tool sistem |
| Apa itu ownership & borrowing di Rust? | Sistem unik Rust yang menjamin memory safety tanpa garbage collector — cocok untuk tool sistem yang harus ringan |

### 9.7 Checklist Phase 9

- [ ] Proyek Rust dibuat dengan `cargo new`
- [ ] Kode jalan di build-VM (`cargo build --release`)
- [ ] Binary masuk ISO, bisa dijalankan dari terminal dan menu
- [ ] Tim bisa menjelaskan cara kerja kode (tiap anggota minimal tahu alurnya)
- [ ] Commit dan snapshot (`P9-spidersense`)

✅ **Selesai kalau:** `spider-sense` jalan di ISO. Kalau tidak sempat, tulis di laporan sebagai pengembangan lanjutan dan jangan memaksakan.

---

## PHASE 10: Build ISO v1 dan Uji Menyeluruh di VM

**Tujuan:** menghasilkan ISO kandidat rilis (v1.0-rc1) dari build bersih dan mengujinya sistematis.
**Output:** `spideros-v1.0-rc1.iso`, checksum, tabel hasil uji, data performa untuk laporan.

### 10.1 Build bersih dari nol

```bash
cd ~/spideros
sudo lb clean --purge
./scripts/build.sh v1.0-rc1
```

Output: `dist/spideros-v1.0-rc1.iso` dan `dist/spideros-v1.0-rc1.iso.sha256`.

### 10.2 Uji fungsional lengkap

Uji di VM BIOS dan VM UEFI. Isi tabel (salin ke `docs/catatan.md`):

| No | Pengujian | BIOS | UEFI | Catatan |
|---|---|---|---|---|
| 1 | Boot menu bertema muncul |  |  |  |
| 2 | Plymouth tampil |  |  |  |
| 3 | Desktop auto-login |  |  |  |
| 4 | Wallpaper dan tema sesuai |  |  |  |
| 5 | Ikon Papirus tampil |  |  |  |
| 6 | Terminal: neofetch dan prompt |  |  |  |
| 7 | `cat /etc/os-release` menampilkan SpiderOS |  |  |  |
| 8 | Internet jalan (browser) |  |  |  |
| 9 | Firefox terbuka dan membuka situs |  |  |  |
| 10 | VLC terbuka |  |  |  |
| 11 | File manager, mousepad, screenshot |  |  |  |
| 12 | `spider-sense` jalan |  |  |  |
| 13 | `sudo ufw status` aktif |  |  |  |
| 14 | Reboot dan shutdown normal |  |  |  |

Isi dengan `Lulus` atau `Gagal` plus catatan.

### 10.3 Data performa untuk BAB IV (bukti "ringan dan efisien")

Jalankan di `[TEST-VM]` setelah desktop idle sekitar 1 menit:

```bash
free -h                              # RAM terpakai saat idle
ps aux | wc -l                       # jumlah proses
systemd-analyze                      # waktu boot
systemd-analyze blame | head -10     # layanan paling lama
df -h | grep -E "overlay|squashfs|/run/live"
```

Di build-VM:

```bash
ls -lh dist/spideros-v1.0-rc1.iso    # ukuran ISO
```

Catat semua angka. Untuk pembanding, kamu bisa mencatat RAM idle Debian XFCE default atau Ubuntu di VM yang sama.

### 10.4 Hentikan penambahan fitur

Setelah rc1 jadi, daftar bug masuk ke `docs/bug-log.md` (format di Phase 12). Fitur baru tidak boleh ditambah, kecuali perbaikan bug.

### 10.5 Commit, tag, snapshot

```bash
git add -A
git commit -m "release: v1.0-rc1"
git tag v1.0-rc1
```

Snapshot: `P10-rc1`.

### 10.6 Checklist Phase 10

- [ ] Build bersih sukses, checksum dibuat
- [ ] 14 pengujian dicoba di BIOS dan UEFI, hasil tercatat
- [ ] Data performa terkumpul
- [ ] Tag Git dan snapshot dibuat

✅ **Selesai kalau:** rc1 boot dan fungsional di VM, daftar bug (jika ada) tercatat.

---

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

| Perangkat (merek/model) | Mode boot | Secure Boot | Boot menu muncul | Desktop muncul | WiFi | Audio | Touchpad | Catatan |
|---|---|---|---|---|---|---|---|---|
|  | UEFI | Off |  |  |  |  |  |  |
|  | Legacy | n/a |  |  |  |  |  |  |

Targetkan minimal 2 perangkat berbeda.

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

- [ ] Checksum cocok, USB berhasil dibuat
- [ ] Boot sukses di minimal 2 perangkat, mode UEFI dan Legacy tercatat
- [ ] Boot menu per merek, matriks hasil uji, dan bukti boot sequence terisi
- [ ] Bug temuan masuk `docs/bug-log.md`

✅ **Selesai kalau:** ISO terbukti boot dari flashdisk di perangkat asli (rumusan masalah ke-4 terjawab).

---

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
| ISO terlalu besar | Hapus paket berat (lihat 5.2) |

### 12.3 Build v2

Satu perbaikan satu commit, lalu build:

```bash
git add -A && git commit -m "fix: B01 tambah firmware wifi"
sudo lb clean --purge
./scripts/build.sh v1.0
```

Ulangi uji: ringkasan 14 poin Phase 10 di VM, lalu uji ulang di perangkat bermasalah (Phase 11).

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

Buat `docs/RELEASE_NOTES.md` berisi: versi, tanggal, isi paket utama, fitur tema, keterbatasan (Secure Boot, tanpa installer, driver NVIDIA proprietary tidak ada), username dan password live.

Cadangan ISO final di minimal dua tempat (misalnya laptop dan cloud/flashdisk lain), plus satu flashdisk bootable siap presentasi.

### 12.6 Checklist Phase 12

- [ ] Semua bug prioritas 1 dan 2 selesai
- [ ] ISO final diuji ulang di VM dan perangkat asli
- [ ] Tag `v1.0`, checksum, release notes ada
- [ ] ISO dibackup, flashdisk presentasi siap

✅ **Selesai kalau:** ISO final stabil dan dibekukan.

---

## PHASE 13: Dokumentasi, Laporan, Demo, dan Presentasi

**Tujuan:** menyelesaikan laporan (BAB III sampai V), panduan pengguna, video demo, dan latihan presentasi.
**Output:** laporan final, panduan pengguna, video demo, ISO + flashdisk siap.

### 13.1 Peta bahan ke bab laporan

**BAB III Perancangan**

| Subbab | Isi yang ditulis | Bahan dari modul |
|---|---|---|
| 3.1 Analisis kebutuhan | Hardware dan software yang dipakai | Phase 1 (spesifikasi VM), tabel software |
| 3.2.1 Persiapan remastering | Instalasi Debian di VM, konfigurasi dasar, direktori kerja, tools | Phase 1 dan 2.2 |
| 3.2.2 Perancangan dan kustomisasi tampilan | Konsep visual, palet, daftar aset, rancangan tema | Phase 4, 6, 7, 8 (bagian rancangan) |
| 3.2.3 Integrasi aplikasi pendukung | Daftar paket dan alasan pemilihan, firewall, firmware, Spider-Sense (Rust) | Phase 5, 9 |
| 3.2.4 Proses remastering | Alur live-build (config, bootstrap, chroot, binary), struktur repo, hook | 2.1, 2.3, 6.1 |
| 3.3 Hasil yang diharapkan | Ringkasan target (ISO ringan, bertema, boot di BIOS dan UEFI) | Bagian 0.1 |

**BAB IV Hasil dan Pembahasan**

| Subbab | Isi yang ditulis | Bukti |
|---|---|---|
| 4.1.1 Lingkungan pengembangan | Spesifikasi VM, versi tools, struktur repo | Screenshot Phase 1, 2 |
| 4.1.2 Kustomisasi tampilan | Hasil tema: desktop, login, boot menu, Plymouth, terminal | Screenshot Phase 6, 7, 8 |
| 4.2 Proses pembuatan ISO | Langkah build, log, ukuran ISO, checksum | Screenshot Phase 2, 10; `build.log` |
| 4.3.1 Pembuatan media live boot | Rufus/Etcher, setting, hasil | Screenshot 11.3–11.4 |
| 4.3.2 Pengujian booting dan boot sequence | Tabel matriks hardware, uraian boot sequence, tombol boot menu | Foto 11.8 dan 11.9 |
| 4.4 Dokumentasi | Foto kerja tim, screenshot ringkas | Foto kelompok, screenshot |
| (Tambahan) Pembahasan performa | RAM idle, waktu boot, ukuran ISO | Data 10.3 |
| (Tambahan) Pembahasan keterbatasan | Secure Boot, tanpa installer, driver NVIDIA | Catatan Phase 11, 12 |

**BAB V Penutup**

| Subbab | Isi |
|---|---|
| 5.1 Kesimpulan | Jawab **satu per satu** keempat rumusan masalah di Bab I |
| 5.2 Saran | Tambah installer (Calamares), dukungan Secure Boot, driver NVIDIA, bahasa Indonesia, tema tambahan, aplikasi Welcome |

Hubungkan juga ke RUP di Bab II: Inception (Phase 1), Elaboration (Phase 2 dan 3), Construction (Phase 4–10), Transition (Phase 11–13).

### 13.2 Penjelasan untuk dosen: kenapa live-build, bukan Cubic

Tulis alasan singkat di Bab III:
- live-build adalah tool resmi proyek Debian Live, jalan native di Debian 13 (basis proyek).
- Seluruh konfigurasi berupa file (paket, hook, include) sehingga ISO dapat dibangun ulang secara konsisten dan terdokumentasi di Git.
- Cubic dirancang untuk lingkungan Ubuntu dan bersifat interaktif, kurang cocok untuk alur kerja yang konsisten pada Debian.
- Tujuan proposal (membuat distro turunan Debian dengan menambah/mengurangi paket, tema, dan konfigurasi) tetap tercapai.

### 13.3 Panduan pengguna (tujuan penelitian poin 3)

Buat `docs/PANDUAN_PENGGUNA.md`:
- Apa itu SpiderOS dan siapa target penggunanya
- Persyaratan (flashdisk 4 GB, laptop x86-64, RAM minimal 2 GB)
- Cara membuat flashdisk (Rufus/Etcher)
- Cara boot (tombol boot menu per merek, matikan Secure Boot)
- Login: user `spider`, password `live`
- Tur fitur: desktop, aplikasi, `spider-sense`, `neofetch`
- Tips: mengubah wallpaper, mengatur WiFi, shutdown
- Keterbatasan dan masalah umum
- Cara membangun ulang ISO (rujuk repo dan `scripts/build.sh`)

### 13.4 Video demo (cadangan kalau demo live bermasalah)

Skenario 3–5 menit, direkam dengan OBS Studio atau perekam layar bawaan:
- (0:00) Tampilkan flashdisk dan laptop/VM
- (0:20) Boot menu SpiderOS (sebutkan BIOS dan UEFI)
- (0:50) Plymouth lalu desktop
- (1:20) Tur tampilan: wallpaper, login, ikon
- (1:50) Terminal: `neofetch`, prompt, `spider-sense`
- (2:30) Aplikasi: Firefox, VLC
- (3:00) Tunjukkan `cat /proc/cmdline` dan `findmnt /` (bukti live boot)
- (3:30) Penutup

Simpan video di dua tempat.

### 13.5 Persiapan presentasi dan demo

Rencana demo bertingkat:
- **Rencana A:** boot flashdisk di laptop presentasi (sudah diuji di tempatnya).
- **Rencana B:** jalankan ISO di VM di laptop presentasi (VirtualBox/VMware).
- **Rencana C:** putar video demo.

Checklist H-1:
- [ ] Flashdisk bootable + 1 cadangan
- [ ] ISO dan VM test ada di laptop presentasi
- [ ] Tombol boot menu laptop yang dipakai sudah dicatat
- [ ] Video demo siap
- [ ] Slide: latar belakang, rumusan, alur live-build, tema, uji hardware, boot sequence, demo, kesimpulan
- [ ] Pembagian bicara jelas, semua anggota paham alur teknis
- [ ] Latihan sekali penuh dengan timer

### 13.6 Pertanyaan yang mungkin ditanyakan dosen

| Pertanyaan | Arah jawaban |
|---|---|
| Ini remastering atau membuat distro baru? | Distro turunan Debian: paket, tema, dan konfigurasi diubah dari Debian 13, kernel dan sistem inti tetap dari Debian |
| Apa beda live-build dan Cubic? | Lihat 13.2 |
| Bagaimana ISO bisa boot di BIOS dan UEFI? | ISO hybrid, ada MBR/ISOLINUX untuk BIOS dan partisi EFI untuk UEFI (bukti `xorriso -report_el_torito`) |
| Apa itu squashfs dan overlay? | Squashfs = filesystem terkompresi read-only berisi sistem; overlay = lapisan tulis di RAM di atasnya sehingga sistem live bisa dipakai tanpa menulis ke USB |
| Perubahan hilang setelah reboot, kenapa? | Sistem live tidak persisten secara default; persistensi bisa ditambahkan (Lampiran E) |
| Kenapa Secure Boot harus mati? | Bootloader tidak ditandatangani Microsoft/shim |
| Apa kontribusi koding kalian? | Spider-Sense ditulis dalam **Rust**, membaca `/proc` (lihat 9.6) — memanfaatkan memory safety dan performa Rust |
| Kenapa pakai Rust, bukan C++? | Rust menjamin memory safety tanpa garbage collector, cocok untuk tool sistem; modern, dan jadi nilai tambah di laporan |

### 13.7 Checklist Phase 13

- [ ] BAB III sampai V lengkap dan konsisten dengan ISO final
- [ ] Daftar pustaka dan atribusi aset (credits) lengkap
- [ ] Panduan pengguna, video demo, release notes siap
- [ ] Latihan presentasi selesai, rencana A, B, C siap

✅ **Selesai kalau:** laporan final, demo, dan materi presentasi siap dikumpulkan.

---

## LAMPIRAN

### Lampiran A: Cheat sheet perintah

```bash
# Konfigurasi dan build
sudo lb config                      # generate config dari auto/config
sudo lb build 2>&1 | tee build.log  # build penuh
sudo lb clean                       # hapus hasil build, simpan cache
sudo lb clean --binary              # hapus tahap binary saja
sudo lb clean --purge               # hapus semua (mulai dari nol)
./scripts/build.sh vX.Y             # build + rename + checksum

# Inspeksi ISO
file dist/*.iso
sha256sum dist/*.iso
xorriso -indev dist/spideros-vX.Y.iso -report_el_torito plain

# Inspeksi isi sistem yang dibangun (tanpa boot)
ls chroot/etc/skel/.config
sudo chroot chroot cat /usr/lib/os-release
sudo chroot chroot dpkg -l | wc -l

# Rust / Spider-Sense
cd tools/spider-sense
cargo build --release               # build release
./target/release/spider-sense       # jalankan
cargo clean                         # hapus target/

# Git
git status
git add -A && git commit -m "pesan"
git tag vX.Y
git log --oneline | head

# Cek ruang
df -h /
du -sh chroot cache binary 2>/dev/null
```

### Lampiran B: Troubleshooting besar (ringkasan)

| Area | Gejala | Cek pertama |
|---|---|---|
| Build | Gagal di tengah | 50 baris terakhir `build.log`, `grep -i "^E:\|error" build.log` |
| Build | Hook gagal | Cari nama hook di `build.log`, jalankan perintahnya manual di chroot |
| Includes | File tidak masuk | Nama folder `includes.chroot_after_packages`, cek `ls chroot/<path>` |
| Hook | Tidak jalan | Executable (`chmod +x`), nama berakhiran `.hook.chroot`, folder `config/hooks/live/` |
| Tema | Tidak berubah | `ls /usr/share/themes`, nama di XML, build hook 0100 |
| Boot | BIOS gagal, UEFI sukses (atau sebaliknya) | `xorriso -report_el_torito`, cara flash (DD mode), pengaturan BIOS |
| Boot | Layar hitam | `nomodeset`, firmware GPU, uji di perangkat lain |
| Plymouth | Tidak tampil | `splash` di cmdline, `plymouth-set-default-theme`, hook 0200 |
| Jaringan | Tidak ada WiFi | Firmware paket, chipset (`lspci -nnk`), `rfkill list` |
| Disk | Penuh | `df -h`, `sudo lb clean --purge`, perbesar VM disk |
| Waktu | Error "not valid yet" | `timedatectl set-ntp true` |
| Rust | `cargo: command not found` | `sudo apt install cargo rustc` |
| Rust | Build gagal | Baca error `cargo build`, biasanya typo atau import kurang |

### Lampiran C: Struktur akhir repo

```
spideros/
 ├── auto/
 │   ├── build
 │   ├── clean
 │   └── config
 ├── config/
 │   ├── package-lists/spideros.list.chroot
 │   ├── hooks/live/
 │   │   ├── 0050-system-config.hook.chroot
 │   │   ├── 0100-theme.hook.chroot
 │   │   ├── 0200-plymouth.hook.chroot
 │   │   └── 0300-branding.hook.chroot
 │   ├── bootloaders/                       # template boot menu yang diedit
 │   └── includes.chroot_after_packages/
 │       ├── etc/
 │       │   ├── lightdm/lightdm-gtk-greeter.conf
 │       │   └── skel/
 │       │       ├── .bash_aliases
 │       │       └── .config/
 │       │           ├── autostart/spideros-wallpaper.desktop
 │       │           ├── neofetch/{config.conf,spider.txt}
 │       │           └── xfce4/
 │       │               ├── terminal/terminalrc
 │       │               └── xfconf/xfce-perchannel-xml/{xsettings.xml,xfwm4.xml}
 │       └── usr/
 │           ├── local/bin/{spideros-wallpaper,spider-sense}
 │           ├── share/applications/spider-sense.desktop
 │           ├── share/backgrounds/spiderman/{wallpaper.png,login.png}
 │           ├── share/pixmaps/spideros-logo.png
 │           └── share/plymouth/themes/spiderman/{spiderman.plymouth,spiderman.script,logo.png}
 ├── assets-src/ , assets-final/
 ├── tools/spider-sense/                    # proyek Rust (Cargo.toml, src/main.rs)
 ├── scripts/build.sh
 ├── dist/                                  # ISO hasil (tidak di-commit)
 └── docs/
     ├── catatan.md, bug-log.md, desain.md, credits.md
     ├── RELEASE_NOTES.md, PANDUAN_PENGGUNA.md
     └── screenshots/
```

### Lampiran D: Checklist master

**Persiapan**
- [ ] Phase 1: VM, tools, repo, dokumentasi, peran

**Fondasi**
- [ ] Phase 2: ISO baseline terbangun
- [ ] Phase 3: Baseline boot di VM BIOS dan UEFI

**Pembangunan**
- [ ] Phase 4: Aset desain lengkap
- [ ] Phase 5: Paket dan konfigurasi sistem
- [ ] Phase 6: Tema desktop
- [ ] Phase 7: Boot menu dan Plymouth
- [ ] Phase 8: Branding terminal
- [ ] Phase 9: Spider-Sense (Rust, bonus)
- [ ] Phase 10: ISO v1 rc dan uji VM

**Penutupan**
- [ ] Phase 11: Uji USB dan hardware asli
- [ ] Phase 12: Bug fix dan freeze
- [ ] Phase 13: Laporan, panduan, demo, presentasi

### Lampiran E: Opsional dan lanjutan

1. **Mirror Debian yang lebih cepat.** Jika unduhan lambat, tambahkan di `auto/config`:
   ```
   --mirror-bootstrap "http://deb.debian.org/debian/" \
   --mirror-chroot "http://deb.debian.org/debian/" \
   --mirror-binary "http://deb.debian.org/debian/" \
   ```
2. **Bahasa Indonesia.** Ganti `locales=en_US.UTF-8` menjadi `locales=id_ID.UTF-8` dan tambah `locales-all` di daftar paket.
3. **Installer ke disk (Calamares).** Di luar batasan masalah proposal, tapi bisa jadi saran pengembangan.
4. **Persistence (data tetap setelah reboot).** Tambah `persistence` di `--bootappend-live`, siapkan partisi di flashdisk berlabel `persistence` dengan isi file `persistence.conf` (`/ union`).
5. **Memperkecil ISO.** Hapus aplikasi berat, gunakan JPG untuk wallpaper, hindari paket dokumentasi besar.

### Lampiran F: Referensi

- Debian Live Manual (live-build): https://live-team.pages.debian.net/live-manual/
- Debian Wiki, DebianLive: https://wiki.debian.org/DebianLive
- Rust Book (untuk Spider-Sense): https://doc.rust-lang.org/book/
- Dokumentasi Plymouth: lihat `/usr/share/plymouth/themes/` di sistem Debian
- Silberschatz et al. (2018), *Operating System Concepts*
- Harjono (2016) tentang remastering Debian
- IBM, penjelasan Linux

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