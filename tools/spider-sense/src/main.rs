//! # Spider-Sense
//!
//! Sistem monitor CLI SpiderOS yang membaca `/proc` secara langsung,
//! mem-parsing data kernel secara manual, dan merender antarmuka terminal
//! dengan escape sequence ANSI.
//!
//! Batasan spesifikasi (modul 9.1):
//! - Hanya [`std`] — tanpa crate eksternal.
//! - Tidak memanggil program eksternal (`top`, `free`, `ps`, dst).
//! - Parsing manual terhadap `proc(5)`.
//! - Rendering ANSI manual, tanpa library TUI.
//!
//! ## Arsitektur
//!
//! Kode terbagi lima bagian agar mudah dijelaskan:
//!
//! 1. [`model`] — struktur data (`CpuSample`, `MemInfo`, `SystemSample`).
//! 2. [`parser`] — fungsi murni `teks → data`, tidak membaca file langsung.
//! 3. [`collector`] — satu-satunya bagian yang membaca filesystem via [`ProcFS`].
//! 4. [`render`] — renderer ANSI (frame, progress bar, warna).
//! 5. [`cli`] — parsing argumen dan entry point [`main`].
//!
//! ## Preview lintas-platform
//!
//! `/proc` hanya ada di Linux. Untuk pengembangan di Windows, [`ProcFS::resolve`]
//! membaca env `SPIDEROS_PROC_ROOT` sebagai path data alternatif; tanpa env var
//! tersebut program memakai `/proc` asli.
//!
//! Contoh preview di Windows:
//!
//! ```text
//! $env:SPIDEROS_PROC_ROOT = "tests/fixtures/proc"
//! cargo run -- --watch
//! ```

use std::env;
use std::cell::Cell;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

// === Model ========================================================

/// Satu sampel agregasi baris `cpu` dari `/proc/stat`.
///
/// Nilai di `/proc/stat` adalah hitungan kumulatif dalam satuan
/// `USER_HZ` sejak boot, sehingga penggunaan CPU wajib dihitung dari
/// selisih dua sampel, bukan satu.
///
/// Lihat `proc(5)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuSample {
    /// Waktu idle + iowait (keduanya dianggap "tidak produktif").
    pub idle: u64,
    /// Jumlah delapan field pertama: user, nice, system, idle, iowait, irq, softirq, steal.
    pub total: u64,
}

/// Penggunaan memori dari `/proc/meminfo`, dalam kB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemInfo {
    /// `MemTotal`.
    pub total_kb: u64,
    /// `MemAvailable` — memori yang benar-benar bisa dipakai aplikasi,
    /// memperhitungkan cache yang dapat dilepas (bukan sekadar `MemFree`).
    pub available_kb: u64,
}

/// Gabungan seluruh metrik pada satu waktu sampling.
#[derive(Debug, Clone)]
pub struct SystemSample {
    /// Sampel CPU saat sampling.
    pub cpu: CpuSample,
    /// Sampel memori saat sampling.
    pub mem: MemInfo,
    /// Nama model CPU dari `/proc/cpuinfo`, kosong bila tidak ditemukan.
    pub cpu_model: String,
    /// Versi kernel dari `/proc/sys/kernel/osrelease`.
    pub kernel: String,
    /// Uptime sistem dalam detik.
    pub uptime_secs: u64,
    /// Load average 1, 5, dan 15 menit.
    pub load: (f64, f64, f64),
    /// Jumlah folder PID di root `/proc`.
    pub proc_count: usize,
}

// === Parser /proc (fungsi murni) =================================

/// Mengambil sampel CPU dari isi `/proc/stat`.
///
/// Hanya baris agregat `cpu ` yang dipakai (baris `cpuN` per core diabaikan).
/// Rumus sesuai modul: `idle = idle + iowait`, `total` = jumlah delapan
/// field pertama.
///
/// # Errors
///
/// Mengembalikan `None` bila baris `cpu` tidak ada, atau jumlah field
/// kurang dari delapan, atau ada field yang gagal dikonversi ke `u64`.
pub fn parse_cpu_line(text: &str) -> Option<CpuSample> {
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("cpu ") else { continue };
        let fields: Vec<Option<u64>> = rest
            .split_whitespace()
            .map(|f| f.parse::<u64>().ok())
            .collect();
        if fields.len() < 8 || fields[..8].iter().any(|f| f.is_none()) {
            return None;
        }
        let f: Vec<u64> = fields[..8].iter().map(|v| v.unwrap()).collect();
        return Some(CpuSample {
            idle: f[3] + f[4],
            total: f.iter().sum(),
        });
    }
    None
}

/// Mengambil [`MemInfo`] dari isi `/proc/meminfo`.
///
/// Keduanya wajib ada — `MemAvailable` yang hilang dianggap data tidak valid
/// karena perhitungan persentase bergantung padanya.
///
/// # Errors
///
/// Mengembalikan `None` bila `MemTotal` atau `MemAvailable` tidak ditemukan.
pub fn parse_meminfo(text: &str) -> Option<MemInfo> {
    let mut total_kb = None;
    let mut available_kb = None;
    for line in text.lines() {
        let mut parts = line.split(':');
        let key = parts.next().unwrap_or("").trim();
        let val = parts
            .next()
            .unwrap_or("")
            .split_whitespace()
            .next()
            .and_then(|v| v.parse::<u64>().ok());
        match key {
            "MemTotal" => total_kb = val,
            "MemAvailable" => available_kb = val,
            _ => {}
        }
        if total_kb.is_some() && available_kb.is_some() {
            break;
        }
    }
    Some(MemInfo {
        total_kb: total_kb?,
        available_kb: available_kb?,
    })
}

/// Mengambil nama model CPU pertama dari isi `/proc/cpuinfo`.
///
/// # Errors
///
/// Mengembalikan `String` kosong bila tidak ada baris `model name`.
pub fn parse_cpuinfo_model(text: &str) -> String {
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("model name") {
            if let Some(val) = rest.split(':').nth(1) {
                return val.trim().to_string();
            }
        }
    }
    String::new()
}

/// Mengambil uptime dalam detik (dibulatkan) dari isi `/proc/uptime`.
///
/// Format file: `<uptime> <idle-time>`, keduanya bertipe f64.
///
/// # Errors
///
/// Mengembalikan `None` bila token pertama bukan angka.
pub fn parse_uptime(text: &str) -> Option<u64> {
    text.split_whitespace()
        .next()
        .and_then(|tok| tok.parse::<f64>().ok())
        .map(|f| f as u64)
}

/// Mengambil load average 1/5/15 menit dari isi `/proc/loadavg`.
///
/// Tiga token pertama adalah rata-rata jumlah proses yang siap jalan
/// atau sedang menunggu I/O.
///
/// # Errors
///
/// Mengembalikan `None` bila ada token yang bukan `f64`.
pub fn parse_loadavg(text: &str) -> Option<(f64, f64, f64)> {
    let mut it = text.split_whitespace();
    let a = it.next()?.parse().ok()?;
    let b = it.next()?.parse().ok()?;
    let c = it.next()?.parse().ok()?;
    Some((a, b, c))
}

/// Menghitung jumlah entri folder dengan nama numerik (folder PID) di `dir`.
///
/// Sesuai `proc(5)`: setiap proses memiliki satu folder bernama PID.
///
/// # Errors
///
/// Mengembalikan `0` bila `dir` tidak bisa dibaca — jumlah proses
/// dianggap tidak tersedia, program tidak boleh panic.
pub fn count_pids(dir: &Path) -> usize {
    match fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .map(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()))
                    .unwrap_or(false)
            })
            .count(),
        Err(_) => 0,
    }
}

// === Perhitungan (fungsi murni) ==================================

/// Penggunaan CPU (%) dari dua sampel: `100 × (1 − Δidle/Δtotal)`.
///
/// [`SystemSample`] berisi hitungan kumulatif sejak boot; penggunaan nyata
/// hanya muncul dari selisih dua sampel dalam rentang waktu.
///
/// # Errors
///
/// Mengembalikan `0.0` bila Δtotal ≤ 0 (data tidak bergerak atau tidak valid).
pub fn cpu_usage_percent(prev: CpuSample, cur: CpuSample) -> f64 {
    let d_idle = cur.idle as f64 - prev.idle as f64;
    let d_total = cur.total as f64 - prev.total as f64;
    if d_total <= 0.0 {
        return 0.0;
    }
    (100.0 * (1.0 - d_idle / d_total)).clamp(0.0, 100.0)
}

/// Persentase memori terpakai: `(total − available) / total × 100`.
///
/// # Errors
///
/// Mengembalikan `0.0` bila `total_kb == 0` (menghindari pembagian nol).
pub fn mem_usage_percent(mem: MemInfo) -> f64 {
    if mem.total_kb == 0 {
        return 0.0;
    }
    let used = mem.total_kb.saturating_sub(mem.available_kb);
    (used as f64 / mem.total_kb as f64 * 100.0).clamp(0.0, 100.0)
}

/// Memformat uptime sebagai `Hh Mm Ss`, atau `Hh Mm` bila sudah melewati
/// lebih dari sehari.
pub fn format_uptime(secs: u64) -> String {
    let d = secs / 86_400;
    let h = (secs % 86_400) / 3_600;
    let m = (secs % 3_600) / 60;
    let s = secs % 60;
    if d > 0 {
        format!("{d}d {h}h {m}m")
    } else if h > 0 {
        format!("{h}h {m}m {s}s")
    } else {
        format!("{m}m {s}s")
    }
}

/// Membentuk progress bar lebar tetap: `█` terisi dan `░` kosong,
/// diakhiri persentase satu desimal. Nilai di-clamp ke 0..100.
pub fn render_bar(percent: f64, width: usize) -> String {
    let filled = ((percent.clamp(0.0, 100.0) / 100.0) * width as f64).round() as usize;
    let filled = filled.min(width);
    format!(
        "{}{}{:.1}",
        "█".repeat(filled),
        "░".repeat(width - filled),
        percent.clamp(0.0, 100.0)
    )
}

// === Sampler =====================================================

/// Menyimpan sampel CPU sebelumnya dan menghitung penggunaan dari delta.
///
/// Sampel pertama hanya menjadi baseline — belum menghasilkan angka.
pub struct CpuSampler {
    prev: Option<CpuSample>,
}

impl CpuSampler {
    /// Membuat sampler tanpa baseline.
    #[must_use]
    pub fn new() -> Self {
        Self { prev: None }
    }

    /// Memasukkan sampel baru; mengembalikan penggunaan CPU antar sampel.
    ///
    /// # Errors
    ///
    /// Mengembalikan `None` untuk pemanggilan pertama (baseline belum ada).
    pub fn usage(&mut self, cur: CpuSample) -> Option<f64> {
        let pct = self.prev.map(|p| cpu_usage_percent(p, cur));
        self.prev = Some(cur);
        pct
    }
}

/// Akses filesystem `/proc` yang parametrik.
///
/// [`ProcFS::resolve`] memilih sumber data:
///
/// - env `SPIDEROS_PROC_ROOT` terisi → dipakai sebagai root (preview
///   dengan fixture di Windows).
/// - kosong / tidak ada → `/proc` asli (runtime Linux/VM).
///
/// Dengan demikian tidak ada `#[cfg(windows)]` di seluruh kode — jalur
/// preview tetap satu kode dengan runtime produksi.
///
/// # Fitur preview: alternasi `stat.b`
///
/// Bila file `stat.b` ada di root, mode watch membaca `stat` dan `stat.b`
/// bergantian sehingga delta CPU bergerak secara deterministik — memungkinkan
/// simulasi beban di Windows. Di `/proc` asli file ini tidak pernah ada,
/// sehingga runtime produksi selalu membaca `stat` tunggal.
pub struct ProcFS {
    root: PathBuf,
    /// Status giliran alternasi `stat`/`stat.b` (khusus fixture preview).
    stat_alt: Cell<bool>,
}

impl ProcFS {
    /// Menentukan root data sesuai env `SPIDEROS_PROC_ROOT` atau default `/proc`.
    #[must_use]
    pub fn resolve() -> Self {
        match env::var("SPIDEROS_PROC_ROOT") {
            Ok(p) if !p.is_empty() => Self { root: PathBuf::from(p), stat_alt: Cell::new(false) },
            _ => Self { root: PathBuf::from("/proc"), stat_alt: Cell::new(false) },
        }
    }

    /// Root yang sedang dipakai, untuk pesan error dan diagnostic.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Membaca satu file di dalam root.
    fn read(&self, name: &str) -> io::Result<String> {
        fs::read_to_string(self.root.join(name))
    }

    /// Membaca sumber `/proc/stat` dengan alternasi `stat.b` bila tersedia.
    fn read_stat(&self) -> io::Result<String> {
        if self.root.join("stat.b").exists() {
            let use_alt = !self.stat_alt.get();
            self.stat_alt.set(!use_alt);
            return self.read(if use_alt { "stat.b" } else { "stat" });
        }
        self.read("stat")
    }

    /// Mengumpulkan satu [`SystemSample`] dari seluruh sumber `/proc`.
    ///
    /// Versi kernel dicoba dari `sys/kernel/osrelease` terlebih dahulu,
    /// lalu fallback ke file `osrelease` di root (format fixture).
    ///
    /// # Errors
    ///
    /// Mengembalikan `Err` hanya bila file inti (`stat`, `meminfo`,
    /// `cpuinfo`, `uptime`, `loadavg`) gagal dibaca; field pelengkap
    /// (osrelease, jumlah proses) ditoleransi kosong agar tidak panic.
    pub fn sample(&self) -> io::Result<SystemSample> {
        let stat = self.read_stat()?;
        let meminfo = self.read("meminfo")?;
        let cpuinfo = self.read("cpuinfo")?;
        let uptime = self.read("uptime")?;
        let loadavg = self.read("loadavg")?;

        let kernel = self
            .read("sys/kernel/osrelease")            .or_else(|_| self.read("osrelease"))
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        Ok(SystemSample {
            cpu: parse_cpu_line(&stat).unwrap_or(CpuSample { idle: 0, total: 0 }),
            mem: parse_meminfo(&meminfo).unwrap_or(MemInfo { total_kb: 0, available_kb: 0 }),
            cpu_model: parse_cpuinfo_model(&cpuinfo),
            kernel,
            uptime_secs: parse_uptime(&uptime).unwrap_or(0),
            load: parse_loadavg(&loadavg).unwrap_or((0.0, 0.0, 0.0)),
            proc_count: count_pids(&self.root),
        })
    }
}

// === Renderer ANSI ===============================================

const RED: &str = "\x1b[1;31m";
const RESET: &str = "\x1b[0m";
const BAR_WIDTH: usize = 20;

/// Palet lembut Spider-Sense — satu keluarga warna hangat, tanpa
/// kombinasi merah-biru yang bentrok. Digunakan di seluruh renderer.
const MAROON: &str = "\x1b[38;5;88m"; // maroon — identitas utama
const ROSE: &str = "\x1b[38;5;174m"; // rose soft — aksen sekunder
const CREAM: &str = "\x1b[38;5;223m"; // cream — nilai/teks hangat
const SAND: &str = "\x1b[38;5;180m"; // sand — label
const DUSK: &str = "\x1b[38;5;138m"; // dusk muted — underline/pemisah

/// Menyiapkan terminal Windows agar memproses escape sequence ANSI.
///
/// Console lama Windows (conhost) menonaktifkan virtual terminal
/// processing secara default, sehingga `ESC[2J`/`ESC[H` tampil apa
/// adanya dan frame menumpuk. Fungsi ini mengaktifkan
/// `ENABLE_VIRTUAL_TERMINAL_PROCESSING` lewat deklarasi FFI `kernel32`
/// langsung — tetap std-only, tanpa crate.
///
/// Di platform non-Windows ini adalah no-op karena terminal Unix
/// memproses ANSI secara native.
#[cfg(windows)]
fn enable_virtual_terminal() {
    const STD_OUTPUT_HANDLE: i32 = -11;
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetStdHandle(n_std_handle: i32) -> *mut core::ffi::c_void;
        fn GetConsoleMode(console: *mut core::ffi::c_void, mode: *mut u32) -> i32;
        fn SetConsoleMode(console: *mut core::ffi::c_void, mode: u32) -> i32;
    }

    unsafe {
        let console = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut mode: u32 = 0;
        if GetConsoleMode(console, &mut mode) != 0 {
            SetConsoleMode(console, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

/// No-op di platform non-Windows (lihat versi `#[cfg(windows)]`).
#[cfg(not(windows))]
fn enable_virtual_terminal() {}

/// Merender satu frame ke teks ANSI.
///
/// `colored == false` (`--no-color`) mematikan warna saja; pembersihan
/// layar pada mode watch tetap aktif sesuai spesifikasi.
pub struct Renderer {
    colored: bool,
}

impl Renderer {
    /// Membuat renderer; `colored` mengikuti opsi `--no-color`.
    #[must_use]
    pub fn new(colored: bool) -> Self {
        Self { colored }
    }

    /// Kode warna bila warna aktif, string kosong bila tidak.
    fn color<'a>(&self, code: &'a str) -> &'a str {
        if self.colored {
            code
        } else {
            ""
        }
    }

    /// Bar 20 kolom dengan persentase satu desimal.
    fn bar(&self, percent: f64) -> String {
        render_bar(percent, BAR_WIDTH)
    }

/// Word art "SPIDEROS" — hasil generator ASCII art, identitas visual
/// Spider-Sense di kolom kiri.
const SPIDER_ART: [&str; 11] = [
    r"$$$$$$\            $$\       $$\                     ",
    r"$$  __$$\           \__|      $$ |                    ",
    r"$$ /  \__| $$$$$$\  $$\  $$$$$$$ | $$$$$$\   $$$$$$\  ",
    r"\$$$$$$\  $$  __$$\ $$ |$$  __$$ |$$  __$$\ $$  __$$\ ",
    r" \____$$\ $$ /  $$ |$$ |$$ /  $$ |$$$$$$$$ |$$ |  \__|",
    r"$$\   $$ |$$ |  $$ |$$ |$$ |  $$ |$$   ____|$$ |      ",
    r"\$$$$$$  |$$$$$$$  |$$ |\$$$$$$$ |\$$$$$$$\ $$ |      ",
    r" \______/ $$  ____/ \__| \_______| \_______|\__|      ",
    r"          $$ |                                        ",
    r"          $$ |                                        ",
    r"          \__| ",
];

/// Panjang teks yang terlihat — mengabaikan escape sequence ANSI.
///
/// Padding kolom `format!` menghitung semua karakter termasuk kode
/// warna yang invisible, sehingga menyebabkan kolom info molor.
/// Fungsi ini menghitung hanya karakter yang benar-benar tampil.
fn visible_width(text: &str) -> usize {
    let mut count = 0;
    let mut in_escape = false;
    for c in text.chars() {
        if in_escape {
            if c.is_ascii_alphabetic() {
                in_escape = false; // terminator sequence (mis. 'm' pada kode warna)
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            count += 1;
        }
    }
    count
}

    /// Menyusun satu frame ala neofetch: word art di kiri, info sistem di
    /// kanan; baris info yang melebihi art dilanjutkan dengan kolom kiri
    /// kosong agar kolom kanan tetap sejajar.
    ///
    /// Tiap baris diakhiri `ESC[K` (hapus ke kanan) dan frame ditutup
    /// `ESC[J` (hapus ke bawah) — kombinasi ini membuat mode watch bisa
    /// menimpa frame sebelumnya tanpa clear penuh, sehingga tidak kedip.
    pub fn frame(&self, cpu_pct: f64, mem_pct: f64, s: &SystemSample) -> String {
        let mut out = String::new();
        let info = self.info_lines(cpu_pct, mem_pct, s);
        let mut info_iter = info.iter();

        let art_len = Self::SPIDER_ART.len();
        let rows = art_len.max(info.len());
        for i in 0..rows {
            let left = if i < art_len {
                format!("{}{}", self.color(MAROON), Self::SPIDER_ART[i])
            } else {
                String::new()
            };
            let pad = 55usize.saturating_sub(Self::visible_width(&left));
            let line = match info_iter.next() {
                Some(line) => line.clone(),
                None => String::new(),
            };
            out.push_str(&format!("  {}{}{}{}\x1b[K\n", left, " ".repeat(pad), line, ""));
        }
        out.push_str("\x1b[J");
        out
    }

/// Susun baris-baris info ala neofetch (label + nilai, sejajar titik-dua).
fn info_lines(&self, cpu_pct: f64, mem_pct: f64, s: &SystemSample) -> Vec<String> {
    let (c_maroon, c_sand, c_cream, c_dusk, c_reset) = (
        self.color(MAROON),
        self.color(SAND),
        self.color(CREAM),
        self.color(DUSK),
        self.color(RESET),
    );
    let label = |name: &str| format!(" {c_sand}{name:<8}{c_reset}: ");
    let user = env_user();
    let host = env_host();

    let mut v = Vec::new();
    v.push(format!(" {c_maroon}{user}{c_reset}{c_dusk}@{c_reset}{c_sand}{host}{c_reset}"));
    v.push(format!(" {c_dusk}----------------{c_reset}"));
    v.push(format!("{}{c_cream}SpiderOS 1.0 (Debian 13){c_reset}", label("OS")));
    v.push(format!("{}{c_cream}{host}{c_reset}", label("Host")));
    v.push(format!("{}{c_cream}{}{c_reset}", label("Kernel"), if s.kernel.is_empty() { "N/A" } else { &s.kernel }));
    v.push(format!("{}{c_cream}{}{c_reset}", label("Uptime"), format_uptime(s.uptime_secs)));
    v.push(format!("{}{c_cream}{}{c_reset}", label("Procs"), s.proc_count));
    v.push(format!("{}{c_cream}{:.2}, {:.2}, {:.2}{c_reset}", label("Load"), s.load.0, s.load.1, s.load.2));
    v.push(format!("{}{c_cream}{}{c_reset}", label("Shell"), env_shell()));
    v.push(format!("{}{c_cream}{}{c_reset}", label("Terminal"), env_terminal()));
    v.push(format!(" {c_sand}{:<8}{c_reset}[{c_cream}{}{c_reset}] {:.1}%", "CPU", self.bar(cpu_pct), cpu_pct));
    v.push(format!(" {c_sand}{:<8}{c_reset}[{c_cream}{}{c_reset}] {:.1}%", "RAM", self.bar(mem_pct), mem_pct));
    v.push(self.status_line(cpu_pct, mem_pct));
    v.push(String::new());
    v.push(self.color_blocks());
    v
}

/// Baris status berdasarkan metrik tertinggi (CPU atau RAM):
/// `< 40` Tenang, `40..=75` Waspada, `> 75` Bahaya.
fn status_line(&self, cpu_pct: f64, mem_pct: f64) -> String {
    let c_bold = self.color("\x1b[1m");
    let c_reset = self.color(RESET);
    let peak = cpu_pct.max(mem_pct);
    // Status tetap kontras tapi lembut: sage untuk tenang, amber untuk
    // waspada, merah terang hanya untuk kondisi bahaya.
    const SAGE: &str = "\x1b[38;5;108m";
    const AMBER: &str = "\x1b[38;5;179m";
    let (color, text) = if peak > 75.0 {
        (self.color(RED), "BAHAYA! Spider-Sense berdenyut!")
    } else if peak >= 40.0 {
        (self.color(AMBER), "Waspada")
    } else {
        (self.color(SAGE), "Tenang")
    };
    format!(" {c_bold}Status{c_reset}  : {color}{text}{RESET}")
}

/// Blok warna palet lembut ala neofetch: maroon, rose, cream.
fn color_blocks(&self) -> String {
    if !self.colored {
        return "███ ███ ███ ███ ███ ███".to_string();
    }
    let maroon = "███ ".repeat(3).trim_end().to_string();
    let rose = "███ ".repeat(3).trim_end().to_string();
    let cream = "███ ".repeat(3).trim_end().to_string();
    format!(
        "{MAROON}{maroon}{RESET}  {ROSE}{rose}{RESET}  {CREAM}{cream}{RESET}"
    )
}

    /// Inisialisasi layar (dipanggil SEKALI di awal watch): clear penuh +
    /// sembunyikan kursor. Frame berikutnya tidak perlu clear lagi.
    pub fn init_screen(&self) {
        print!("\x1b[2J\x1b[?25l");
        let _ = io::stdout().flush();
    }

    /// Pindahkan kursor ke home tanpa menghapus layar — dipakai tiap
    /// frame agar tidak ada momen layar kosong (anti-kedip).
    pub fn home_cursor(&self) {
        print!("\x1b[H");
    }

    /// Masuk alternate screen buffer (`ESC[?1049h`) pada mode watch.
    ///
    /// Tanpa ini, jejak frame lama tersimpan di scrollback terminal
    /// dan muncul saat di-scroll. Dengan alternate screen — mekanisme
    /// yang sama dipakai `htop` dan `vim` — frame hanya hidup di buffer
    /// terpisah, dan layar asli terpulihkan penuh saat program keluar.
    pub fn enter_alt_screen(&self) {
        print!("\x1b[?1049h");
        let _ = io::stdout().flush();
    }

    /// Keluar dari alternate screen dan pulihkan terminal.
    pub fn leave_alt_screen(&self) {
        print!("\x1b[?1049l");
    }
}

/// Menampilkan kembali kursor sebelum program keluar (`ESC[?25h`).
pub fn show_cursor() {
    print!("\x1b[?25h");
    let _ = io::stdout().flush();
}

// === CLI =========================================================

/// Nama user dari environment (POSIX: `USER`, fallback Windows: `USERNAME`).
fn env_user() -> String {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "spider".into())
}

/// Nama host dari environment (POSIX: `HOSTNAME`, fallback Windows: `COMPUTERNAME`).
fn env_host() -> String {
    env::var("HOSTNAME")
        .or_else(|_| env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "spideros".into())
}

/// Nama shell dari environment (`SHELL` di POSIX), tanpa path.
fn env_shell() -> String {
    env::var("SHELL")
        .map(|p| {
            Path::new(&p)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&p)
                .to_string()
        })
        .unwrap_or_else(|_| "unknown".into())
}

/// Nama terminal dari environment (`TERM` di POSIX, `TERM_PROGRAM` di Windows Terminal).
fn env_terminal() -> String {
    env::var("TERM")
        .or_else(|_| env::var("TERM_PROGRAM"))
        .unwrap_or_else(|_| "unknown".into())
}

const VERSION_LINE: &str = "spider-sense 1.0.0 (Rust, std-only)";
const HELP_TEXT: &str = "\
Spider-Sense — SpiderOS system monitor (membaca /proc secara langsung)

Pemakaian:
  spider-sense [OPSI]

Opsi:
  --watch       Mode real-time: sampling tiap 1 detik (default)
  --once        Render satu frame lalu keluar (smoke test)
  --no-color    Matikan warna ANSI (clear screen di watch tetap aktif)
  --help        Tampilkan bantuan ini
  --version     Tampilkan versi

Lingkungan:
  SPIDEROS_PROC_ROOT  Path alternatif pengganti /proc
                      (dipakai untuk preview dengan data fixture di Windows)
";

/// Mencetak teks bantuan `--help`.
fn print_help() {
    println!("{}", HELP_TEXT.trim_end());
}

/// Mencetak baris versi `--version` (format wajib modul 9.7).
fn print_version() {
    println!("{VERSION_LINE}");
}

/// Verifikasi root data tersedia sebelum masuk loop; tanpa ini pengguna
/// di Windows hanya melihat "os error 3" tanpa petunjuk.
///
/// # Errors
///
/// Mengembalikan `Err` bila root tidak ada, dengan pesan berisi
/// instruksi env `SPIDEROS_PROC_ROOT`.
fn validate_root(procfs: &ProcFS) -> io::Result<()> {
    let root = procfs.root();
    if !root.is_dir() {
        return Err(io::Error::other(format!(
            "sumber data '{}' tidak tersedia.\n\
             Di Windows, arahkan ke folder fixture:\n      \
             $env:SPIDEROS_PROC_ROOT = \"tests\\fixtures\\proc\"\n\
             Di Linux/VM, biarkan default (/proc).",
            root.display()
        )));
    }
    Ok(())
}

/// Mode `--once`: sampel awal → tunggu ~300 ms → sampel kedua → satu frame → keluar.
///
/// # Errors
///
/// Meneruskan `Err` dari [`ProcFS::sample`] dan kegagalan flush stdout.
fn run_once(colored: bool) -> io::Result<()> {
    let procfs = ProcFS::resolve();
    validate_root(&procfs)?;
    let mut sampler = CpuSampler::new();
    let s1 = procfs.sample()?;
    sampler.usage(s1.cpu);
    thread::sleep(Duration::from_millis(300));
    let s2 = procfs.sample()?;
    let cpu_pct = sampler.usage(s2.cpu).unwrap_or(0.0);
    let mem_pct = mem_usage_percent(s2.mem);
    print!("{}", Renderer::new(colored).frame(cpu_pct, mem_pct, &s2));
    io::stdout().flush()
}

/// Mode `--watch` (default): loop sampling tiap 1 detik, clear screen, render ulang.
///
/// # Errors
///
/// Meneruskan `Err` dari [`ProcFS::sample`] dan flush stdout; kegagalan
/// sampling menampilkan pesan ke stderr lalu loop berlanjut.
fn run_watch(colored: bool) -> io::Result<()> {
    let procfs = ProcFS::resolve();
    validate_root(&procfs)?;
    let mut sampler = CpuSampler::new();
    let renderer = Renderer::new(colored);
    sampler.usage(procfs.sample()?.cpu);

    // Alternate screen: frame hanya hidup di buffer terpisah, scrollback
    // terminal tidak tercemar; layar asli terpulihkan saat keluar.
    renderer.enter_alt_screen();
    let result = (|| -> io::Result<()> {
        // Clear penuh hanya di awal; frame berikutnya menimpa via
        // home + ESC[K per baris sehingga tidak ada flash.
        renderer.init_screen();
        let s_first = procfs.sample()?;
        loop {
            let s = if sampler.prev.is_some() {
                match procfs.sample() {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!(
                            "spider-sense: gagal membaca {}: {e}",
                            procfs.root().display()
                        );
                        thread::sleep(Duration::from_secs(1));
                        continue;
                    }
                }
            } else {
                s_first.clone()
            };
            let cpu_pct = sampler.usage(s.cpu).unwrap_or(0.0);
            let mem_pct = mem_usage_percent(s.mem);
            renderer.home_cursor();
            print!("{}", renderer.frame(cpu_pct, mem_pct, &s));
            io::stdout().flush()?;
            thread::sleep(Duration::from_secs(1));
        }
    })();
    renderer.leave_alt_screen();
    result
}

/// Entry point: parsing argumen manual, lalu menjalankan mode `--watch`
/// (default) atau `--once`.
///
/// # Panics
///
/// Tidak ada — kesalahan runtime ditangani dengan kode keluar 1/2.
fn main() {
    enable_virtual_terminal();

    let mut colored = true;
    let mut watch = true;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--once" => watch = false,
            "--watch" => watch = true,
            "--no-color" => colored = false,
            "--help" => return print_help(),
            "--version" => return print_version(),
            other => {
                eprintln!("spider-sense: opsi tidak dikenal: {other}");
                eprintln!("Coba: spider-sense --help");
                std::process::exit(2);
            }
        }
    }

    let result = if watch { run_watch(colored) } else { run_once(colored) };
    show_cursor();
    if let Err(e) = result {
        eprintln!("spider-sense: {e}");
        std::process::exit(1);
    }
}

// === Unit test ===================================================

#[cfg(test)]
mod tests {
    use super::*;

    const STAT_FIXTURE: &str = "\
cpu  74609 1234 24531 512340 8123 0 4210 0 0 0
cpu0 18652 310 6133 128085 2031 0 1053 0 0 0
intr 1234567
";

    #[test]
    fn parse_cpu_line_menggabungkan_idle_dan_iowait() {
        let s = parse_cpu_line(STAT_FIXTURE).expect("baris cpu harus ter-parse");
        assert_eq!(s.idle, 512_340 + 8_123);
        assert_eq!(s.total, 74609 + 1234 + 24531 + 512340 + 8123 + 0 + 4210 + 0);
        assert!(s.total > s.idle);
    }

    #[test]
    fn parse_cpu_line_menolak_field_tidak_lengkap() {
        assert!(parse_cpu_line("cpu 1 2 3").is_none());
        assert!(parse_cpu_line("intr 123\n").is_none());
    }

    #[test]
    fn parse_meminfo_mengambil_total_dan_available() {
        let text = "MemTotal: 16322000 kB\nMemFree: 4120500 kB\nMemAvailable: 9254300 kB\n";
        let m = parse_meminfo(text).expect("meminfo harus ter-parse");
        assert_eq!(m.total_kb, 16_322_000);
        assert_eq!(m.available_kb, 9_254_300);
        assert!(parse_meminfo("MemFree: 1 kB\n").is_none());
    }

    #[test]
    fn mem_usage_percent_menghitung_dan_menghindari_pembagian_nol() {
        let m = MemInfo { total_kb: 10_000, available_kb: 2_500 };
        assert!((mem_usage_percent(m) - 75.0).abs() < 1e-9);
        assert_eq!(mem_usage_percent(MemInfo { total_kb: 0, available_kb: 0 }), 0.0);
    }

    #[test]
    fn cpu_usage_percent_menghitung_delta_dua_sampel() {
        let prev = CpuSample { idle: 900, total: 1_000 };
        let cur = CpuSample { idle: 950, total: 1_100 };
        assert!((cpu_usage_percent(prev, cur) - 50.0).abs() < 1e-9);
        assert_eq!(cpu_usage_percent(prev, prev), 0.0);
    }

    #[test]
    fn render_bar_mengisi_sesuai_proporsi_dengan_clamp() {
        assert_eq!(render_bar(50.0, 10), "█████░░░░░50.0");
        assert_eq!(render_bar(150.0, 4), "████100.0");
        assert_eq!(render_bar(-10.0, 4), "░░░░0.0");
    }

    #[test]
    fn format_uptime_memilih_satuan_terbesar() {
        assert_eq!(format_uptime(12_312), "3h 25m 12s");
        assert_eq!(format_uptime(65), "1m 5s");
        assert_eq!(format_uptime(190_000), "2d 4h 46m");
    }

    #[test]
    fn count_pids_menghitung_folder_angka_di_fixture() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/proc");
        assert_eq!(count_pids(&dir), 3);
    }

    #[test]
    fn parse_loadavg_mengambil_tiga_angka_pertama() {
        let (a, b, c) = parse_loadavg("0.52 0.58 0.59 2/412 3512").unwrap();
        assert_eq!(a, 0.52);
        assert_eq!(b, 0.58);
        assert_eq!(c, 0.59);
        assert!(parse_loadavg("bukan angka").is_none());
    }

    #[test]
    fn parse_cpuinfo_model_mengambil_nilai_setelah_titik_dua() {
        let model = parse_cpuinfo_model("processor\t: 0\nmodel name\t: Intel(R) Core(TM) i7\n");
        assert_eq!(model, "Intel(R) Core(TM) i7");
    }

    #[test]
    fn cpu_sampler_kembalikan_none_di_baseline_lalu_delta() {
        let mut sampler = CpuSampler::new();
        assert_eq!(sampler.usage(CpuSample { idle: 900, total: 1_000 }), None);
        let pct = sampler.usage(CpuSample { idle: 950, total: 1_100 }).unwrap();
        assert!((pct - 50.0).abs() < 1e-9);
    }

    #[test]
    fn parse_uptime_membulatkan_ke_detik() {
        assert_eq!(parse_uptime("12753.42 98213.07"), Some(12_753));
        assert!(parse_uptime("bukan").is_none());
    }

    #[test]
    fn visible_width_mengabaikan_kode_warna_ansi() {
        assert_eq!(Renderer::visible_width("abc"), 3);
        assert_eq!(Renderer::visible_width("\x1b[1;31mabc\x1b[0m"), 3);
        assert_eq!(Renderer::visible_width("\x1b[38;5;88m█\x1b[0m"), 1);
        assert_eq!(Renderer::visible_width(""), 0);
    }
}
