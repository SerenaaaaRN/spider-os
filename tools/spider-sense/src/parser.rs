//! Parser `/proc` — fungsi murni `teks → data`, tidak membaca file langsung.

use std::fs;
use std::path::Path;

use crate::model::{CpuSample, MemInfo};

/// Sampel CPU dari baris `cpu ` (agregat, bukan per-core).
/// `None` bila field kurang dari delapan atau gagal dikonversi ke `u64`.
pub fn parse_cpu_line(text: &str) -> Option<CpuSample> {
    for line in text.lines() {
        let Some(rest) = line.strip_prefix("cpu ") else { continue };
        // take(8): hanya delapan field pertama; satu field gagal → seluruhnya None.
        let fields = rest
            .split_whitespace()
            .take(8)
            .map(|f| f.parse::<u64>().ok())
            .collect::<Option<Vec<u64>>>()?;
        if fields.len() < 8 {
            return None;
        }
        return Some(CpuSample {
            idle: fields[3] + fields[4],
            total: fields.iter().sum(),
        });
    }
    None
}

/// `MemTotal` dan `MemAvailable`; keduanya wajib ada.
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

/// Nama model CPU pertama; `String` kosong bila tidak ditemukan.
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

/// Uptime dalam detik (dibulatkan).
pub fn parse_uptime(text: &str) -> Option<u64> {
    text.split_whitespace()
        .next()
        .and_then(|tok| tok.parse::<f64>().ok())
        .map(|f| f as u64)
}

/// Load average 1/5/15 menit.
pub fn parse_loadavg(text: &str) -> Option<(f64, f64, f64)> {
    let mut it = text.split_whitespace();
    let a = it.next()?.parse().ok()?;
    let b = it.next()?.parse().ok()?;
    let c = it.next()?.parse().ok()?;
    Some((a, b, c))
}

/// Jumlah folder bernama numerik (PID) di `dir`; `0` bila tidak bisa dibaca.
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
        let expected: u64 = [74609, 1234, 24531, 512340, 8123, 0, 4210, 0].iter().sum();
        assert_eq!(s.total, expected);
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
    fn parse_cpuinfo_model_mengambil_nilai_setelah_titik_dua() {
        let model = parse_cpuinfo_model("processor\t: 0\nmodel name\t: Intel(R) Core(TM) i7\n");
        assert_eq!(model, "Intel(R) Core(TM) i7");
    }

    #[test]
    fn parse_uptime_membulatkan_ke_detik() {
        assert_eq!(parse_uptime("12753.42 98213.07"), Some(12_753));
        assert!(parse_uptime("bukan").is_none());
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
    fn count_pids_menghitung_folder_angka_di_fixture() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/proc");
        assert_eq!(count_pids(&dir), 3);
    }
}
