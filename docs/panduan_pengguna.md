# Panduan Pengguna SpiderOS

SpiderOS adalah distro live x86-64 turunan Debian 13 (trixie) dengan desktop XFCE bertema Spider-Man. Sistem berjalan langsung dari flashdisk, tanpa instalasi ke hard disk. Semua perubahan hilang setelah komputer dimatikan.

## 1. Persyaratan

- Laptop atau PC x86-64, RAM minimal 2 GB (4 GB lebih nyaman)
- Flashdisk minimal 4 GB (8 GB disarankan)
- Windows untuk membuat media boot, atau VM untuk mencoba ISO lebih dulu
- Secure Boot dimatikan sementara saat uji

File ISO: `dist/spideros-v1.iso`.

## 2. Membuat flashdisk bootable

Dengan Rufus:

1. Colok flashdisk, buka Rufus.
2. Pilih flashdisk yang benar di **Device**.
3. **Boot selection** → pilih `spideros-v1.iso`.
4. Skema partisi: **GPT + UEFI** untuk laptop UEFI, **MBR + BIOS** untuk laptop lama. Karena ISO bersifat hybrid, keduanya biasanya tetap bisa dipakai.
5. Klik **START**. Bila ditanya mode, pilih **DD Image mode**.
6. Tunggu sampai berstatus **READY**.

Alternatif dengan Balena Etcher: **Flash from file** → pilih ISO → **Select target** → **Flash!**

Semua isi flashdisk akan terhapus. Backup data penting lebih dulu.

## 3. Cara boot

1. Matikan laptop, colok flashdisk.
2. Nyalakan, tekan berulang tombol boot menu sesuai merek:

| Merek | Tombol |
|---|---|
| Acer | F12 (kadang aktifkan dulu di BIOS) |
| ASUS | Esc |
| Lenovo | F12 atau tombol Novo |
| HP | Esc lalu F9 |
| Dell | F12 |
| MSI | F11 |
| Gigabyte | F12 |

3. Pilih flashdisk. Bila ada dua entri (biasa dan berawalan `UEFI:`), coba keduanya.
4. Boot menampilkan splash Plymouth, lalu langsung masuk desktop. Tidak ada menu yang perlu dipilih.

Kalau flashdisk tidak muncul, matikan Secure Boot di BIOS (Security atau Boot → Secure Boot → Disabled). Aktifkan kembali setelah selesai. Nonaktifkan juga Fast Startup Windows atau pakai **Restart**, bukan Shut down.

## 4. Login

Sistem masuk otomatis sebagai user `spider`. Bila diminta kata sandi, gunakan `live`.

## 5. Tur fitur

- **Desktop:** wallpaper dan ikon bertema Spider-Man dengan palet maroon, rose, cream, sand, dan dusk.
- **Terminal:** menjalankan `neofetch` menampilkan satu frame Spider-Sense lalu prompt langsung kembali.
- **Aplikasi:** Firefox ESR, VLC, file manager, mousepad, screenshot, dan pengatur sistem.
- **Jaringan:** Network Manager di panel atas untuk WiFi.

## 6. Membaca komik

Aplikasi **SpiderOS Comics** (mcomix) tersedia di menu aplikasi. Mendukung file CBZ/CBR, tampilan cover/thumbnail, bookmark, dan mengingat posisi halaman terakhir.

ISO tidak menyediakan komik berhak cipta. Tambahkan sendiri file komik ke folder:

```
/usr/share/spideros/comics/
```

Buka aplikasi, lalu buka file dari folder tersebut.

## 7. Game bawaan

Tersedia dari repo Debian dan muncul di menu aplikasi:

- GNOME Mines
- GNOME Sudoku
- Bastet (game terminal)
- NetHack (game terminal)

Game terminal juga bisa dijalankan dari terminal dengan `bastet` atau `nethack`.

## 8. Media offline

- `mpv` — pemutar video
- `audacious` — pemutar audio

Keduanya bisa dibuka dari menu aplikasi atau lewat terminal.

## 9. Spider-Sense

Monitor sistem berbasis Rust yang membaca `/proc` langsung.

```bash
spider-sense            # mode live, perbarui tiap 1 detik; hentikan dengan Ctrl+C
spider-sense --once     # satu frame lalu selesai
spider-sense --watch    # sama dengan mode default
spider-sense --no-color # tanpa warna ANSI
spider-sense --help     # daftar opsi
spider-sense --version  # spider-sense 1.0.0 (Rust, std-only)
```

Status sistem ditampilkan sebagai Tenang (hijau), Waspada (kuning), atau BAHAYA (merah) berdasarkan pemakaian CPU dan RAM. Perintah `neofetch` memanggil binary yang sama lewat symlink, tetapi dalam mode snapshot.

## 10. Keterbatasan

- Sistem live tidak persisten: file yang dibuat hilang setelah reboot.
- Tidak ada opsi instalasi ke disk.
- Secure Boot harus dimatikan karena bootloader tidak ditandatangani.
- Driver NVIDIA proprietary tidak disertakan.
- Kecepatan boot sangat bergantung pada kualitas flashdisk.
- Perubahan konfigurasi WiFi dan kata sandi hanya berlaku selama sesi berjalan.

## 11. Membangun ulang ISO

Source ada di Git. Di VM Debian 13, clone repo lalu jalankan:

```bash
git clone https://github.com/SerenaaaaRN/spider-os.git
cd spider-os
./scripts/install-hooks.sh
./scripts/setup-bootloaders.sh
./scripts/build.sh v1
```

`scripts/build.sh` menyiapkan sumber Spider-Sense, menjalankan `lb clean` tanpa `--purge` agar cache tetap dipakai, lalu membangun ISO dan membuat file `.sha256`. Artefak masuk folder `dist/`.
