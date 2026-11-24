## PHASE 10: Build ISO v1 dan Uji Menyeluruh di VM

**Tujuan:** menghasilkan ISO kandidat rilis (v1.0-rc1) dari build bersih dan mengujinya sistematis.
**Output:** `spideros-v1.0-rc1.iso`, checksum, tabel hasil uji, data performa untuk laporan.

### 10.1 Build bersih dari nol

Ini satu-satunya build penuh. Seluruh konfigurasi Phase 5–9 sudah disiapkan dan divalidasi tanpa build sebelum perintah ini dijalankan.

```bash
cd ~/spideros
./scripts/build.sh v1.0-rc1
```

`scripts/build.sh` menjalankan `sudo lb clean` (cache tetap dipakai), lalu `lb config` dan `lb build`. Tidak memakai `--purge`.

Output: `dist/spideros-v1.0-rc1.iso` dan `dist/spideros-v1.0-rc1.iso.sha256`.

### 10.2 Uji fungsional lengkap

Uji di VM BIOS dan VM UEFI. Isi tabel (salin ke `docs/catatan.md`):

| No | Pengujian | BIOS | UEFI | Catatan |
|---|---|---|---|---|
| 1 | Boot menu bertema muncul |  |  |  |
| 2 | Plymouth tampil |  |  |  |
| 3 | Desktop auto-login |  |  |  |
| 4 | Wallpaper dan tema sesuai |  |  |  |
| 5 | Ikon Papirus tampil |  |  |  |
| 6 | Terminal: neofetch dan prompt |  |  |  |
| 7 | `cat /etc/os-release` menampilkan SpiderOS |  |  |  |
| 8 | Internet jalan (browser) |  |  |  |
| 9 | Firefox terbuka dan membuka situs |  |  |  |
| 10 | VLC terbuka |  |  |  |
| 11 | File manager, mousepad, screenshot |  |  |  |
| 12 | `spider-sense --version` menampilkan identitas Rust |  |  |  |
| 13 | `spider-sense --once --no-color` menampilkan metrik `/proc` |  |  |  |
| 14 | `spider-sense --watch` memperbarui layar real-time |  |  |  |
| 15 | `cargo` dan `rustc` tidak ada di ISO final |  |  |  |
| 16 | `sudo ufw status` aktif |  |  |  |
| 17 | Reboot dan shutdown normal |  |  |  |
| 18 | "SpiderOS Comics" (YACReader) terbuka, cover + status dibaca jalan |  |  |  |
| 19 | Folder `/usr/share/spideros/comics/` ada dengan README |  |  |  |
| 20 | Game dari repo terbuka dari menu (`bastet`, NetHack, Mines, Sudoku) |  |  |  |
| 21 | `mpv` dan `audacious` terbuka |  |  |  |

Isi dengan `Lulus` atau `Gagal` plus catatan.

### 10.3 Verifikasi khusus binary Spider-Sense

Verifikasi ini membuktikan bahwa komponen koding benar-benar terintegrasi ke ISO, bukan hanya ada di repositori.

Di `[TEST-VM]`:

```bash
command -v spider-sense
file /usr/local/bin/spider-sense
spider-sense --version
spider-sense --once --no-color
spider-sense --watch
```

Yang diharapkan:

- `command -v` menemukan `/usr/local/bin/spider-sense`.
- `file` menampilkan executable ELF 64-bit, bukan skrip.
- `--version` menampilkan `spider-sense 1.0.0 (Rust, std-only)`.
- `--once --no-color` menampilkan CPU, RAM, jumlah proses, uptime, dan load dari `/proc`.
- `--watch` memperbarui layar setiap detik.
- `command -v cargo` dan `command -v rustc` gagal, tanda toolchain build sudah dibersihkan.

Bukti tambahan bahwa source dan toolchain tidak ikut ke ISO:

```bash
test ! -d /opt/spider-sense-src && echo "OK: source staging tidak ikut"
! command -v cargo && echo "OK: cargo tidak ikut"
! command -v rustc && echo "OK: rustc tidak ikut"
```

Di `[BUILD-VM]`, verifikasi statis isi ISO tanpa boot:

```bash
xorriso -osirrox on -indev dist/spideros-v1.0-rc1.iso \
  -extract /live/filesystem.squashfs /tmp/spideros-fs.squashfs

unsquashfs -l /tmp/spideros-fs.squashfs | grep -E "spider-sense|spider-sense.desktop"

unsquashfs -d /tmp/spideros-root /tmp/spideros-fs.squashfs usr/local/bin/spider-sense
file /tmp/spideros-root/usr/local/bin/spider-sense
```

📸 `P10-spidersense-01.png` (output `--once`), `P10-spidersense-watch-01.png` (tampilan `--watch`).

### 10.4 Data performa untuk BAB IV (bukti "ringan dan efisien")

Jalankan di `[TEST-VM]` setelah desktop idle sekitar 1 menit:

```bash
free -h                              # RAM terpakai saat idle
ps aux | wc -l                       # jumlah proses
systemd-analyze                      # waktu boot
systemd-analyze blame | head -10     # layanan paling lama
df -h | grep -E "overlay|squashfs|/run/live"
```

Di build-VM:

```bash
ls -lh dist/spideros-v1.0-rc1.iso    # ukuran ISO
```

Catat semua angka. Sebagai pembanding, data RAM idle Debian XFCE default atau Ubuntu pada VM yang sama dapat dicatat.

### 10.5 Hentikan penambahan fitur

Setelah rc1 jadi, daftar bug masuk ke `docs/bug-log.md` (format di Phase 12). Fitur baru tidak boleh ditambah, kecuali perbaikan bug.

### 10.6 Commit, tag, snapshot

```bash
git add -A
git commit -m "release: v1.0-rc1"
git tag v1.0-rc1
```

Snapshot: `P10-rc1`.

### 10.7 Checklist Phase 10

- [ ] Build bersih sukses, checksum dibuat
- [ ] 21 pengujian dicoba di BIOS dan UEFI, hasil tercatat
- [ ] Verifikasi khusus `spider-sense` (binary ELF, `--version`, `--once`, `--watch`, toolchain bersih) lulus
- [ ] Data performa terkumpul
- [ ] Tag Git dan snapshot dibuat

✅ **Selesai kalau:** rc1 boot dan fungsional di VM, daftar bug (jika ada) tercatat.

---

