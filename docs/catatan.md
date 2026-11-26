# Catatan Pengerjaan SpiderOS
Format tiap entri: tanggal, phase, apa yang dilakukan, command, hasil.

| Tanggal | Phase | Langkah | Command | Hasil |
| :-- | :-- | :-- | :-- | :-- |
| 2026-11-06 | Phase 1 | Setup lingkungan build dan repo Git awal | `git init`, `mkdir -p auto config/package-lists ...`, setup shared folder | Repo siap, tools terpasang, struktur folder terbentuk |
| 2026-11-14 | Phase 2.4 | Menambahkan kernel eksplisit ke daftar paket untuk mencegah error vmlinuz hilang | `cat > config/package-lists/spideros.list.chroot <<'EOF' linux-image-amd64 linux-headers-amd64 xorg lightdm ... EOF` | File `spideros.list.chroot` terupdate dengan dependensi kernel wajib |
| 2026-11-14 | Phase 2.5 | Build baseline pertama kali setelah konfigurasi ulang | `sudo lb clean --purge && sudo lb config && sudo lb build 2>&1 \| tee build.log` | Gagal: Error `No space left on device` karena disk VM hanya 22GB |
| 2026-11-14 | Troubleshooting | Hapus artefak build via recovery mode agar sistem bisa boot kembali | `rm -rf /home/serena/spider-os-build/{chroot,binary,cache} /var/cache/apt/*` | Sistem berhasil boot normal, siap untuk resize partisi |
| 2026-11-14 | Troubleshooting | Resize partisi root pasca extend disk VMware 22GB→60GB | `sudo swapoff -a`, GParted GUI (Delete sda2 → Resize sda1 → New Swap), `swapon -a` | Root partition berhasil diperbesar menjadi ~58GB, swap dikonfigurasi ulang |
| 2026-11-14 | Phase 2.6 | Build ulang ISO Baseline dengan storage yang sudah lega | `cd ~/spider-os-build && sudo lb clean --purge && sudo lb config && sudo lb build 2>&1 \| tee build.log` | **SUKSES**: `live-image-amd64.hybrid.iso` terbentuk (~700MB+), cache tersimpan aman |
| 2026-11-14 | Phase 2.7 | Verifikasi hasil build ISO baseline | `ls -lh *.iso`, `file live-image-amd64.hybrid.iso`, `sha256sum live-image-amd64.hybrid.iso` | ISO hybrid valid, ukuran sesuai, checksum tercatat |
| 2026-11-14 | Phase 2.10 | Finalisasi Git: Update .gitignore spesifik & commit baseline | `echo "config/hooks/live/0010-*" >> .gitignore` (dst), `git add .`, `git commit -m "feat: baseline..."`, `git push origin main` | Repository bersih dari artefak build, hook standar ter-ignore, hook custom tetap bisa dideteksi di masa depan |
| 2026-11-14 | Phase 3.3 | Uji boot BIOS dan verifikasi sistem dasar | Boot → Live system → `whoami`, `uname -r`, `ping deb.debian.org` | **SUKSES**: Boot ISOLINUX (biru), user `spider`, internet aktif, XFCE normal |
| 2026-11-14 | Phase 3.3 | Uji boot UEFI dan verifikasi konsistensi | Boot → Live system → `whoami`, `uname -r`, `ping deb.debian.org` | **SUKSES**: Boot GRUB (hitam), semua fungsi identik dengan BIOS |
| 2026-11-24 | Phase 4 | Tulis konsep & palet desain di `docs/desain.md`, samakan dengan palet ANSI Spider-Sense (`ansi.rs`: 88/174/223/180/138 + 108/179/1;31). Guide Phase 4.1, 6.3, 7.1, 7.2, 8.2–8.4 juga disesuaikan (pilihan B) | `python scripts/generate-assets.py` (Pillow) → 6 PNG + preview; `git diff docs/modules/modul_guide.md` | **DUMMY**: 7 aset placeholder sudah ada di `assets-final/` + SVG di `assets-src/`. ⚠️ Ganti dengan aset final sebelum build Phase 6 — cek `docs/desain.md` §4 & `assets-final/README-GANTI-ASET.txt`. Preview: `docs/screenshots/P04-aset-preview-01.png` |
| 2026-11-24 | Dokumentasi | Modul dipecah jadi 19 file phase di `docs/modules/`, dibuat `docs/index.md` + generator; draft integrasi dihapus dan isinya dipindah ke file phase | `python scripts/generate-modul.py` | `modul_guide.md` 2715 baris dari 20 file sumber; 24 link indeks valid; aset masih DUMMY |
| 2026-11-24 | Phase 9 | Rapikan referensi generator, sinkronkan doc Phase 9 dengan 11 modul Rust + 13 test, siapkan paket integrasi hook `0400` (staging script, build list, desktop entry) | `python scripts/generate-modul.py`; `cargo test` | `modul_guide.md` 2776 baris, generator idempotent, 23 link valid, 13 test lulus; hook tetap dibuat di VM |
| 2026-11-24 | Phase 4 | Aset final disiapkan dan dinormalisasi: konversi WEBP/JPEG/AVIF ke PNG, ukuran disesuaikan, logo 400×400 + 128×128, duplikat SVG dihapus | Python + Pillow; cek dimensi via Pillow | 6 PNG final siap; checklist `docs/desain.md` §4 dan `README-GANTI-ASET.txt` tercentang |
| 2026-11-24 | Phase 5 | Siapkan package list lengkap, `scripts/build.sh`, template hook `0050`, dan skrip sync artefak Windows↔VM | `git add` + commit | 4 file siap; VM tinggal `git pull`, lalu salin template ke `config/hooks/live/` |
| 2026-11-24 | Dokumentasi | Strategi build diubah jadi satu build final di Phase 10; Phase 5–9 jadi validasi tanpa build | edit file phase + `python scripts/generate-modul.py` | Referensi `v0.2`–`v0.6` dihapus; checklist disiapkan untuk ISO final |
| 2026-11-24 | Phase 6–9 | Siapkan file tema & hook: wallpaper/greeter, Plymouth, neofetch, prompt, terminalrc, 5 template hook, serta skrip install-hooks & setup-bootloaders | `git add` + commit | Semua konfigurasi masuk repo; VM tinggal `git pull` lalu jalankan 2 skrip, tanpa build |
| 2026-11-24 | Validasi | Cek paket, hook, aset, dan Rust di VM. `neofetch` tidak tersedia di trixie, jadi diganti skrip sendiri; `cargo` host tidak dipakai karena toolchain ada di chroot | `apt-cache show`, `bash -n`, `cargo test` (Windows) | Hook & aset OK; 1 paket diganti; `Cargo.lock` v4 kompatibel dengan rustc trixie 1.85.1 |
| 2026-11-24 | Build v1.0-rc1 | Build final sukses setelah perbaikan `policykit-1-gnome` → `polkitd`+`lxpolkit`, staging sumber Rust, dan patch `build.sh` | `./scripts/build.sh v1.0-rc1`; `tail build.log` | ISO 1,8 G; `P: Build completed successfully`; checksum `d173ec…6508` cocok di Windows |
| 2026-11-24 | Phase 8 | Satukan identitas terminal: skrip `neofetch` dan `spider.txt` dihapus. `neofetch` jadi symlink ke binary Rust; mode default diambil dari nama pemanggil lewat `cli::default_watch()` | edit `cli.rs`, `main.rs`, hook `0300`, `cargo test` | Satu sumber ASCII + palet; `neofetch` snapshot (prompt balik), `sense`/`spider-sense` live; 15 test lulus |
| 2026-11-24 | Phase 10 | Rebuild setelah penyatuan neofetch, lalu uji boot di TEST-VM | `./scripts/build.sh` (tanpa argumen → `spideros-dev.iso`); boot di VMware | Sukses; frame Rust + Status tampil otomatis di terminal, prompt langsung balik tanpa Ctrl+C |
| 2026-11-24 | Phase 10 | Finalisasi artefak: rename ISO jadi `spideros-v1.iso`, buat ulang checksum, bersihkan file lama | `mv`, `sha256sum -c`, `Dismount-DiskImage` | `spideros-v1.iso` 1,8 G; checksum `559c65…2504` COCOK; ISO lama yang ter-mount sebagai drive E: sudah dilepas dan dihapus |
| 2026-11-24 | Dokumentasi | Catat hasil uji Phase 10 ke tabel di bawah; perbedaan BIOS/UEFI tidak dijadikan kriteria (auto-boot by design) | edit `docs/catatan.md`, `docs/index.md` | Phase 1–10 ditandai selesai; item yang belum sempat diuji ditandai jujur |
| 2026-11-24 | Phase 9.5/9.6 | Audit akhir: ternyata Phase 9.5/9.6 hanya ada di doc. Buat package list `content.list.chroot` + `games.list.chroot`, 3 desktop entry, folder komik + README | `write` file baru di `config/` | File siap; perlu rebuild sekali lagi supaya masuk ISO |
| 2026-11-26 | Phase 11 | Verifikasi ISO, flash USB 8 GB (SanDisk Cruzer Blade) dengan Rufus 4.16 mode DD, lalu uji boot di laptop ASUS Vivobook | `Get-FileHash` (8eac0f45…5a74ca cocok), Rufus START → DD Image mode, boot menu → Live system | Boot live system **berhasil**. Bug: tap-to-click touchpad mati, harus klik fisik (dicatat B01) |
| 2026-11-26 | Phase 12 | Siapkan perbaikan bug B01 + B03 + pencegahan bug hardware: konfigurasi libinput tap-to-click + autostart, DPI 120 untuk layar HiDPI, package list hardware (firmware SOF/Intel audio, MediaTek, Bluetooth + modul audio, storage, power, VA-API) | `write`/edit file di `config/`; tabel bug masuk `docs/catatan.md` §12.1; edit `docs/index.md`, `docs/modules/phase-11-usb-hardware.md`, `docs/modules/phase-12-freeze.md` | File siap; **belum di-build**. Langkah berikutnya: rebuild `./scripts/build.sh v1.0` di VM lalu uji ulang di ASUS |

### 10.2 Hasil uji Phase 10

BIOS dan UEFI tidak dibedakan: `live-build` menyembunyikan menu boot dan langsung masuk entri "Live system", sehingga pengalaman keduanya identik. Perbedaan firmware bukan tujuan remastering.

| No | Pengujian | Hasil | Catatan |
|---|---|---|---|
| 1 | Boot menu bertema muncul | N/A | Auto-boot by design; menu disembunyikan `live-build` |
| 2 | Plymouth tampil | Lulus | `P10-boot-bios-1.png` |
| 3 | Desktop auto-login | Lulus | `P10-boot-bios-2.png` |
| 4 | Wallpaper dan tema sesuai | Lulus | `P10-boot-bios-2.png` |
| 5 | Ikon Papirus tampil | Lulus (visual) | Terlihat di desktop; tidak dicek nama paketnya |
| 6 | Terminal: neofetch dan prompt | Lulus | Frame Rust + prompt langsung balik; `P10-boot-bios-2.png` |
| 7 | `cat /etc/os-release` menampilkan SpiderOS | Belum direkam | Hook `0300` mengatur `NAME="SpiderOS"` |
| 8 | Internet jalan (browser) | Belum diuji | — |
| 9 | Firefox terbuka dan membuka situs | Belum diuji | — |
| 10 | VLC terbuka | Belum diuji | — |
| 11 | File manager, mousepad, screenshot | Belum diuji | — |
| 12 | `spider-sense --version` | Belum direkam | Identitas muncul di frame; output eksplisit tidak disimpan |
| 13 | `spider-sense --once --no-color` | Lulus | Frame tampil di `P10-boot-bios-2.png` |
| 14 | `spider-sense --watch` | Lulus | Mode live dijalankan, dihentikan dengan Ctrl+C |
| 15 | `cargo` dan `rustc` tidak ada di ISO final | Lulus (tidak langsung) | Hook `0400` menghapus toolchain; tidak dicek di VM |
| 16 | `sudo ufw status` aktif | Lulus (dari log) | `build.log`: layanan `ufw` di-enable saat build |
| 17 | Reboot dan shutdown normal | Lulus (tidak langsung) | VM beberapa kali di-boot ulang tanpa masalah |
| 18 | "SpiderOS Comics" (mcomix) terbuka | Lulus | Aplikasi jalan dari menu; CBZ terbaca dengan cover dan bookmark |
| 19 | Folder `/usr/share/spideros/comics/` ada | Lulus | Folder dan `README.txt` tersedia di ISO |
| 20 | Game dari repo terbuka dari menu | Lulus | Mines, Sudoku, Bastet, NetHack jalan |
| 21 | `mpv` dan `audacious` terbuka | Lulus | Keduanya jalan dari menu |

Bukti: `P10-boot-bios-1.png` (Plymouth), `P10-boot-bios-2.png` (desktop + terminal + frame Spider-Sense). Data performa belum dikumpulkan.

### 12.1 Bug log (Phase 11–12)

Ditemukan bug selama Phase 10–12. Urutan prioritas: (1) tidak bisa boot, (2) tidak bisa dipakai, (3) tampilan, (4) kosmetik.

| ID | Ditemukan di | Deskripsi | Penyebab | Perbaikan | Status |
|---|---|---|---|---|---|
| B01 | Phase 11 — ASUS Vivobook | Tap-to-click touchpad tidak berfungsi; harus menekan tombol fisik di bawah touchpad | libinput default `Tapping=0` di Debian/XFCE | `config/includes.chroot_after_packages/etc/X11/xorg.conf.d/40-libinput-touchpad.conf` + autostart `spideros-touchpad-tap` | Diperbaiki, menunggu uji ISO baru |
| B02 | Phase 12 — dokumentasi | Checksum di `docs/catatan.md` (`559c65…2504`) tidak cocok dengan ISO final di `dist/` (`8eac0f45…5a74ca`) | ISO dibangun ulang setelah entri catatan lama dibuat | Samakan checksum di catatan/release notes setelah build final | Terbuka |
| B03 | Phase 11 — ASUS Vivobook | Teks dan ikon terasa kecil di layar HiDPI (tajam, tetapi ukurannya mungil) | XFCE memakai 96 DPI, sedangkan Windows memakai 125–150% | `config/includes.chroot_after_packages/etc/skel/.config/xfce4/xfconf/xfce-perchannel-xml/xsettings.xml` → `Xft/DPI = 120` | Diperbaiki, menunggu uji ISO baru |

Catatan:

- B01 adalah bug fungsional prioritas 2 (perangkat tidak nyaman dipakai).
- Setelah rebuild Phase 12, uji ulang B01 di ASUS Vivobook dan perangkat kedua.
- Bug baru ditambahkan di tabel ini sebelum rebuild berikutnya.
