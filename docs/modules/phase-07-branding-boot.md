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

> ⚠️ **TODO sebelum build:** `assets-final/logo.png` masih **dummy** (400×400 transparan). Ganti dulu — lihat `docs/desain.md` §4.

```bash
cd ~/spideros
INC=config/includes.chroot_after_packages
PLY=$INC/usr/share/plymouth/themes/spiderman
mkdir -p $PLY
# TODO: ganti dummy
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
Window.SetBackgroundTopColor(0.09, 0.06, 0.06);
Window.SetBackgroundBottomColor(0.29, 0.04, 0.04);

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

Angka `(0.09, 0.06, 0.06)` adalah `#17100F` (kanvas), sedangkan `(0.29, 0.04, 0.04)` adalah `#4A0B0B` (maroon dalam), sesuai palet di `docs/desain.md` §2.

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

> ⚠️ **TODO sebelum build:** `isolinux-splash.png` dan `grub-bg.png` masih **dummy**; ukuran/format harus sama seperti file aslinya. Lihat `docs/desain.md` §4.

```bash
# TODO: ganti dummy
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
menu color border   0 #ff870000 #00000000 none
menu color title    1;37;40 #ffd7af #00000000 none
menu color sel      7;37;40 #ffffffff #ff870000 all
menu color unsel    37;40 #ffd7af #00000000 none
```

Untuk GRUB, cari `menu_color` atau `color_normal` di `config/bootloaders/grub-*/`:

```
set menu_color_normal=white/black
set menu_color_highlight=white/red
```

`white/red` dipilih karena GRUB hanya mengenal nama warna, bukan `#HEX`. Kalau highlight terlalu terang, pakai `white/black` dengan splash maroon.

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

