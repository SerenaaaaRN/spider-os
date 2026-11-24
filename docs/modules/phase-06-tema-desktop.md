## PHASE 6: Tema Desktop (Wallpaper, GTK, Ikon, Login Screen)

**Tujuan:** tampilan desktop benar-benar bertema Spider-Man.
**Output:** desktop dengan wallpaper, tema warna hangat (maroon/cream), ikon, dan login screen khusus.

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

> ✅ **Aset final:** `assets-final/wallpaper.png` dan `login.png` sudah 1920×1080. Tinggal disalin saat build.

```bash
INC=config/includes.chroot_after_packages
cp assets-final/wallpaper.png $INC/usr/share/backgrounds/spiderman/wallpaper.png
cp assets-final/login.png     $INC/usr/share/backgrounds/spiderman/login.png
cp assets-final/spideros-logo-small.png $INC/usr/share/pixmaps/spideros-logo.png
```

### 6.3 Tema GTK "SpiderOS" lewat hook

Cara: salin tema Arc-Dark lalu ganti warna aksen biru menjadi maroon SpiderOS (`#870000`).

```bash
cat > config/hooks/live/0100-theme.hook.chroot <<'EOF'
#!/bin/bash
set -e
SRC=/usr/share/themes/Arc-Dark
DST=/usr/share/themes/SpiderOS
if [ -d "$SRC" ]; then
  rm -rf "$DST"
  cp -r "$SRC" "$DST"
  # Ganti aksen biru Arc (#5294E2) menjadi maroon SpiderOS (#870000)
  find "$DST" -type f \( -name '*.css' -o -name 'gtkrc' -o -name '*.svg' -o -name 'themerc' -o -name '*.scss' \) \
    -exec sed -i -e 's/#5294E2/#870000/Ig' -e 's/rgba(82, *148, *226/rgba(135, 0, 0/Ig' {} +
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

### 6.8 Validasi tanpa build

Build ditunda sampai Phase 10. Cek dulu bahwa aset sudah masuk ke staging:

```bash
ls config/includes.chroot_after_packages/usr/share/backgrounds/spiderman
```

Daftar di bawah diverifikasi di `[TEST-VM]` setelah build final:
- [ ] Wallpaper Spider-Man tampil
- [ ] Tema jendela dan tombol berwarna maroon gelap
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

