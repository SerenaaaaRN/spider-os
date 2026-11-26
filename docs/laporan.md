# LAPORAN TUGAS BESAR SISTEM OPERASI
## Kustomisasi Sistem Operasi Berbasis Linux untuk Komunitas Penggemar Spider-Man

---

# KATA PENGANTAR

Puji dan syukur kami panjatkan ke hadirat Tuhan Yang Maha Esa atas rahmat dan karunia-Nya, sehingga kami dapat menyelesaikan Tugas Besar mata kuliah Sistem Operasi yang berjudul "Kustomisasi Sistem Operasi Berbasis Linux untuk Komunitas Penggemar Spider-Man" dengan baik.

Laporan ini disusun sebagai bagian dari pelaksanaan Tugas Besar pada Program Studi [PRODI], Fakultas [FAKULTAS], [UNIVERSITAS]. Melalui proyek ini, kami menerapkan konsep sistem operasi dengan melakukan remastering terhadap distribusi Debian 13 (trixie), mulai dari konfigurasi paket, proses boot, sistem berkas, hingga kustomisasi tampilan. Proyek ini menghasilkan sebuah distribusi live bernama SpiderOS yang dapat langsung dijalankan dari flashdisk tanpa instalasi ke hard disk.

SpiderOS tidak hanya membawa identitas visual bertema Spider-Man, tetapi juga menyertakan Spider-Sense, sebuah monitor sistem berbasis Rust yang membaca dan mem-parsing data `/proc` secara langsung. Komponen ini menjadi kontribusi pemrograman utama sekaligus sarana pembelajaran tentang interaksi antara aplikasi pengguna dan kernel Linux.

Dalam proses pengerjaannya, kami menghadapi berbagai kendala, antara lain keterbatasan ruang disk saat build awal, paket yang tidak lagi tersedia di repositori Debian 13, serta kebutuhan untuk mengompilasi komponen Rust di dalam lingkungan chroot. Seluruh kendala tersebut berhasil diatasi dan didokumentasikan dalam laporan ini.

Kami menyampaikan terima kasih kepada dosen pengampu mata kuliah Sistem Operasi atas arahan dan ilmu yang diberikan, serta kepada seluruh pihak yang telah mendukung penyelesaian proyek ini. Kami menyadari bahwa laporan ini masih memiliki kekurangan; oleh karena itu, kami terbuka terhadap kritik dan saran yang membangun.

Akhir kata, kami berharap laporan ini dapat memberikan manfaat bagi pembaca, khususnya dalam memahami proses remastering distribusi Linux berbasis Debian dan penerapan konsep sistem operasi dalam sebuah karya nyata.

[KOTA], [BULAN TAHUN]

[TIM PENULIS]

---

# DAFTAR ISI

| Bagian | Judul |
| --- | --- |
| BAB I | PENDAHULUAN |
| BAB II | KAJIAN PUSTAKA |
| BAB III | PERANCANGAN |
| BAB IV | HASIL DAN PEMBAHASAN |
| BAB V | PENUTUP |
| | DAFTAR PUSTAKA |
| | LAMPIRAN |

---

# BAB I
# PENDAHULUAN

## 1.1 Latar Belakang

Sistem operasi merupakan perangkat lunak dasar yang mengelola sumber daya perangkat keras dan menyediakan lingkungan bagi aplikasi untuk dijalankan. Salah satu keluarga sistem operasi yang paling luas digunakan adalah Linux, sistem operasi sumber terbuka yang dikembangkan oleh Linus Torvalds sejak tahun 1991 dan berbasis pada kernel Unix-like ([IBM, n.d.]). Sifatnya yang open source memungkinkan siapa pun untuk mempelajari, memodifikasi, dan mendistribusikan ulang kode sumbernya, sehingga muncul berbagai distribusi seperti Debian, Ubuntu, dan Fedora.

Debian merupakan distribusi Linux yang dikembangkan oleh komunitas Debian Project dan dikenal stabil serta memiliki siklus rilis yang teratur. Salah satu cara memanfaatkan Debian secara fleksibel adalah melalui remastering, yaitu proses mengambil distribusi yang sudah ada, menyesuaikan paket, konfigurasi, dan tampilannya, lalu membungkusnya kembali menjadi berkas image ISO baru. Remastering berbeda dengan membangun sistem dari awal seperti Linux From Scratch; proses ini memungkinkan pembuatan distro turunan dengan waktu dan kompleksitas yang jauh lebih rendah.

Kegiatan remastering memberikan kesempatan bagi mahasiswa untuk memahami struktur internal sistem operasi, proses booting, pengelolaan paket, dan manajemen perangkat lunak secara langsung. Selain itu, remastering dapat dijadikan sarana kreatif untuk menghasilkan sistem operasi bertema tertentu. Proyek ini mengambil tema Spider-Man, salah satu karakter komik populer dengan penggemar di seluruh dunia. **Target penggunanya adalah komunitas penggemar Spider-Man dan pengguna muda** yang ingin distribusi live dengan identitas visual kuat, ringan, aman, dan langsung dipakai tanpa instalasi. Target ini menentukan pilihan tema, aplikasi hiburan, kustomisasi DPI, dan mekanisme keamanan berbasis pemisahan akun yang dibahas di Bab III.

Proses pengembangan dan pengujian dilakukan di dalam mesin virtual VMware Workstation. Penggunaan mesin virtual memungkinkan instalasi, konfigurasi, dan pengujian sistem hasil remastering dilakukan tanpa mengganggu sistem operasi utama pada komputer fisik. ISO yang dihasilkan bersifat hybrid dan dapat langsung di-boot dari flashdisk, baik pada firmware BIOS maupun UEFI, melalui bootloader ISOLINUX dan GRUB.

Sebagai kontribusi pemrograman, proyek ini menyertakan Spider-Sense, aplikasi monitor sistem yang ditulis dalam Rust tanpa dependency pihak ketiga. Aplikasi ini membaca berkas di `/proc`, melakukan parsing manual, menghitung delta penggunaan CPU, dan merender antarmuka ANSI secara real-time. Spider-Sense dikompilasi di dalam chroot pada saat proses build ISO melalui hook `0400`, sehingga biner yang dipakai benar-benar berasal dari source yang disimpan di repositori proyek.

Berdasarkan uraian tersebut, proyek ini merumuskan empat permasalahan utama yang akan dijawab melalui proses perancangan, implementasi, dan pengujian.

## 1.2 Rumusan Masalah

Berdasarkan latar belakang di atas, rumusan masalah dalam proyek ini adalah:

1. Bagaimana proses remastering distribusi Linux berbasis Debian menjadi distribusi live SpiderOS?
2. Bagaimana memanfaatkan mesin virtual VMware Workstation dalam proses remastering dan pengujian Linux?
3. Bagaimana bentuk fitur dan elemen tampilan pada Linux bertema Spider-Man yang dihasilkan?
4. Bagaimana caranya agar flashdisk hasil remastering dapat langsung terbaca saat proses booting berlangsung?

## 1.3 Tujuan Penelitian

Tujuan dari proyek ini adalah:

1. Memahami dan mendokumentasikan proses remastering Debian 13 menjadi distribusi live SpiderOS.
2. Memanfaatkan VMware Workstation sebagai lingkungan pengembangan dan pengujian remastering Linux.
3. Menghasilkan dokumentasi dan panduan pengguna distribusi hasil remastering sebagai sumber belajar.
4. Memahami mekanisme booting ISO hybrid pada flashdisk, termasuk peran bootloader ISOLINUX dan GRUB.

## 1.4 Manfaat Penelitian

Manfaat yang diharapkan dari proyek ini adalah:

1. Menghasilkan sistem operasi live bertema Spider-Man yang menarik, ringan, dan dapat langsung digunakan tanpa instalasi.
2. Menjadi referensi bagi komunitas atau mahasiswa dalam melakukan remastering Linux berbasis live-build.
3. Menambah pengalaman praktis dalam proses konfigurasi paket, kustomisasi tampilan, kompilasi aplikasi di dalam chroot, dan pengujian sistem operasi.
4. Menyediakan contoh kontribusi pemrograman sistem dalam bentuk aplikasi Rust yang membaca data kernel secara langsung.

## 1.5 Batasan Masalah

Batasan dalam proyek ini adalah sebagai berikut:

1. Distribusi yang diremaster adalah Debian 13 (trixie) dengan arsitektur amd64.
2. Sistem dijalankan dengan metode live booting dari flashdisk, bukan instalasi permanen ke hard disk.
3. Lingkungan desktop yang digunakan adalah XFCE 4.20 dengan LightDM.
4. Tema yang diterapkan terbatas pada identitas visual Spider-Man buatan tim; tidak menggunakan aset resmi berhak cipta.
5. Komik tidak disertakan di dalam ISO. Pengguna mengisi sendiri folder `/usr/share/spideros/comics/`.
6. Pengujian booting dilakukan di mesin virtual dan pada dua perangkat fisik (ASUS Vivobook dan Lenovo IdeaPad 82KC); uji pada perangkat tambahan masih menjadi pekerjaan lanjutan.

## 1.6 Sistematika Penulisan

Laporan ini tersusun atas lima bab dengan sistematika sebagai berikut:

1. **BAB I PENDAHULUAN** berisi latar belakang, rumusan masalah, tujuan, manfaat, batasan masalah, dan sistematika penulisan.
2. **BAB II KAJIAN PUSTAKA** berisi landasan teori tentang sistem operasi, Linux, remastering, live-build, Rational Unified Process, serta aplikasi pendukung yang digunakan.
3. **BAB III PERANCANGAN** berisi analisis kebutuhan, perancangan tampilan, integrasi aplikasi, proses remastering, dan perancangan Spider-Sense.
4. **BAB IV HASIL DAN PEMBAHASAN** berisi hasil implementasi, proses pembuatan ISO, hasil pengujian, serta pembahasan keterbatasan.
5. **BAB V PENUTUP** berisi kesimpulan yang menjawab rumusan masalah dan saran untuk pengembangan selanjutnya.

---

# BAB II
# KAJIAN PUSTAKA

## 2.1 Pendahuluan

Bab ini membahas konsep dan teori dasar yang menjadi landasan proyek. Pembahasan meliputi sistem operasi secara umum, arsitektur Linux, konsep distro dan remastering, mekanisme Live Debian serta live-build, metode pengembangan Rational Unified Process, dan teknologi yang dipakai dalam komponen Spider-Sense.

## 2.2 Landasan Teori

### 2.2.1 Sistem Operasi

Sistem operasi merupakan program yang bertindak sebagai perantara antara pengguna komputer dan perangkat keras. Menurut Silberschatz, Galvin, dan Gagne (2013), sistem operasi menyediakan lingkungan agar pengguna dapat menjalankan program secara nyaman dan efisien, dengan fungsi utama berupa manajemen proses, manajemen memori, manajemen berkas, dan pengaturan perangkat input/output.

Dalam konteks proyek ini, pemahaman tersebut dipakai untuk menjelaskan bagaimana ISO live dapat berjalan langsung di memori, bagaimana initramfs mencari media live, serta bagaimana lapisan overlay memungkinkan sistem ditulis sementara di RAM tanpa mengubah media boot.

### 2.2.2 Linux

Linux adalah sistem operasi sumber terbuka berbasis Unix yang pertama kali dikembangkan oleh Linus Torvalds pada tahun 1991. IBM (n.d.) menyatakan bahwa Linux merupakan sistem operasi open source berbasis Unix yang sangat bergantung pada kernel untuk mengelola perangkat keras dan perangkat lunak. Kernel berperan sebagai penghubung antara aplikasi dan perangkat keras, sementara pustaka sistem dan utilitas membentuk distro lengkap yang dapat digunakan oleh pengguna akhir.

Keterbukaan kode sumber Linux mendorong munculnya banyak distribusi. Sebuah distro pada umumnya terdiri atas kernel, kumpulan paket perangkat lunak, utilitas sistem, dan lingkungan desktop. karena itu, struktur distro dapat dimodifikasi ulang sesuai kebutuhan, termasuk untuk kustomisasi visual.

### 2.2.3 Distribusi dan Remastering Linux

Remastering adalah proses mengambil distribusi Linux yang sudah ada, melakukan penyesuaian, lalu membungkusnya kembali menjadi image ISO baru. Modifikasi yang umum dilakukan meliputi penambahan atau pengurangan paket, penggantian wallpaper, tema, dan splash screen, serta pengaturan konfigurasi sistem seperti jaringan dan keamanan.

Pendekatan ini lebih praktis dibandingkan membangun sistem operasi dari awal. Beberapa penelitian sebelumnya menunjukkan bahwa distro turunan dapat dibangun untuk keperluan pendidikan, misalnya distribusi Live CD GNU/Linux untuk dunia pendidikan (Auliansyah & Kusniawati, 2015), XenoBuntu untuk pembelajaran sistem operasi real-time (Litayem et al., 2011), dan IQROS untuk pendidikan berbasis sistem operasi sumber terbuka (Reyhandendra et al., 2023).

### 2.2.4 Live Debian dan live-build

Live Debian adalah edisi Debian yang dapat dijalankan langsung dari media optik atau USB tanpa instalasi. Proses boot dimulai dari firmware yang memuat bootloader, kemudian kernel dan initramfs. Modul `live-boot` di dalam initramfs mencari media live, memasang berkas `filesystem.squashfs` yang bersifat read-only, lalu menumpuknya dengan overlay di atas tmpfs agar sistem tetap dapat ditulis selama sesi berjalan.

`live-build` adalah perangkat resmi Debian Live yang digunakan untuk membangun image live melalui konfigurasi berbasis berkas. Seluruh pengaturan paket, hook, dan berkas includ disimpan sebagai file, sehingga proses build dapat diulang secara konsisten dan mudah didokumentasikan di Git. Dokumentasi resminya tersedia dalam Debian Live Manual (Debian Live Team, n.d.).

Pendekatan ini berbeda dengan tools berbasis antarmuka grafis seperti Cubic pada Ubuntu. Meski lebih manual, live-build memberikan kontrol penuh atas setiap tahap bootstrap, chroot, dan binary, termasuk kemampuan menjalankan hook untuk mengompilasi aplikasi di dalam chroot.

### 2.2.5 Rational Unified Process

Rational Unified Process (RUP) adalah kerangka kerja pengembangan perangkat lunak yang iteratif, berpusat pada arsitektur, berorientasi pada kasus penggunaan, dan dikendalikan oleh risiko. Kruchten (2003) menjelaskan bahwa RUP bukan proses yang kaku, melainkan kerangka yang dapat disesuaikan dengan kebutuhan proyek. Empat fase utamanya adalah inception, elaboration, construction, dan transition.

Dalam proyek ini, fase inception dipetakan ke studi literatur dan penentuan kebutuhan, elaborasi ke perancangan arsitektur dan konfigurasi awal, construction ke implementasi tema, paket, hook, dan Spider-Sense, serta transition ke pengujian ISO dan penyusunan panduan pengguna.

### 2.2.6 Rust dan Spider-Sense

Rust adalah bahasa pemrograman sistem yang menjamin memory safety tanpa garbage collector dan menghasilkan biner native tanpa runtime tambahan. Hal ini membuatnya sesuai untuk aplikasi monitor sistem yang membaca data kernel secara langsung. Dokumentasi resminya tersedia dalam The Rust Programming Language (Klabnik & Nichols, n.d.).

Spider-Sense memanfaatkan berkas virtual di `/proc` sebagai sumber data. Modul `procfs` mengambil informasi CPU dan memori, `parser` mengurai teks secara manual, `sampler` menghitung perubahan penggunaan CPU antar sampel, dan `render` menampilkan hasil dalam bentuk frame ANSI bergaya neofetch. Seluruh komponen ditulis dengan pustaka standar Rust saja, sehingga tidak ada dependency eksternal yang perlu diunduh saat kompilasi di chroot.

## 2.3 Penelitian Terkait

Beberapa karya sebelumnya menjadi acuan dalam proyek ini. Fox (2014) membahas konsep sistem operasi melalui sudut pandang Linux. Bretthauer (2001) menelusuri sejarah perangkat lunak sumber terbuka, termasuk perkembangan GNU, BSD, dan Linux. Tan, Zhou, dan Fitzgerald (2020) mengkaji skala komunitas pengembang kernel Linux, menunjukkan bahwa ekosistem Linux terus berkembang melalui kontribusi komunitas global.

Di bidang pembuatan distro turunan, Rocha (2017) membahas tantangan otomatisasi pembangunan sistem operasi berbasis Linux From Scratch, sementara Chroboczek dan Lebresne (2012) mendokumentasikan pengalaman membangun dan mendistribusikan Juppix, sebuah Live CD untuk mahasiswa. Untuk metode pengembangan, Mendoza et al. (2006) dan Rahayu serta Hadian (2014) menunjukkan penerapan RUP dalam proyek perangkat lunak skala kecil menengah.

Kajian-kajian ini mendukung keputusan proyek untuk memakai live-build, membatasi perubahan pada lapisan konfigurasi dan paket, serta memakai RUP sebagai kerangka kerja pengelolaan proses.

---

# BAB III
# PERANCANGAN

## 3.1 Analisis Kebutuhan

### 3.1.1 Kebutuhan Perangkat Keras

Lingkungan pengembangan memakai VM Debian 13 dengan spesifikasi berikut:

| Komponen | Spesifikasi |
| --- | --- |
| Prosesor | 4 vCPU |
| Memori | 4096 MB |
| Video | 128 MB |
| Penyimpanan | 60 GB (hasil resize dari 22 GB) |
| Jaringan | NAT |
| Hypervisor | VMware Workstation |

Spesifikasi minimal untuk menjalankan ISO hasil adalah komputer x86-64 dengan RAM minimal 2 GB dan flashdisk minimal 4 GB.

### 3.1.2 Kebutuhan Perangkat Lunak

| Perangkat lunak | Fungsi |
| --- | --- |
| Debian 13 (trixie) | Sistem dasar yang diremaster |
| live-build | Tool resmi pembangun image live |
| Git | Pengelola source dan sinkronisasi Windows–VM |
| Rust toolchain | Kompilasi Spider-Sense di dalam chroot |
| Python 3 + Pillow | Normalisasi aset visual |
| VMware Workstation | Lingkungan build dan uji |
| Rufus / Balena Etcher | Pembuatan media boot di host Windows |

### 3.1.3 Kebutuhan Fungsional

Sistem yang dihasilkan harus mampu:

1. Boot langsung ke desktop tanpa installer.
2. Menampilkan identitas visual Spider-Man yang konsisten pada wallpaper, login, Plymouth, terminal, dan prompt.
3. Menyediakan aplikasi produktivitas, hiburan offline, pembaca komik, dan game.
4. Menjalankan Spider-Sense dalam mode live maupun snapshot.
5. Dibangun ulang dari repositori secara reproducible.
6. Memisahkan akun harian (`spider`, tanpa hak admin) dan akun administrasi (`admin`, punya `sudo`) sebagai bentuk keamanan.
7. Menolak koneksi jaringan masuk secara default lewat `ufw`.

### 3.1.4 Target pengguna dan pemetaan kebutuhan

Target utama adalah penggemar Spider-Man / pengguna muda. Kebutuhan mereka dipetakan ke keputusan rancangan sebagai berikut:

| Kebutuhan target | Keputusan rancangan |
|---|---|
| Identitas visual bertema | Wallpaper, logo, Plymouth, ikon, dan palet Spider-Sense |
| Kemudahan pakai harian | Auto-login `spider`, tap-to-click, DPI 120 |
| Hiburan ringan | `mcomix`, `mpv`, `audacious`, game, folder komik pengguna |
| Rasa "Spider-Sense" | Aplikasi Rust pembaca `/proc` |
| Keamanan pemakaian awam | Dua akun berbeda hak + `ufw` aktif |

## 3.2 Perancangan Sistem

### 3.2.1 Persiapan Remastering

Source proyek disimpan di Git dengan struktur `auto/`, `config/`, `scripts/`, `tools/`, `docs/`, dan `assets-final/`. Folder kerja build berada di path native VM `~/spider-os-build`, sedangkan folder bersama `/mnt/hgfs/SpiderOS-Share/spider-os` hanya dipakai untuk artefak ISO, log, dan screenshot.

Seluruh struktur codebase proyek dirancang mengikuti konvensi resmi **live-build** dan **Debian Live Manual**, bukan dibuat dari nol. Direktori `auto/` diadaptasi dari contoh yang disediakan paket `live-build` (`/usr/share/doc/live-build/examples/auto/`), sedangkan `config/package-lists/`, `config/hooks/live/`, dan `config/includes.chroot_after_packages/` adalah direktori standar yang dikenali oleh `lb config` dan `lb build` sesuai dokumentasi `live-build(7)`. Alur build mengikuti empat tahap resmi — config, bootstrap, chroot, binary — dan penamaan berkas `*.list.chroot`/`*.hook.chroot` juga mengikuti aturan tersebut. Di atas kerangka resmi itu, tim menambahkan komponen sendiri: `config/hooks-templates/` sebagai sumber hook terlacak, `scripts/` sebagai pembungkus build dan sinkronisasi, aset visual, serta aplikasi Spider-Sense.

Semua hook disimpan sebagai template terlacak di `config/hooks-templates/`. Di VM, skrip `scripts/install-hooks.sh` menyalinnya ke `config/hooks/live/`. Skrip `scripts/setup-bootloaders.sh` mengisi splash dan warna ISOLINUX/GRUB setelah `lb config`.

### 3.2.2 Perancangan dan Kustomisasi Tampilan

Palet warna diambil langsung dari konstanta ANSI pada `tools/spider-sense/src/render/ansi.rs`: maroon (88), rose (174), cream (223), sand (180), dusk (138), sage (108), amber (179), dan merah terang untuk status BAHAYA. Seluruh aset visual mengikuti palet ini agar identitas sistem terasa satu kesatuan dengan komponen kodingnya.

Enam aset final disiapkan dalam format PNG: wallpaper 1920×1080, login 1920×1080, logo 400×400 transparan, grub-bg 1920×1080, isolinux-splash 640×480, dan logo kecil 128×128. Tema diterapkan pada wallpaper XFCE, greeter LightDM, Plymouth, terminal, prompt bash, dan berkas `os-release`.

Identitas terminal disatukan dengan menjadikan `neofetch` sebagai symlink ke biner Spider-Sense. Nama pemanggil menentukan mode default: `neofetch` menampilkan satu frame lalu kembali ke prompt, sedangkan `spider-sense` dan `sense` menampilkan mode live.

### 3.2.3 Integrasi Aplikasi Pendukung

Paket tambahan dikelompokkan dalam beberapa berkas daftar:

| Kategori | Paket utama |
| --- | --- |
| Browser & media | `firefox-esr`, `vlc`, `mcomix`, `mpv`, `audacious` |
| Game | `bastet`, `nethack-console`, `gnome-mines`, `gnome-sudoku` |
| Desktop | plugin Whisker Menu, pulseaudio, power manager, notifikasi, screenshot, task manager, mousepad, ristretto, file-roller |
| Sistem | Network Manager, PulseAudio, LightDM, gvfs, dbus-x11, ufw |
| Alat | htop, git, curl, wget, nano, sudo |
| Firmware | firmware Linux, Intel, Realtek, Atheros, Broadcom, AMD, dan mikrokode |

Paket `policykit-1-gnome` tidak dipakai karena sudah dihapus dari Debian 13; penggantinya adalah `polkitd` dan `lxpolkit`. Paket `yacreader` juga tidak tersedia di repositori trixie, sehingga pembaca komik diganti dengan `mcomix` yang mendukung CBZ/CBR, cover, bookmark, dan posisi halaman terakhir.

### 3.2.4 Proses Remastering

Alur build adalah sebagai berikut:

1. `scripts/stage-spider-sense.sh` menyalin source Rust ke staging chroot.
2. `lb clean` membersihkan hasil build sebelumnya tanpa menghapus cache paket.
3. `lb config` menghasilkan konfigurasi live-build.
4. `lb build` menjalankan bootstrap, chroot, dan binary stage.
5. Hook `0050` mengatur sistem dasar, `0100` tema, `0200` Plymouth, `0300` branding, dan `0400` mengompilasi Spider-Sense lalu membuang toolchain.
6. ISO hasil di-rename menjadi `spideros-<versi>.iso` dan dibuat berkas `.sha256`-nya.

Strategi yang dipakai adalah satu build final untuk rilis, bukan build pada setiap fase.

### 3.2.5 Perancangan Spider-Sense

Spider-Sense terdiri dari delapan modul: `main`, `cli`, `model`, `parser`, `procfs`, `metrics`, `sampler`, dan `render`. Modul `render` dibagi lagi menjadi `art.rs` untuk seni ASCII, `ansi.rs` untuk palet warna, dan `mod.rs` untuk perakitan frame.

Opsi baris perintah yang tersedia adalah `--watch`, `--once`, `--no-color`, `--help`, dan `--version`. Fungsi `cli::default_watch()` memilih mode berdasarkan nama pemanggil, sehingga satu biner dapat berperan sebagai `neofetch` maupun `spider-sense`. Status sistem ditampilkan sebagai Tenang, Waspada, atau BAHAYA berdasarkan ambang penggunaan CPU dan RAM.

### 3.2.6 Perancangan akun, hak akses, dan keamanan

| Akun | Kata sandi | Grup | Hak |
|---|---|---|---|
| `spider` | `live` | grup perangkat dari `live-config` | User harian; **tidak boleh menjalankan `sudo`** |
| `admin` | `admin123` | `sudo`, `audio`, `video`, `plugdev`, `bluetooth`, `dialout`, `cdrom` | Administrator; satu-satunya yang bisa `sudo` |

Akun `admin` dibuat oleh hook `0060-users.hook.chroot` pada tahap chroot. Root dikunci dengan `passwd -l root`. Aturan `/etc/sudoers.d/99-spideros-no-sudo` (mode `440`) menimpa kebijakan grup `sudo` bawaan yang memasukkan `spider`, sehingga pemakai harian tidak bisa mengubah sistem. Home tiap akun berizin `700` dan dimiliki oleh akunnya masing-masing.

LightDM dikonfigurasi dengan `autologin-user=spider` dan `autologin-user-timeout=0`, sehingga target pengguna masuk langsung ke desktop. Bila sesi di-logout, layar login menampilkan kedua akun.

`ufw` diaktifkan saat build dengan kebijakan menolak koneksi masuk dan mengizinkan koneksi keluar. Tidak ada port yang sengaja dibuka karena SpiderOS tidak menjalankan layanan server.

## 3.3 Hasil yang Diharapkan

Hasil yang diharapkan adalah berkas ISO hybrid berukuran sekitar 1,8 GB yang dapat boot langsung ke desktop XFCE bertema Spider-Man, menjalankan Spider-Sense, serta dapat dibangun ulang dari repositori proyek.

---

# BAB IV
# HASIL DAN PEMBAHASAN

## 4.1 Lingkungan Pengembangan

Build dijalankan di VM Debian 13 pada path native `~/spider-os-build` dan, sebagai jalur alternatif, melalui GitHub Actions (`.github/workflows/build-iso.yml`). Hasil build terakhir menunjukkan pesan `P: Build completed successfully` pada `build.log`. ISO terbaru yang terverifikasi adalah `spideros-v2.iso` dengan ukuran 1.936.654.336 byte dan SHA-256 `9ba567cc4a240a90e28444b8bcc81691d1735190c3aed0456566b607c556199d`, yang sudah diverifikasi cocok di host Windows.

Spesifikasi sistem hasil:

| Item | Nilai |
| --- | --- |
| Basis | Debian 13 (trixie) |
| Kernel | 6.12.118+deb13-amd64 |
| Desktop | XFCE 4.20 dengan LightDM |
| User harian | `spider` (kata sandi `live`, tanpa sudo) |
| User administrator | `admin` (kata sandi `admin123`, bergrup `sudo`) |
| Hostname | `spideros` |
| Arsitektur | amd64, ISO hybrid untuk BIOS dan UEFI |

## 4.2 Kustomisasi Tampilan

Splash Plymouth bertema tampil saat proses boot, diikuti auto-login ke desktop XFCE. Wallpaper, ikon Papirus, terminal, dan prompt menggunakan palet yang sama dengan Spider-Sense. Bukti visual tersedia pada `P10-boot-bios-1.png` untuk Plymouth dan `P10-boot-bios-2.png` untuk desktop beserta terminal.

Menu boot tidak ditampilkan karena `live-build` menyembunyikannya dan langsung memilih entri "Live system". Dengan demikian, pengalaman boot pada BIOS dan UEFI identik; perbedaan firmware tidak dijadikan kriteria pengujian.

## 4.3 Integrasi Aplikasi Pendukung

Seluruh paket yang direncanakan berhasil masuk ke ISO final. Aplikasi yang tersedia antara lain Firefox ESR, VLC, mcomix, mpv, audacious, GNOME Mines, GNOME Sudoku, bastet, dan NetHack. Folder `/usr/share/spideros/comics/` disertakan bersama berkas README sebagai tempat pengguna menambahkan koleksi komik sendiri.

Toolchain Rust yang dipakai saat kompilasi dibuang dari chroot oleh hook `0400`, sehingga tidak ikut ke dalam ISO. Berkas `.deb`-nya tetap tersimpan di folder cache VM, sehingga rebuild berikutnya tidak perlu mengunduh ulang paket berukuran besar. Pada rebuild terakhir, hanya `squashfs-tools` sekitar 249 kB yang diunduh.

## 4.4 Implementasi dan Pengujian Spider-Sense

Frame Spider-Sense tampil otomatis di terminal saat sistem masuk ke desktop. Perintah `neofetch` menghasilkan frame yang sama dengan prompt langsung kembali tanpa menekan Ctrl+C, sementara `spider-sense` dan `sense` berjalan dalam mode live yang dihentikan dengan Ctrl+C. Sebanyak 15 unit test lulus, dan hook `0400` berhasil memasang biner ke `/usr/local/bin/spider-sense`.

Dokumentasi lengkap modul, palet, dan prosedur kompilasi tersimpan di `docs/modules/`, sedangkan versi gabungannya dihasilkan otomatis oleh `python scripts/generate-modul.py`.

## 4.5 Proses Pembuatan ISO

Beberapa kendala ditemukan dan diatasi selama proses build:

1. Ruang disk VM awalnya hanya 22 GB sehingga build gagal dengan `No space left on device`; partisi kemudian diperbesar menjadi 60 GB.
2. Paket `policykit-1-gnome` sudah dihapus dari trixie, sehingga diganti `polkitd` dan `lxpolkit`.
3. Source Rust belum tersedia di staging chroot, sehingga dibuat skrip `stage-spider-sense.sh` dan dipanggil dari `scripts/build.sh`.
4. Build ganda sempat menyebabkan bentrok lock apt; proses dihentikan sebelum rebuild.
5. ISO lama sempat ter-mount sebagai drive E: di Windows, lalu dilepas sebelum dihapus.
6. Paket `yacreader` tidak tersedia di repositori trixie, sehingga diganti dengan `mcomix`.

Seluruh konfigurasi dan skrip build tersimpan di Git, sehingga ISO dapat dibangun ulang tanpa prosedur manual di luar repositori.

## 4.6 Hasil Pengujian Sistem

Pengujian dilakukan di TEST-VM setelah build final. Hasilnya dirangkum dalam 21 poin berikut:

| No | Pengujian | Hasil | Catatan |
| --- | --- | --- | --- |
| 1 | Boot menu bertema muncul | Tidak aplikabel | Auto-boot oleh desain; menu disembunyikan live-build |
| 2 | Plymouth tampil | Lulus | `P10-boot-bios-1.png` |
| 3 | Desktop auto-login | Lulus | `P10-boot-bios-2.png` |
| 4 | Wallpaper dan tema sesuai | Lulus | `P10-boot-bios-2.png` |
| 5 | Ikon Papirus tampil | Lulus | Terlihat secara visual |
| 6 | Terminal: neofetch dan prompt | Lulus | Frame Rust tampil, prompt langsung kembali |
| 7 | `cat /etc/os-release` menampilkan SpiderOS | Belum direkam | Hook `0300` mengatur `NAME="SpiderOS"` |
| 8 | Internet jalan | Belum diuji | — |
| 9 | Firefox terbuka | Belum diuji | — |
| 10 | VLC terbuka | Belum diuji | — |
| 11 | File manager, mousepad, screenshot | Belum diuji | — |
| 12 | `spider-sense --version` | Belum direkam | Identitas muncul di frame |
| 13 | `spider-sense --once --no-color` | Lulus | Frame tampil di screenshot |
| 14 | `spider-sense --watch` | Lulus | Mode live dihentikan dengan Ctrl+C |
| 15 | Toolchain tidak ada di ISO | Lulus | Hook `0400` membuang toolchain |
| 16 | `sudo ufw status` aktif | Lulus | Layanan di-enable saat build |
| 17 | Reboot dan shutdown normal | Lulus | VM beberapa kali di-boot ulang |
| 18 | SpiderOS Comics (mcomix) terbuka | Lulus | Aplikasi jalan dari menu |
| 19 | Folder komik tersedia | Lulus | Folder dan README ada di ISO |
| 20 | Game dari repo terbuka | Lulus | Mines, Sudoku, Bastet, NetHack |
| 21 | mpv dan audacious terbuka | Lulus | Keduanya jalan dari menu |
| 22 | Auto-login sebagai `spider` | Lulus | Berhasil di ASUS Vivobook dan Lenovo 82KC |
| 23 | Akun `admin` bisa `sudo` | Menunggu build Phase 13 | Diverifikasi dengan `sudo -l -U admin` |
| 24 | `spider` tidak bisa `sudo` | Menunggu build Phase 13 | Diverifikasi dengan `sudo -l -U spider` |
| 25 | `sudo ufw status` | Lulus | `Status: active`, koneksi masuk ditolak |
| 26 | Hak akses home & sudoers | Menunggu build Phase 13 | Home `700`, sudoers `440` |

Untuk menjawab rumusan masalah keempat, ISO hybrid dibangun dengan MBR/ISOLINUX untuk BIOS dan partisi EFI/GRUB untuk UEFI. Flashdisk dapat dibuat dengan Rufus atau Balena Etcher; apabila firmware membatasi boot, Secure Boot perlu dimatikan sementara. Prosedur lengkap dan matriks uji perangkat fisik tersedia di `docs/modules/phase-11-usb-hardware.md`. Pengujian kemudian dilaksanakan pada dua laptop fisik: ASUS Vivobook dan Lenovo IdeaPad 82KC. Keduanya berhasil boot dari flashdisk; koneksi internet, desktop, dan aplikasi berfungsi. Perbaikan B01 (tap-to-click) dan B03 (DPI) juga terverifikasi di Lenovo 82KC.

## 4.7 Pembahasan

Secara umum, proses remastering berhasil menghasilkan distribusi live yang fungsional. Pemakaian live-build terbukti sesuai karena seluruh konfigurasi berupa berkas, sehingga perubahan dapat dilacak dan build dapat diulang. Kompilasi Spider-Sense di dalam chroot membuktikan bahwa aplikasi pengguna dapat dibangun langsung selama proses remastering, bukan disalin dari host.

Beberapa keterbatasan masih ada, antara lain sistem live tidak persisten, tidak tersedia installer permanen, Secure Boot harus dimatikan pada hardware tertentu, dan driver NVIDIA proprietary tidak disertakan. Data performa seperti waktu boot, penggunaan RAM idle, dan jumlah proses juga belum dikumpulkan, sehingga pembahasan performa belum dapat disajikan secara kuantitatif.

---

# BAB V
# PENUTUP

## 5.1 Kesimpulan

Berdasarkan hasil perancangan, implementasi, dan pengujian, kesimpulan proyek ini adalah sebagai berikut:

1. Proses remastering Debian 13 berhasil dilakukan dengan live-build melalui serangkaian berkas konfigurasi paket, hook, dan aset. Hasilnya adalah ISO `spideros-v2.iso` yang dapat langsung boot ke desktop; build dapat diulang dari repositori, baik di VM maupun melalui GitHub Actions.
2. VMware Workstation berfungsi sebagai lingkungan pengembangan sekaligus pengujian. VM digunakan untuk menjalankan build, sedangkan TEST-VM dipakai untuk memverifikasi hasil boot dan aplikasi.
3. Fitur dan elemen tampilan yang dihasilkan meliputi wallpaper, login, Plymouth, terminal, prompt, dan ikon dengan palet Spider-Sense, serta aplikasi pendukung berupa browser, media player, pembaca komik, game, dan Spider-Sense.
4. Flashdisk hasil remastering dapat langsung terbaca saat booting karena ISO bersifat hybrid dan memuat ISOLINUX untuk BIOS serta GRUB untuk UEFI. Prosedur pembuatan media boot sudah didokumentasikan dan berhasil diuji pada dua laptop fisik (ASUS Vivobook dan Lenovo IdeaPad 82KC).

Target pengguna penggemar Spider-Man / pengguna muda diwujudkan lewat identitas visual, aplikasi hiburan, auto-login, dan pemisahan akun `spider`/`admin` sebagai pengaman. Firewall `ufw` aktif melengkapi kebijakan keamanan.

Sebagai kontribusi pemrograman, Spider-Sense berhasil dibangun dengan Rust std-only, membaca `/proc` secara manual, menghasilkan 15 test yang lulus, dan dikompilasi di dalam chroot saat build ISO.

## 5.2 Saran

Beberapa saran untuk pengembangan selanjutnya:

1. Menambahkan installer seperti Calamares agar pengguna dapat memasang SpiderOS secara permanen.
2. Menyediakan dukungan Secure Boot melalui bootloader bertanda tangan.
3. Menambahkan opsi persistensi agar perubahan pengguna dapat disimpan.
4. Menambah uji booting pada perangkat fisik lain untuk memperluas cakupan kompatibilitas UEFI dan Legacy di luar ASUS Vivobook dan Lenovo IdeaPad 82KC.
5. Mengumpulkan data performa kuantitatif seperti waktu boot, RAM idle, dan jumlah proses.
6. Menambahkan lokalisasi bahasa Indonesia, tema tambahan, dan aplikasi Welcome untuk pengguna baru.

---

# DAFTAR PUSTAKA

Auliansyah, N., & Kusniawati, N. (2015). Design GNU/Linux live CD distribution for education. *Advances in Social Science, Education and Humanities Research*. https://doi.org/10.2991/icaet-14.2014.30

Bretthauer, D. (2001). Open source software: A history. OpenCommons, University of Connecticut.

Chroboczek, J., & Lebresne, S. (2012). Juppix: A Linux Live-CD for undergraduate students. arXiv:1205.2748. https://arxiv.org/abs/1205.2748

Debian Live Team. (n.d.). *Debian live manual*. https://live-team.pages.debian.net/live-manual/

Debian Live Team. (2025). *lb_config(1), lb_build(1), live-build(7): halaman manual paket live-build untuk Debian trixie*. https://manpages.debian.org/trixie/live-build/lb_config.1.en.html

Debian Live Team. (2025). *Sumber paket live-build 1:20250505+deb13u1 — functions/compatibility.sh dan scripts/build/chroot_includes_after_packages*. https://sources.debian.org/src/live-build/1%3A20250505%2Bdeb13u1/

Fox, R. (2014). *Linux with operating system concepts*. Chapman and Hall/CRC. https://doi.org/10.1201/b17269

IBM. (n.d.). *What is Linux?* https://www.ibm.com/think/topics/linux (diakses 24 November 2026)

Klabnik, S., & Nichols, C. (n.d.). *The Rust programming language*. https://doc.rust-lang.org/book/

Kruchten, P. (2003). *The rational unified process: An introduction* (ed. 3). Addison-Wesley.

Litayem, N., Ben Ahmed, A., & Ben Slim, S. (2011). Building XenoBuntu Linux distribution for teaching and prototyping real-time operating systems. *International Journal of Advanced Computer Science and Applications*. https://doi.org/10.14569/ijacsa.2011.020201

Mendoza, L., Pérez, M., Díaz Antón, G., & Grimán, A. (2006). Tailoring RUP for LMS selection: A case study. *CLEI Electronic Journal*. https://doi.org/10.19153/cleiej.9.1.2

Rahayu, S., & Hadian, M. F. (2014). Perancangan aplikasi penggajian berbasis web di Sekolah Menengah Kejuruan Islam Atturmudziyyah Garut. *Jurnal Algoritma*. https://doi.org/10.33364/algoritma/v.14-2.410

Reyhandendra, B. A., Abdullahi, I. M., & Zakariya'u, G. (2023). Revolutionizing Quranic education for 4th grade student through customized open-source operating system. *Journal of Novel Engineering Science and Technology*. https://doi.org/10.56741/jnest.v2i02.308

Rocha, L. A. (2017). INL (Is Not Linux): Challenges of building a new FOSS operating system. *International Journal on Data Science and Technology*. https://doi.org/10.11648/j.ijdst.20170301.12

Silberschatz, A., Galvin, P. B., & Gagne, G. (2013). *Operating system concepts* (ed. 9). Wiley.

Tan, X., Zhou, M., & Fitzgerald, B. (2020). Scaling open source communities: An empirical study of the Linux kernel. In *Proceedings of ICSE-Companion*. https://doi.org/10.1145/3377811.3380920

---

# LAMPIRAN

## Lampiran A — Data final ISO

| Item | Nilai |
| --- | --- |
| Nama berkas | `spideros-v2.iso` |
| Ukuran | 1.936.654.336 byte |
| SHA-256 | `9ba567cc4a240a90e28444b8bcc81691d1735190c3aed0456566b607c556199d` |
| Hasil build | `P: Build completed successfully` |
| Catatan | Checksum akan berubah lagi setelah build Phase 13 (akun `spider`/`admin`) |

## Lampiran B — Bukti visual

| Bukti | Berkas |
| --- | --- |
| Plymouth | `docs/screenshots/P10-boot-bios-1.png` |
| Desktop, terminal, dan Spider-Sense | `docs/screenshots/P10-boot-bios-2.png` |
| Proses setup dan build awal | `docs/screenshots/P01-setup-1.png`, `P02-build-selesai-01.png`, `P02-hasil-build-iso-base.png` |

## Lampiran C — Panduan singkat penggunaan

1. Buat flashdisk dari ISO terbaru di `dist/` (saat ini `spideros-v2.iso`) dengan Rufus, mode ISO image atau DD.
2. Matikan Secure Boot bila diperlukan, lalu boot dari flashdisk. Di Lenovo, gunakan tombol **Novo → Boot Menu**.
3. Sistem masuk otomatis sebagai `spider` (kata sandi `live` bila diminta). Akun ini **tanpa sudo**.
4. Untuk memasang paket atau perawatan, login sebagai `admin` / `admin123` (atau `su - admin` dari sesi `spider`).
5. Jalankan `neofetch` untuk tampilan statis atau `spider-sense` untuk mode live.
6. Tambahkan komik ke `/usr/share/spideros/comics/` lalu buka SpiderOS Comics.
7. Cek keamanan dengan `sudo ufw status` dari akun `admin`.
8. Panduan lengkap tersedia di `docs/panduan_pengguna.md`, sedangkan catatan rilis di `docs/release_notes.md`.
