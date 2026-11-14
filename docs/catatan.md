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
