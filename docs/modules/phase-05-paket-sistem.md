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
polkitd
lxpolkit
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
- Meta-package `firmware-linux` di Trixie sudah mencakup banyak driver yang sebelumnya harus dipasang manual. Ini membuat ISO lebih kompatibel *out-of-the-box*.
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

### 5.5 Validasi tanpa build

Build ISO ditunda sampai Phase 10 (satu build final). Yang dicek sekarang tanpa `lb build`:

```bash
cd ~/spideros
apt-cache policy firefox-esr vlc ufw firmware-linux-nonfree
bash -n config/hooks/live/0050-system-config.hook.chroot
```

Daftar di bawah diverifikasi di `[TEST-VM]` setelah build final:

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

- [ ] Paket lengkap terpasang; diverifikasi pada ISO final di Phase 10
- [ ] Firefox, VLC, file manager, terminal jalan
- [ ] `ufw` aktif
- [ ] Ukuran ISO dicatat (target di bawah sekitar 2 GB)
- [ ] Commit dan snapshot dibuat

✅ **Selesai kalau:** sistem bisa dipakai harian dasar (browsing, media, file).

---

