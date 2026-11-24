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

Sebelum build, pastikan tidak ada sisa referensi Debian 12 dan seluruh konfigurasi memakai trixie:

```bash
grep -RniE "debian[ -]12" auto config scripts tools docs || echo "OK: tidak ada referensi Debian 12"
grep -n -- "--distribution" auto/config
# Output yang diharapkan: --distribution trixie
```

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

- [x] `auto/config`, paket list, `lb config` sukses
- [x] `auto/config` memakai `--distribution trixie` dan tidak ada referensi Debian 12
- [x] `sudo lb build` selesai tanpa error fatal
- [x] ISO ada, `file` menunjukkan hybrid, El Torito menunjukkan BIOS dan UEFI
- [x] `build.log` tersimpan, screenshot diambil
- [x] Commit Git dan snapshot dibuat
- [x] ISO sudah dipindah ke Windows

✅ **Selesai kalau:** ISO baseline ada dan sudah ada di Windows siap diuji.

---

