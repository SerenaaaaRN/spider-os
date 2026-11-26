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
| 3.2.5 Perancangan perangkat lunak Spider-Sense | Arsitektur `main.rs`, parser `/proc`, sampler CPU, renderer ANSI, unit test, hook `0400` | 9.1–9.6 |
| 3.3 Hasil yang diharapkan | Ringkasan target (ISO ringan, bertema, boot di BIOS dan UEFI) | Bagian 0.1 |

**BAB IV Hasil dan Pembahasan**

| Subbab | Isi yang ditulis | Bukti |
|---|---|---|
| 4.1.1 Lingkungan pengembangan | Spesifikasi VM, versi tools, struktur repo | Screenshot Phase 1, 2 |
| 4.1.2 Kustomisasi tampilan | Hasil tema: desktop, login, boot menu, Plymouth, terminal | Screenshot Phase 6, 7, 8 |
| 4.1.3 Implementasi dan pengujian Spider-Sense | Cuplikan kode parser dan renderer, hasil `cargo test`, verifikasi binary di ISO | Screenshot Phase 9, 10; `build.log` |
| 4.2 Proses pembuatan ISO | Langkah build, log, ukuran ISO, checksum | Screenshot Phase 2, 10; `build.log` |
| 4.3.1 Pembuatan media live boot | Rufus/Etcher, setting, hasil | Screenshot 11.3–11.4 |
| 4.3.2 Pengujian booting dan boot sequence | Tabel matriks hardware, uraian boot sequence, tombol boot menu | Foto 11.8 dan 11.9 |
| 4.4 Dokumentasi | Foto kerja tim, screenshot ringkas | Foto kelompok, screenshot |
| (Tambahan) Pembahasan performa | RAM idle, waktu boot, ukuran ISO | Data 10.4 |
| (Tambahan) Pembahasan keterbatasan | Secure Boot, tanpa installer, driver NVIDIA | Catatan Phase 11, 12 |

**BAB V Penutup**

| Subbab | Isi |
|---|---|
| 5.1 Kesimpulan | Jawab **satu per satu** keempat rumusan masalah di Bab I |
| 5.2 Saran | Tambah installer (Calamares), dukungan Secure Boot, driver NVIDIA, bahasa Indonesia, tema tambahan, aplikasi Welcome |

Hubungkan juga ke RUP di Bab II: Inception (Phase 1), Elaboration (Phase 2 dan 3), Construction (Phase 4–10), Transition (Phase 11–13).

### 13.2 Penjelasan untuk dosen: mengapa live-build

Tulis alasan singkat di Bab III:
- live-build adalah tool resmi proyek Debian Live dan berjalan native di Debian 13 (basis proyek).
- Seluruh konfigurasi berupa file (paket, hook, include) sehingga ISO dapat dibangun ulang secara konsisten dan terdokumentasi di Git.
- Proses build dapat diotomasi, termasuk kompilasi komponen Rust di dalam chroot melalui hook.
- Tujuan proposal (membuat distro turunan Debian dengan menambah/mengurangi paket, tema, dan konfigurasi) tetap tercapai.

### 13.3 Panduan pengguna (tujuan penelitian poin 3)

Buat `docs/panduan_pengguna.md`:
- Apa itu SpiderOS dan siapa target penggunanya
- Persyaratan (flashdisk 4 GB, laptop x86-64, RAM minimal 2 GB)
- Cara membuat flashdisk (Rufus/Etcher)
- Cara boot (tombol boot menu per merek, matikan Secure Boot)
- Login: user `spider`, password `live`
- Tur fitur: desktop, aplikasi, `spider-sense`, `neofetch`
- Aplikasi "SpiderOS Comics": cara menambahkan komik ke folder `/usr/share/spideros/comics/` (cover, bookmark, posisi halaman terakhir)
- Game bawaan: Mines, Sudoku, Bastet, NetHack
- Cara memakai Spider-Sense: `spider-sense`, `spider-sense --once`, `spider-sense --watch`, `spider-sense --help`
- Tips: mengubah wallpaper, mengatur WiFi, shutdown
- Keterbatasan dan masalah umum
- Cara membangun ulang ISO (rujuk repo dan `scripts/build.sh`)

### 13.4 Video demo (cadangan kalau demo live bermasalah)

Skenario 3–5 menit, direkam dengan OBS Studio atau perekam layar bawaan:
- (0:00) Tampilkan flashdisk dan laptop/VM
- (0:20) Boot menu SpiderOS (sebutkan BIOS dan UEFI)
- (0:50) Plymouth lalu desktop
- (1:20) Tur tampilan: wallpaper, login, ikon
- (1:50) Terminal: `neofetch`, prompt, `spider-sense --once`, `spider-sense --watch`
- (2:30) Aplikasi: Firefox, VLC
- (2:40) Aplikasi "SpiderOS Comics": tambahkan satu CBZ uji, tampilkan cover dan bookmark
- (3:00) Game bawaan: GNOME Mines atau `bastet` (~20 detik)
- (3:20) Tunjukkan `cat /proc/cmdline` dan `findmnt /` (bukti live boot)
- (3:50) Tunjukkan source Rust dan hook `0400-spider-sense-build.hook.chroot` di repositori
- (4:30) Penutup

Simpan video di dua tempat.

### 13.5 Persiapan presentasi dan demo

Rencana demo bertingkat:
- **Rencana A:** boot flashdisk di laptop presentasi (sudah diuji di tempatnya).
- **Rencana B:** jalankan ISO di VM di laptop presentasi (VirtualBox/VMware).
- **Rencana C:** putar video demo.

Checklist H-1:
- [ ] Flashdisk bootable + 1 cadangan
- [ ] ISO dan VM test ada di laptop presentasi
- [x] Tombol boot menu laptop yang dipakai sudah dicatat (ASUS: Esc/F2; Lenovo IdeaPad 82KC: Novo → Boot Menu)
- [ ] Video demo siap
- [ ] Slide: latar belakang, rumusan, alur live-build, tema, uji hardware, boot sequence, demo, kesimpulan
- [ ] Pembagian bicara jelas, semua anggota paham alur teknis
- [ ] Latihan sekali penuh dengan timer

### 13.6 Pertanyaan yang mungkin diajukan dosen

| Pertanyaan | Arah jawaban |
|---|---|
| Ini remastering atau membuat distro baru? | Distro turunan Debian: paket, tema, dan konfigurasi diubah dari Debian 13, kernel dan sistem inti tetap dari Debian |
| Mengapa memilih live-build? | Tool resmi Debian Live; konfigurasi berbasis file, reproducible, dan mudah diotomasi (lihat 13.2) |
| Bagaimana ISO bisa boot di BIOS dan UEFI? | ISO hybrid, ada MBR/ISOLINUX untuk BIOS dan partisi EFI untuk UEFI (bukti `xorriso -report_el_torito`) |
| Apa itu squashfs dan overlay? | Squashfs = filesystem terkompresi read-only berisi sistem; overlay = lapisan tulis di RAM di atasnya sehingga sistem live bisa dipakai tanpa menulis ke USB |
| Perubahan hilang setelah reboot, kenapa? | Sistem live tidak persisten secara default; persistensi bisa ditambahkan (Lampiran E) |
| Kenapa Secure Boot harus mati? | Bootloader tidak ditandatangani Microsoft/shim |
| Apa kontribusi koding tim? | Spider-Sense ditulis dalam **Rust**, membaca dan mem-parsing `/proc` secara manual, lalu merender ANSI (lihat 9.8) |
| Kenapa memakai Rust? | Rust menjamin memory safety tanpa garbage collector, menghasilkan binary native tanpa runtime bahasa, dan cocok untuk tool sistem |
| Di mana kode dikompilasi? | Di dalam chroot saat build ISO melalui hook `0400-spider-sense-build.hook.chroot`, bukan binary yang dibangun di host |
| Dari mana isi komik dan game-nya? | ISO menyediakan aplikasinya saja (mcomix dari repo Debian); pengguna mengisi komiknya sendiri setelah install. Game dan aplikasi lain di-install dari repo resmi Debian — tidak ada aset berhak cipta pihak ketiga di ISO |

### 13.6 Target pengguna, akun, dan pengamanan

Target pengguna SpiderOS adalah **penggemar Spider-Man / pengguna muda**: pengguna laptop pribadi yang menginginkan distro live dengan identitas visual kuat, ringan, dan langsung pakai. Kebutuhan ini diterjemahkan menjadi wallpaper/ikon bertema, Plymouth, aplikasi hiburan (komik, musik, video, game), DPI 120, dan auto-login.

Pemisahan akun dipakai sebagai pengamanan:

| Akun | Kata sandi | Grup utama | Hak akses |
|---|---|---|---|
| `spider` | `live` | grup perangkat dari `live-config` | User harian, auto-login, **tanpa sudo** |
| `admin` | `admin123` | `sudo`, `audio`, `video`, `plugdev`, `bluetooth` | Administrator, satu-satunya yang bisa `sudo` |

Hook `0060-users.hook.chroot` membuat `admin`, menyalin `/etc/skel`, mengunci root, dan memasang `/etc/sudoers.d/99-spideros-no-sudo` (mode `440`) yang melarang `spider` memakai `sudo`. LightDM dikonfigurasi lewat `lightdm.conf.d/50-spideros.conf` agar auto-login sebagai `spider`. Home tiap akun berizin `700`. Firewall `ufw` aktif dengan kebijakan tolak koneksi masuk dan izinkan koneksi keluar.

Verifikasi di ISO hasil build v3: `id spider`, `id admin`, `sudo -l -U spider` (harus ditolak), `sudo -l -U admin` (diizinkan), `sudo ufw status` (aktif).

### 13.7 Checklist Phase 13

- [ ] BAB III sampai V lengkap dan konsisten dengan ISO final
- [x] Source Rust, hook `0400`, hasil `cargo test`, dan verifikasi binary terdokumentasi
- [ ] Panduan pengguna dan release notes siap; video demo masih tertunda
- [ ] Latihan presentasi selesai, rencana A, B, C siap
- [x] Target pengguna, akun `spider`/`admin`, kata sandi, dan hak akses terdokumentasi
- [ ] Pemisahan hak (`sudo -l`) dan `ufw status` diverifikasi di ISO hasil build

✅ **Selesai kalau:** laporan final, demo, dan materi presentasi siap dikumpulkan.

---

