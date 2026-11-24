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
cargo test                          # unit test parser, bar, uptime
cargo build --release --locked      # build release
./target/release/spider-sense --once --no-color
./target/release/spider-sense --watch
cargo clean                         # hapus target/

# Verifikasi integrasi di chroot dan ISO
./scripts/stage-spider-sense.sh
sudo chroot chroot /usr/local/bin/spider-sense --once --no-color
sudo chroot chroot sh -c 'command -v cargo || echo "cargo sudah dibersihkan"'
xorriso -osirrox on -indev dist/spideros-vX.Y.iso \
  -extract /live/filesystem.squashfs /tmp/spideros-fs.squashfs
unsquashfs -l /tmp/spideros-fs.squashfs | grep spider-sense

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
| Rust | `cargo: command not found` di build-VM | `sudo apt install cargo rustc` |
| Rust | Build gagal | Baca error `cargo build`, biasanya typo, import kurang, atau `Cargo.lock` belum ada |
| Rust | Hook 0400 gagal | Pastikan `scripts/stage-spider-sense.sh` dijalankan, `Cargo.lock` ada, dan paket `cargo rustc build-essential` masuk daftar build-only |
| Rust | `--locked` gagal | Jalankan `cargo generate-lockfile` di `tools/spider-sense`, commit `Cargo.lock`, lalu build ulang |
| Rust | Binary tidak ada di ISO | Cek log hook 0400, jalankan `sudo chroot chroot /usr/local/bin/spider-sense --once --no-color` |
| Rust | Error linker di chroot | Pastikan `build-essential` ada di `spider-sense-build.list.chroot` |
| Rust | Tampilan ANSI berantakan | Jalankan `spider-sense --once --no-color` untuk memisahkan masalah data dari masalah rendering |

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
- [ ] Phase 9: Spider-Sense (Rust, komponen inti koding)
- [ ] Phase 9.5: Kolektor komik offline (YACReader)
- [ ] Phase 9.6: Game offline dari repo Debian
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
- Rust Standard Library: https://doc.rust-lang.org/std/
- Cargo Book: https://doc.rust-lang.org/cargo/
- Linux `proc(5)` manual: https://man7.org/linux/man-pages/man5/proc.5.html
- ECMA-48 (Control Functions for Coded Character Sets) untuk ANSI escape sequence
- Dokumentasi Plymouth: lihat `/usr/share/plymouth/themes/` di sistem Debian
- Silberschatz et al. (2018), *Operating System Concepts*
- Harjono (2016) tentang remastering Debian
- IBM, penjelasan Linux

