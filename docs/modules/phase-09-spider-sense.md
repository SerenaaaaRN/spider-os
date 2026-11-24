## PHASE 9: System Programming dalam Rust — Spider-Sense (Komponen Inti)

**Tujuan:** membangun tool CLI bertema Spider-Man yang membaca `/proc` secara langsung, mem-parsing data kernel secara manual, dan merender informasi sistem secara real-time dengan ANSI, lalu mengompilasinya di dalam chroot saat proses build ISO.
**Status:** inti (wajib). Phase ini adalah komponen teknis utama dan tidak boleh dilewati.

### 9.1 Spesifikasi dan batasan teknis

Spider-Sense menampilkan kondisi sistem dengan gaya Spider-Man:

| Info | Sumber di `/proc` | Konsep SO |
|---|---|---|
| Model CPU | `/proc/cpuinfo` | Informasi hardware |
| Penggunaan CPU (%) | `/proc/stat` (selisih dua sampel) | Penjadwalan CPU, waktu user/system/idle |
| RAM total dan tersedia | `/proc/meminfo` | Manajemen memori (`MemAvailable`) |
| Jumlah proses | folder angka di `/proc` | Manajemen proses (satu folder per PID) |
| Uptime | `/proc/uptime` | Waktu sistem berjalan |
| Beban rata-rata | `/proc/loadavg` | Antrian proses (load) |
| Versi kernel | `/proc/sys/kernel/osrelease` | Identitas kernel |
| Level "bahaya" | dihitung dari CPU dan RAM | Logika ambang batas |

Level bahaya: di bawah 40% = "Tenang", 40–75% = "Waspada", di atas 75% = "BAHAYA! Spider-Sense berdenyut!".

Batasan teknis yang menjadi inti kompetensi koding:

- **Hanya memakai standard library Rust (`std`)**. Tidak ada crate eksternal seperti `sysinfo`, `crossterm`, `ratatui`, atau `ncurses`.
- **Tidak memanggil program lain** seperti `top`, `free`, atau `ps`. Semua data dibaca langsung dari `/proc`.
- **Parsing manual**: setiap baris `/proc/stat`, `/proc/meminfo`, `/proc/cpuinfo`, `/proc/uptime`, dan `/proc/loadavg` dipecah serta dikonversi sendiri.
- **Rendering ANSI manual**: kursor, pembersihan layar, warna, dan progress bar dibentuk dari escape sequence, bukan library TUI.
- **Mode real-time**: program mengambil sampel berulang, menghitung delta CPU dari sampel sebelumnya, lalu menggambar ulang layar setiap detik.

Penjelasan field `/proc/stat` yang dipakai:

| Field | Arti |
|---|---|
| `user` | Waktu CPU untuk proses user |
| `nice` | Waktu CPU untuk proses user dengan prioritas nice |
| `system` | Waktu CPU untuk proses kernel |
| `idle` | Waktu CPU idle |
| `iowait` | Waktu menunggu I/O |
| `irq`, `softirq` | Waktu menangani interrupt |
| `steal` | Waktu CPU yang diambil hypervisor (relevan di VM) |

Rumus yang dipakai: `idle = idle + iowait`, `total = jumlah delapan field pertama`, lalu `cpu% = 100 × (1 − Δidle/Δtotal)`.

Untuk memori, program membaca `MemTotal` dan `MemAvailable` dari `/proc/meminfo`, lalu menghitung `used = MemTotal − MemAvailable`. `MemAvailable` dipilih karena memperhitungkan cache yang dapat dilepas, bukan hanya `MemFree`.

### 9.2 Arsitektur proyek dan posisi `tools/`

Proyek Rust disimpan di `tools/spider-sense/` dan menjadi sumber kebenaran (canonical source). Folder ini tidak hanya berisi binary, tetapi proyek Cargo lengkap:

```
tools/spider-sense/
├── Cargo.toml          # metadata, profile release, tanpa dependencies
├── Cargo.lock          # dikunci untuk build reproducible
├── guide.md            # catatan pengembangan Phase 9
├── src/
│   ├── main.rs         # entry point + orkestrasi --once / --watch
│   ├── cli.rs          # argumen CLI dan identitas lingkungan
│   ├── model.rs        # struktur metrik dan level bahaya
│   ├── procfs.rs       # akses /proc (path bisa diganti lewat SPIDEROS_PROC_ROOT)
│   ├── parser.rs       # parsing manual cpuinfo, stat, meminfo, uptime, loadavg
│   ├── sampler.rs      # sampel idle/total dan delta CPU
│   ├── metrics.rs      # pengumpul metrik dan persentase
│   └── render/
│       ├── mod.rs      # frame builder, progress bar, layout
│       ├── ansi.rs     # konstanta warna kanonik (sumber kebenaran palet)
│       ├── art.rs      # ASCII art SpiderOS
│       └── term.rs     # escape sequence, alternate screen anti-flicker
└── tests/fixtures/proc/  # data fixture untuk pengujian di Windows
```

Kode dibagi menjadi beberapa modul agar mudah dijelaskan di laporan:

1. **CLI** (`cli.rs`) — `--once`, `--watch`, `--no-color`, `--help`, `--version`.
2. **Parser `/proc`** (`parser.rs`) — fungsi murni yang menerima teks dan mengembalikan struktur data.
3. **Sampler CPU** (`sampler.rs`) — menyimpan sampel sebelumnya dan menghitung delta.
4. **Renderer ANSI** (`render/`) — menyusun frame, progress bar, warna, dan kontrol layar.
5. **Unit test** — 13 test: `parser.rs` (7), `metrics.rs` (4), `sampler.rs` (1), dan `render/mod.rs` (1).

Saat build, sumber disalin sementara ke `config/includes.chroot_after_packages/opt/spider-sense-src/` agar dapat diakses hook di dalam chroot. Direktori staging ini bersifat generated dan tidak di-commit; sumber asli tetap di `tools/spider-sense/`.

### 9.3 Setup proyek Rust di build-VM

Pastikan Rust sudah terpasang (Phase 1.4):

```bash
cargo --version
rustc --version
```

Buat proyek:

```bash
cd ~/spideros/tools
cargo new spider-sense
cd spider-sense
```

Atur `Cargo.toml` dengan ketentuan berikut (tanpa menambah dependency):

| Bagian | Nilai |
|---|---|
| `name` | `spider-sense` |
| `version` | `1.0.0` |
| `edition` | `2021` |
| `[dependencies]` | kosong |
| `[profile.release]` | `opt-level = "z"`, `lto = true`, `codegen-units = 1`, `strip = true` |

Profile release dipakai agar binary kecil dan cepat. Setelah build pertama, `Cargo.lock` terbentuk; commit file tersebut agar hook dapat memakai `--locked`.

### 9.4 Panduan implementasi Spider-Sense

Implementasi sudah ada di `tools/spider-sense/src/` dan mengikuti spesifikasi berikut. Bagian ini dipakai untuk menjelaskan keputusan desain kode di laporan.

**Struktur yang disarankan** (boleh satu file, boleh dipecah menjadi modul):

| Bagian | Tanggung jawab |
|---|---|
| CLI | Membaca `--once`, `--watch`, `--no-color`, `--help`, `--version`; default `--watch`. |
| Parser `/proc` | Fungsi yang menerima teks dan mengembalikan data; tidak membaca file langsung agar mudah diuji. |
| Sampler CPU | Menyimpan sampel `idle` dan `total` sebelumnya, lalu menghitung delta. |
| Collector | Membaca `/proc/stat`, `/proc/meminfo`, `/proc/cpuinfo`, `/proc/uptime`, `/proc/loadavg`, `/proc/sys/kernel/osrelease`, dan menghitung jumlah folder PID di `/proc`. |
| Renderer ANSI | Menyusun frame, progress bar, warna, dan pembersihan layar dengan escape sequence. |
| Unit test | Menguji parser, perhitungan persentase memori, batas progress bar, dan format uptime. |

**Ketentuan implementasi:**

1. Gunakan hanya `std`. Jangan menambah crate eksternal.
2. Jangan memanggil `top`, `free`, `ps`, atau program lain; baca `/proc` langsung.
3. `parse_cpu_line` membaca baris pertama `/proc/stat`, mengambil delapan field pertama, menghitung `idle = idle + iowait` dan `total = jumlah delapan field`, lalu mengembalikan `(idle, total)`.
4. Penggunaan CPU dihitung dari dua sampel: `100 × (1 − Δidle/Δtotal)`.
5. `parse_meminfo` mengambil `MemTotal` dan `MemAvailable`; persentase memori memakai `(MemTotal − MemAvailable) / MemTotal × 100`.
6. Jumlah proses dihitung dari entri `/proc` yang namanya hanya angka.
7. Mode `--watch` mengambil sampel setiap 1 detik, membersihkan layar dengan ANSI (`ESC[2J` dan `ESC[H`), lalu menggambar ulang.
8. Mode `--once` mencetak satu frame lalu keluar; mode ini dipakai untuk smoke test build.
9. `--no-color` mematikan warna, tetapi tidak mematikan pembersihan layar pada mode watch.
10. Sediakan minimal 5 unit test untuk parser dan fungsi bantu.

**Alur program yang diharapkan:**

1. Parsing argumen.
2. Inisialisasi sampler.
3. Jika `--once`: ambil sampel awal, tunggu sekitar 300 ms, ambil sampel kedua, render satu frame, keluar.
4. Jika `--watch`: ambil sampel awal, lalu loop: tunggu 1 detik, ambil sampel, render, flush stdout.
5. Tampilkan CPU, RAM, model CPU, kernel, uptime, jumlah proses, load average, dan status berdasarkan nilai tertinggi CPU/RAM.

### 9.5 Build dan unit test di build-VM

```bash
cd ~/spideros/tools/spider-sense
cargo test
cargo build --release --locked
./target/release/spider-sense --once --no-color
./target/release/spider-sense --watch
```

Yang diharapkan:

- `cargo test` menjalankan 13 unit test dan semuanya lulus.
- `cargo build --release --locked` menghasilkan `target/release/spider-sense`.
- `--once --no-color` menampilkan satu frame metrik tanpa escape sequence warna.
- `--watch` menampilkan layar yang diperbarui setiap detik.

Di Windows, jalankan preview memakai fixture tanpa `/proc` asli:

```powershell
$env:SPIDEROS_PROC_ROOT = "tests\fixtures\proc"
.\target\release\spider-sense.exe --once --no-color
.\target\release\spider-sense.exe --watch
```

Environment `SPIDEROS_PROC_ROOT` hanya dipakai untuk pengujian. Di VM Debian, variabel ini tidak diset supaya program membaca `/proc` sungguhan.

Uji beban agar status berubah:

```bash
# terminal lain
yes > /dev/null &
# kembali ke terminal Spider-Sense, lalu hentikan beban
kill %1
```

Commit `Cargo.toml`, `Cargo.lock`, dan `src/main.rs`:

```bash
cd ~/spideros
git add tools/spider-sense
git commit -m "feat: spider-sense rust std-only"
```

### 9.6 Integrasi ke ISO: kompilasi di dalam chroot via hook 0400

Binary tidak dibangun di host lalu disalin. Proyek Rust disalin sementara ke chroot, dikompilasi oleh hook `0400-spider-sense-build.hook.chroot`, lalu toolchain build dibersihkan agar tidak ikut ke ISO. Pola ini menunjukkan pengelolaan build pipeline yang reproducible dan sesuai arsitektur target.

**File integrasi:**

| File | Isi/tanggung jawab | Status di repo |
|---|---|---|
| `config/package-lists/spider-sense-build.list.chroot` | `cargo`, `rustc`, `build-essential` (build-only). | Sudah ada |
| `scripts/stage-spider-sense.sh` | Menyalin `tools/spider-sense/` ke `config/includes.chroot_after_packages/opt/spider-sense-src/`, mengecualikan `target/` dan `.git/`. | Sudah ada, executable |
| `config/hooks/live/0400-spider-sense-build.hook.chroot` | Validasi source, `cargo build --release --locked --offline`, install binary ke `/usr/local/bin/spider-sense`, strip, smoke test, hapus source/target, purge paket build-only, apt clean. | Dibuat di VM (`config/hooks/` di-ignore); isi di bawah |
| `config/includes.chroot_after_packages/usr/share/applications/spider-sense.desktop` | Desktop entry dengan `Exec=xfce4-terminal --hold -e spider-sense`. | Sudah ada |
| `.gitignore` | Sudah memuat `config/includes.chroot_after_packages/opt/spider-sense-src/`. | Sudah ada |

Isi hook `0400` — simpan di VM sebagai `config/hooks/live/0400-spider-sense-build.hook.chroot`, lalu `chmod +x`:

```bash
#!/bin/bash
# Kompilasi Spider-Sense di dalam chroot, lalu bersihkan toolchain build.
set -e

SRC=/opt/spider-sense-src
BIN=/usr/local/bin/spider-sense

if [ ! -f "$SRC/Cargo.toml" ]; then
    echo "ERROR: sumber Rust tidak ditemukan di $SRC" >&2
    exit 1
fi

cd "$SRC"
cargo build --release --locked --offline
install -m 0755 target/release/spider-sense "$BIN"
strip "$BIN" || true

# Smoke test ringan selalu jalan; --once butuh /proc yang ter-mount di chroot.
"$BIN" --version
if [ -d /proc/self ]; then
    "$BIN" --once --no-color
else
    echo "INFO: /proc tidak tersedia di chroot; smoke test --once dilewati"
fi

cd /
rm -rf "$SRC"

# Toolchain hanya dipakai saat build, jangan ikut ke ISO.
apt-get purge -y --auto-remove cargo rustc build-essential || true
apt-get clean
rm -rf /var/lib/apt/lists/*
```

**Urutan kerja:**

1. Tambahkan paket build-only ke daftar paket.
2. Buat skrip staging dan pastikan executable.
3. Buat hook `0400` dan pastikan executable.
4. Panggil `./scripts/stage-spider-sense.sh` di `scripts/build.sh` tepat setelah `cd "$(dirname "$0")/.."`.
5. Buat desktop entry.
6. Build dengan `./scripts/build.sh v0.6`.
7. Verifikasi di chroot: binary ELF, smoke test, dan `cargo`/`rustc` sudah tidak ada.

### 9.7 Verifikasi di ISO

Boot ISO hasil build di `[TEST-VM]`, lalu jalankan:

```bash
command -v spider-sense
file /usr/local/bin/spider-sense
spider-sense --version
spider-sense --once --no-color
spider-sense --watch
```

Yang diharapkan:

- `command -v` menemukan `/usr/local/bin/spider-sense`.
- `file` menampilkan executable ELF 64-bit, bukan shell script.
- `--version` menampilkan `spider-sense 1.0.0 (Rust, std-only)`.
- `--once --no-color` menampilkan CPU, RAM, proses, uptime, dan load dari `/proc` aktif.
- `--watch` memperbarui layar setiap detik dan status berubah saat CPU dibebani.

### 9.8 Bahan penjelasan untuk dosen

| Pertanyaan | Jawaban inti |
|---|---|
| Apa itu `/proc`? | Filesystem virtual yang dibuat kernel di memori; file di dalamnya tidak ada di disk, isinya informasi kernel saat itu juga |
| Kenapa tiap proses punya folder angka? | Nama folder = PID, kernel menyediakan info per proses di sana |
| Kenapa CPU dihitung dari dua sampel? | `/proc/stat` berisi hitungan kumulatif sejak boot; penggunaan didapat dari selisih dua sampel dalam rentang waktu |
| Kenapa pakai `MemAvailable`, bukan `MemFree`? | `MemFree` tidak menghitung cache yang bisa dilepas; `MemAvailable` memperkirakan memori yang benar-benar bisa dipakai aplikasi |
| Apa itu load average? | Rata-rata jumlah proses yang siap jalan atau menunggu I/O dalam 1, 5, 15 menit |
| Bagaimana rendering ANSI dilakukan? | Escape sequence `ESC[2J` membersihkan layar, `ESC[H` memindahkan kursor, dan kode warna seperti `ESC[1;31m` memberi warna merah |
| Kenapa tidak memakai library monitoring? | Tujuannya melatih pemrograman sistem: membaca format kernel, mem-parsing manual, dan membangun renderer sendiri |
| Di mana kode dikompilasi? | Di dalam chroot saat build ISO, melalui hook `0400-spider-sense-build.hook.chroot`; bukan binary yang dibangun di host |
| Bagaimana membuktikan binary masuk ISO? | `file`, `--version`, `--once`, dan `--watch` dijalankan di `[TEST-VM]`; lihat Phase 10.3 |
| Kenapa memakai Rust? | Rust memberi memory safety tanpa garbage collector, binary native tanpa runtime bahasa, dan cocok untuk tool sistem yang ringan |
| Apa itu ownership dan borrowing? | Sistem Rust yang menjamin keamanan memori tanpa garbage collector; cocok untuk tool sistem yang harus ringan dan aman |

### 9.9 Checklist Phase 9

- [x] Proyek Rust ada di `tools/spider-sense/` dengan `Cargo.toml`, `Cargo.lock`, dan 11 file di `src/`
- [x] Tidak ada dependency eksternal; hanya `std`
- [x] `cargo test` lulus (13 unit test)
- [x] `cargo build --release --locked` sukses
- [x] `--once` dan `--watch` berjalan dengan fixture di Windows
- [x] `spider-sense-build.list.chroot`, `scripts/stage-spider-sense.sh`, dan desktop entry ada
- [ ] `--once` dan `--watch` berjalan di build-VM dengan `/proc` asli
- [ ] `0400-spider-sense-build.hook.chroot` dibuat di VM dan executable
- [ ] Binary berada di `/usr/local/bin/spider-sense` di dalam chroot
- [ ] `cargo` dan `rustc` tidak ikut ke ISO final
- [ ] Tim dapat menjelaskan alur parser, sampler, renderer ANSI, dan hook build
- [ ] Commit dan snapshot (`P9-spidersense`)

✅ **Selesai kalau:** `spider-sense` berhasil dikompilasi di chroot dan berjalan di ISO. Jika gagal, perbaiki pipeline sebelum melanjutkan ke Phase 10.

---

