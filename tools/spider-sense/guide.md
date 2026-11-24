# 🕷️ Spider-Sense: Full Planning & Development Guide

> **Dokumen ini adalah panduan lengkap pengembangan Spider-Sense (Phase 9) yang dikembangkan di Windows terlebih dahulu, lalu dikompilasi final di dalam chroot Debian pada build-VM.**
> Dibuat sebagai dokumentasi alur agar agent/developer berikutnya bisa melanjutkan tanpa kehilangan konteks.
>
> Referensi utama: `docs/modules/phase-09-spider-sense.md`. Navigasi: `docs/index.md`.
> Status progres pengerjaan: lihat `docs/catatan.md`.

---

## 1. Gambaran Besar (Konsep Kerja Dua-Lingkungan)

```
┌───────────────── WINDOWS (fase sekarang) ────────────┐
│  tools/spider-sense/ (source of truth)               │
│  ├── Tulis kode parser, sampler, renderer            │
│  ├── cargo test      ← unit test, 100% jalan di Win  │
│  ├── cargo build     ← binary Windows, buat preview  │
│  └── cargo run       ← lihat UI pakai DATA FIXTURE   │
│                                                      │
│  /proc ada? TIDAK → collector baca dari folder fixture│
└──────────────────────┬───────────────────────────────┘
                       │ git push / shared folder
┌──────────────────────▼───────────────────────────────┐
│  BUILD-VM Debian 13 (fase berikutnya)                │
│  ├── ./scripts/stage-spider-sense.sh                 │
│  │     salin source (tanpa target/) ke staging       │
│  ├── Hook 0400 di dalam chroot:                      │
│  │     cargo build --release --locked --offline      │
│  │     → /usr/local/bin/spider-sense (binary LINUX)  │
│  ├── Purge cargo/rustc + hapus source staging        │
│  └── lb build → ISO                                  │
│                                                      │
│  /proc ada? YA → collector baca /proc asli           │
└──────────────────────────────────────────────────────┘
```

**Kesimpulan penting:**

- Binary Rust yang di-build di Windows **TIDAK dipakai di ISO**. Format binary Windows (PE) tidak jalan di Linux (ELF).
- Yang masuk ISO hanyalah **source code** (`Cargo.toml`, `Cargo.lock`, `src/*.rs`, `tests/`), lalu dikompilasi ULANG di chroot Debian via hook `0400`.
- Peran Windows: penulisan kode + pengujian logika + preview UI.
- Peran VM: kompilasi final + integrasi ISO + pengujian `/proc` asli.

---

## 2. Spesifikasi Spider-Sense (dari Modul 9.1, WAJIB dipenuhi)

CLI tool bertema Spider-Man yang membaca `/proc` secara langsung dan merender antarmuka terminal dengan ANSI.

| Info | Sumber di `/proc` | Konsep SO |
|---|---|---|
| Model CPU | `/proc/cpuinfo` | Informasi hardware |
| CPU (%) | `/proc/stat` (selisih dua sampel) | Penjadwalan CPU |
| RAM total/tersedia | `/proc/meminfo` | Manajemen memori (`MemAvailable`) |
| Jumlah proses | folder angka di `/proc` | Manajemen proses |
| Uptime | `/proc/uptime` | Waktu sistem |
| Load average | `/proc/loadavg` | Antrian proses |
| Versi kernel | `/proc/sys/kernel/osrelease` | Identitas kernel |
| Level bahaya | dihitung dari CPU & RAM | Logika ambang batas |

**Level bahaya:** < 40% = "Tenang", 40–75% = "Waspada", > 75% = "BAHAYA! Spider-Sense berdenyut!".

**Batasan teknis (tidak boleh dilanggar):**

1. ✅ **Hanya standard library Rust (`std`)** — TIDAK ADA crate eksternal (no `sysinfo`, `crossterm`, `ratatui`, `ncurses`).
2. ✅ **Tidak memanggil program lain** (`top`, `free`, `ps` dilarang) — baca `/proc` langsung.
3. ✅ **Parsing manual** — setiap baris dipecah dan dikonversi sendiri.
4. ✅ **Rendering ANSI manual** — escape sequence, bukan library TUI.
5. ✅ **Mode real-time** — sampling berulang, delta CPU antar sampel, redraw tiap 1 detik.

**Field `/proc/stat` yang dipakai:** `user`, `nice`, `system`, `idle`, `iowait`, `irq`, `softirq`, `steal`.
Rumus: `idle = idle + iowait`, `total = jumlah 8 field pertama`, `cpu% = 100 × (1 − Δidle/Δtotal)`.

**Memori:** baca `MemTotal` dan `MemAvailable` → `used = MemTotal − MemAvailable`, `used% = (MemTotal − MemAvailable) / MemTotal × 100`. `MemAvailable` dipilih karena memperhitungkan cache yang bisa dilepas, bukan hanya `MemFree`.

---

## 3. Kunci Desain #1: ProcFS Parametrik (Windows preview tanpa conditional compile)

`/proc` tidak ada di Windows. Solusinya: **collector menerima root path yang bisa ditukar via environment variable**. Tidak ada `#[cfg(windows)]`, kode tetap satu jalur.

```rust
// Pseudocode arsitektur
pub struct ProcFS { root: PathBuf }

impl ProcFS {
    pub fn resolve() -> Self {
        match env::var("SPIDEROS_PROC_ROOT") {
            Ok(p) => ProcFS { root: PathBuf::from(p) },      // preview di Windows (fixture)
            Err(_) => ProcFS { root: PathBuf::from("/proc") }, // asli di Linux/VM
        }
    }
    pub fn read_stat(&self) -> io::Result<String> { fs::read_to_string(self.root.join("stat")) }
    // dst: meminfo, cpuinfo, uptime, loadavg, osrelease, list dir untuk count pids
}
```

- Di VM: tidak ada env var → otomatis `/proc` asli. Hook chroot tidak perlu tahu soal env var.
- Di Windows: set `SPIDEROS_PROC_ROOT` ke folder fixture → seluruh UI berjalan dengan data simulasi.
- Parser tetap **fungsi murni**: `parse_stat(&str) -> CpuSample` (terima teks, kembalikan data) — memenuhi ketentuan modul 9.4 #3–5.

**Jumlah proses** dihitung dari entri folder yang namanya hanya angka. Di Windows ini di-simulasi lewat fixture (folder angka palsu di dalam folder fixture).

---

## 4. Kunci Desain #2: Fixture (`tests/fixtures/proc/`)

Fixture = file teks contoh yang meniru isi `/proc` asli, format sesuai `proc(5)`:

```
tools/spider-sense/
└── tests/
    └── fixtures/
        └── proc/
            ├── stat          # "cpu  user nice system idle iowait irq softirq steal ..." + baris cpuN per core
            ├── meminfo       # "MemTotal: xxx kB", "MemAvailable: xxx kB", dst.
            ├── cpuinfo       # berisi "model name : ..."
            ├── uptime        # "12345.67 98765.43"
            ├── loadavg       # "0.52 0.58 0.59 1/412 3512"
            ├── osrelease     # "6.12.x-trixie-amd64"
            ├── 1/            # folder angka = simulasi PID (untuk count_pids)
            ├── 42/
            └── 1337/
```

**Kegunaan fixture:**

1. **Preview UI di Windows** — `--watch` tampil berdenyut tiap detik, bar & warna berjalan.
2. **Unit test deterministik** — tidak tergantung kondisi mesin; test CPU 90% pakai angka yang ditulis sendiri.
3. **Simulasi beban** — buat 2–3 varian nilai `stat` (idle rendah vs tinggi) untuk melihat status berubah Tenang → Waspada → BAHAYA tanpa beban asli.

Fixture **wajib di-commit** ke Git (dibutuhkan agar `cargo test` bisa jalan di mesin mana pun).

---

## 5. Kunci Desain #3: ANSI Renderer Manual

Renderer 100% manual pakai escape sequence:

| Sequence | Fungsi | Dipakai untuk |
|---|---|---|
| `ESC[2J` | hapus seluruh layar | awal frame watch |
| `ESC[H` | kursor ke pojok kiri atas | gambar ulang dari atas |
| `ESC[1;31m` … `ESC[0m` | merah bold … reset warna | status BAHAYA, tema merah/biru |
| `ESC[?25l` / `ESC[?25h` | sembunyikan / tampilkan kursor | rapi saat watch |
| `ESC[K` | hapus sisa baris | hindari sisa karakter |

Progress bar: blok karakter `█` dan `░`, jumlah proporsional dengan persentase, clamp 0–100.

**Layout satu frame (target):**

```
 🕷 Spider-Sense 1.0.0
 CPU   [████████░░░░░░░░░░░░]  43.2%
 RAM   [██████░░░░░░░░░░░░░░]  31.0%
 CPU   : AMD Ryzen 7 ...
 Kernel: 6.12.0-trixie-amd64
 Uptime: 3h 25m 12s
 Procs : 412
 Load  : 0.52 0.58 0.59
 Status: ⚠ WASPADA
```

**Catatan Windows:** Windows Terminal mendukung VT sequences penuh. Terminal conhost klasik kadang perlu mode VT diaktifkan — kalau output berantakan, jalankan lewat Windows Terminal. Runtime asli di Debian tidak bermasalah.

---

## 6. Arsitektur modul (sesuai Modul 9.2)

```
tools/spider-sense/
├── Cargo.toml          # name=spider-sense, version=1.0.0, edition=2021
│                       # [dependencies] KOSONG
│                       # [profile.release] opt-level="z", lto=true, codegen-units=1, strip=true
├── Cargo.lock          # WAJIB di-commit (hook pakai --locked)
├── guide.md            # dokumen ini
├── src/
│   ├── main.rs         # entry point + orkestrasi --once / --watch + unit test
│   ├── cli.rs          # --once, --watch, --no-color, --help, --version, identitas host
│   ├── model.rs        # struktur data metrik & level bahaya
│   ├── procfs.rs       # akses path /proc (bisa diganti via SPIDEROS_PROC_ROOT)
│   ├── parser.rs       # parsing manual cpuinfo, stat, meminfo, uptime, loadavg
│   ├── sampler.rs      # sampel idle/total sebelumnya → delta CPU
│   ├── metrics.rs      # pengumpul metrik & perhitungan persentase
│   └── render/
│       ├── mod.rs      # frame builder, progress bar, layout neofetch-style
│       ├── ansi.rs     # konstanta warna kanonik (sumber kebenaran palet)
│       ├── art.rs      # ASCII art SpiderOS
│       └── term.rs     # escape sequence: clear screen, hide cursor, alternate screen
└── tests/
    └── fixtures/proc/  # fixture data (di-commit)
```

| Modul | Tanggung jawab |
|---|---|
| `cli.rs` | Parsing argumen `--once`, `--watch` (default), `--no-color`, `--help`, `--version` |
| `parser.rs` | Fungsi murni: teks → struktur data, tidak baca file langsung |
| `sampler.rs` | Simpan sampel `idle`/`total` sebelumnya, hitung delta |
| `metrics.rs` | Baca semua file via `ProcFS`, hitung jumlah folder PID, error ditangani rapi (jangan panic, tampil "N/A") |
| `render/` | Frame builder, progress bar, warna, clear screen, alternate screen anti-flicker |
| Unit test | 13 test: `parser.rs` (7), `metrics.rs` (4), `sampler.rs` (1), `render/mod.rs` (1) |

**`--version` HARUS persis:** `spider-sense 1.0.0 (Rust, std-only)` — diminta di verifikasi Modul 9.7.

**Alur program (sesuai Modul 9.4):**

1. Parsing argumen.
2. Inisialisasi sampler.
3. `--once`: sampel awal → tunggu ~300 ms → sampel kedua → render 1 frame → keluar (dipakai untuk smoke test build).
4. `--watch`: sampel awal → loop: tunggu 1 detik → sampel → render → flush stdout.
5. Tampilkan CPU, RAM, model CPU, kernel, uptime, jumlah proses, load average, status (dari nilai tertinggi CPU/RAM).

---

## 7. Tahapan Eksekusi (Step-by-Step)

### Tahap A — Scaffold proyek (Windows)
- [ ] `cargo new tools/spider-sense` (atau tulis manual struktur di atas)
- [ ] Atur `Cargo.toml` (edition 2021, no deps, profile release per tabel)
- [ ] Buat `tests/fixtures/proc/` beserta isi file fixture
- [ ] Pastikan `.gitignore` repo: `tools/spider-sense/target/` di-IGNORE, tapi `tests/fixtures/` di-COMMIT

### Tahap B — Parser (jantung logika, kerjakan duluan)
- [ ] `parse_cpu_line(&str) -> (idle, total)` — 8 field pertama, `idle = idle + iowait`
- [ ] `parse_meminfo(&str) -> MemInfo` — ambil `MemTotal`, `MemAvailable` (kB)
- [ ] `parse_cpuinfo(&str) -> String` — model name
- [ ] `parse_uptime(&str)`, `parse_loadavg(&str)`, `count_pids(dir) -> usize` (nama folder = angka saja)
- [ ] **Unit test ≥ 5** di `main.rs`; target: `cargo test` hijau penuh di Windows 📸

### Tahap C — Collector + Sampler
- [ ] `ProcFS::resolve()` (env var → fallback `/proc`)
- [ ] `Collector::sample(&self) -> SystemSample` — gabung semua bacaan, error ditangani rapi
- [ ] `CpuSampler { prev_idle, prev_total }` + `usage(&mut self, cur) -> f64` = `100 × (1 − Δidle/Δtotal)`

### Tahap D — Renderer + CLI
- [ ] Renderer: frame builder, progress bar (clamp), warna via enum kecil
- [ ] `--no-color` mematikan warna TAPI TIDAK mematikan clear screen di watch (ketentuan #9 modul)
- [ ] CLI manual: `--once`, `--watch` (default), `--no-color`, `--help`, `--version`
- [ ] Alur main sesuai spesifikasi 9.4 (once = 2 sampel 300ms; watch = loop 1 detik)

### Tahap E — Preview & polish di Windows
- [ ] `cargo build --release` lalu jalankan:
  ```powershell
  $env:SPIDEROS_PROC_ROOT="tests\fixtures\proc"
  .\target\release\spider-sense.exe --watch
  .\target\release\spider-sense.exe --once --no-color
  .\target\release\spider-sense.exe --help
  .\target\release\spider-sense.exe --version
  ```
- [ ] Uji varian fixture (CPU rendah vs tinggi) → status berubah
- [x] Commit: `Cargo.toml`, `Cargo.lock`, `src/*.rs`, `tests/` (sudah masuk commit `1d39972`)
  ```bash
  git add tools/spider-sense
  git commit -m "feat: spider-sense rust std-only (windows dev, unit test lulus)"
  ```

### Tahap F — Artefak integrasi ISO (file ditulis SEKARANG, dieksekusi nanti di VM)

| File | Isi |
|---|---|
| `config/package-lists/spider-sense-build.list.chroot` | `cargo`, `rustc`, `build-essential` (build-only) |
| `scripts/stage-spider-sense.sh` | Salin `tools/spider-sense/` → `config/includes.chroot_after_packages/opt/spider-sense-src/`, exclude `target/` dan `.git/` |
| `config/hooks/live/0400-spider-sense-build.hook.chroot` | Validasi source → `cargo build --release --locked --offline` → install ke `/usr/local/bin/spider-sense` → strip → smoke test `--once --no-color` → **hapus source & target** → purge `cargo rustc build-essential` → `apt-get clean` |
| `config/includes.chroot_after_packages/usr/share/applications/spider-sense.desktop` | Desktop entry, `Exec=xfce4-terminal --hold -e spider-sense` |
| `.gitignore` | Tambah `config/includes.chroot_after_packages/opt/spider-sense-src/` (staging generated) |

- [ ] `scripts/build.sh` → panggil `./scripts/stage-spider-sense.sh` tepat setelah `cd "$(dirname "$0")/.."`
- [ ] Semua file di-commit meski belum dieksekusi (executable bit: `chmod +x` saat nanti di VM)

### Tahap G — Build Linux final (BUILD-VM, nanti)
- [ ] `./scripts/build.sh v0.6` → hook 0400 mengompilasi binary Linux di chroot
- [ ] Verifikasi di chroot:
  ```bash
  sudo chroot chroot /usr/local/bin/spider-sense --once --no-color
  sudo chroot chroot file /usr/local/bin/spider-sense       # ELF 64-bit
  sudo chroot chroot sh -c 'command -v cargo || echo "cargo sudah dibersihkan"'
  ```
- [ ] Boot TEST-VM → verifikasi lengkap Modul 9.7:
  ```bash
  command -v spider-sense       # /usr/local/bin/spider-sense
  file /usr/local/bin/spider-sense
  spider-sense --version        # spider-sense 1.0.0 (Rust, std-only)
  spider-sense --once --no-color
  spider-sense --watch
  test ! -d /opt/spider-sense-src && echo "OK: source staging tidak ikut"
  ! command -v cargo && echo "OK: cargo tidak ikut"
  ! command -v rustc && echo "OK: rustc tidak ikut"
  ```
- [ ] Commit + snapshot VM `P9-spidersense` + checklist 9.9 modul

---

## 8. Pengetahuan Penting: Kenapa Kompilasi Ulang di Chroot (bukan copy binary)

Jawaban untuk dosen (hafalkan):

1. **Kepatuhan arsitektur target** — ISO live harus berisi binary ELF x86-64 untuk glibc Debian. Binary Windows (PE) tidak jalan, dan binary dari distro/glibc lain bisa gagal load.
2. **Reproducible** — hook + `Cargo.lock` + `--locked` + `--offline` berarti siapa pun yang build dari repo yang sama mendapat binary identik.
3. **ISO tetap bersih** — toolchain (`cargo`, `rustc`, `build-essential`, ratusan MB) di-purge setelah kompilasi; ISO hanya berisi binary kecil (profile `opt-level="z"` + `strip` → ratusan KB).

### ⚠️ Jebakan `--locked` (lintas-platform)

`Cargo.lock` yang dibuat cargo Windows versi lebih baru bisa memakai format lockfile versi lebih tinggi dari rustc di repo trixie → hook `--locked` bisa gagal.

- Solusi (sudah ada di Lampiran B modul): jalankan `cargo generate-lockfile` dengan versi yang cocok, atau sesuaikan lockfile.
- **Jangan menambah dependency apa pun** → lockfile tetap minimal, risiko konflik kecil.
- Amankan: lockfile tetap di lock version 3 (standar aman).

---

## 9. Batasan & Yang Tidak Bisa Dikerjakan di Windows

| Hal | Kenapa | Kapan dieksekusi |
|---|---|---|
| `/proc` asli (nilai live) | Tidak ada di Windows | Tahap G (TEST-VM) |
| Jumlah proses real | `/proc` tidak ada | Tahap G |
| Delta CPU saat beban asli (`yes > /dev/null &`) | Butuh `/proc` live | Tahap G |
| Hook 0400, staging, `lb build` | Butuh live-build + chroot Debian | Tahap G |

Semua yang di atas sudah disiapkan filenya di Tahap F, meski belum dieksekusi.

---

## 10. Cheat Sheet Perintah

```powershell
# Windows: test & preview
cd tools\spider-sense
cargo test
cargo build --release
$env:SPIDEROS_PROC_ROOT="tests\fixtures\proc"
.\target\release\spider-sense.exe --watch
.\target\release\spider-sense.exe --once --no-color
.\target\release\spider-sense.exe --version
```

```bash
# Build-VM: integrasi & build final
cd ~/spideros   # (di repo SpiderOS)
./scripts/build.sh v0.6
sudo chroot chroot /usr/local/bin/spider-sense --once --no-color
# TEST-VM: verifikasi Modul 9.7 (lihat Tahap G di atas)
```

---

## 11. Checklist Akhir Phase 9 (dari Modul 9.9)

- [x] Proyek Rust ada di `tools/spider-sense/` dengan `Cargo.toml`, `Cargo.lock`, dan 11 file di `src/`
- [x] Tidak ada dependency eksternal; hanya `std`
- [x] `cargo test` lulus (13 unit test, dijalankan di Windows dengan fixture)
- [ ] `cargo build --release --locked` sukses
- [ ] `--once` dan `--watch` berjalan (preview di Windows dengan fixture → runtime asli di VM)
- [ ] `0400-spider-sense-build.hook.chroot` ada dan executable
- [ ] `spider-sense-build.list.chroot` dan `scripts/stage-spider-sense.sh` ada
- [ ] Binary berada di `/usr/local/bin/spider-sense` di dalam chroot
- [ ] `cargo` dan `rustc` tidak ikut ke ISO final
- [ ] Tim dapat menjelaskan alur parser, sampler, renderer ANSI, dan hook build
- [ ] Commit dan snapshot (`P9-spidersense`)

✅ **Selesai kalau:** `spider-sense` berhasil dikompilasi di chroot dan berjalan di ISO.

---

## 12. Catatan untuk Agent/Developer Berikutnya

1. **Jangan tambahkan crate eksternal** — spesifikasi modul mewajibkan std-only. Ini dipertanyakan dosen.
2. **Jangan baca `/proc` langsung di parser** — parser harus fungsi murni (teks → data). Reading hanya di collector via `ProcFS`.
3. **Jangan pakai `#[cfg(windows)]`** — jalur preview di Windows sudah ditangani env var `SPIDEROS_PROC_ROOT`.
4. **`Cargo.lock` wajib di-commit** dan jaga formatnya kompatibel dengan rustc trixie (jebakan `--locked`, lihat Bab 8).
5. **`--version` output harus persis** `spider-sense 1.0.0 (Rust, std-only)`.
6. **`--no-color`**: matikan warna, tapi clear screen di mode `--watch` TETAP aktif (ketentuan #9 modul).
7. Error handling collector: **jangan panic** saat file tidak terbaca — tampilkan "N/A" agar program tetap jalan.
8. Setelah selesai di Windows, lanjutkan ke Tahap F (tulis artefak integrasi) lalu Tahap G (eksekusi di VM). Catat progres di `docs/catatan.md`.
9. **Renderer sudah gaya neofetch**: ASCII Debian (kredit di `docs/credits.md`) kiri, info sistem kanan, blok warna tema. Konsisten dengan identitas remaster Debian → SpiderOS.
10. **Fitur preview `stat.b`**: bila `stat.b` ada di root fixture, `--watch` mengambil `stat`/`stat.b` bergantian sehingga CPU bergerak. Di `/proc` asli file ini tidak ada — runtime produksi tidak berubah.

---

*Terakhir diupdate: 2026-11-14 — dibuat sebagai full planning fase pengembangan di Windows.*
