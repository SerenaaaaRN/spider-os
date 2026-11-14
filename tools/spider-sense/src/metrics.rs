use crate::model::{CpuSample, MemInfo};

/// Penggunaan CPU (%) dari dua sampel: `100 × (1 − Δidle/Δtotal)`; `0.0` bila Δtotal ≤ 0.
pub fn cpu_usage_percent(prev: CpuSample, cur: CpuSample) -> f64 {
    let d_idle = cur.idle as f64 - prev.idle as f64;
    let d_total = cur.total as f64 - prev.total as f64;
    if d_total <= 0.0 {
        return 0.0;
    }
    (100.0 * (1.0 - d_idle / d_total)).clamp(0.0, 100.0)
}

/// Persentase memori terpakai; `0.0` bila total nol.
pub fn mem_usage_percent(mem: MemInfo) -> f64 {
    if mem.total_kb == 0 {
        return 0.0;
    }
    let used = mem.total_kb.saturating_sub(mem.available_kb);
    (used as f64 / mem.total_kb as f64 * 100.0).clamp(0.0, 100.0)
}

/// Uptime sebagai `Hh Mm Ss`, atau `Hh Mm` bila lebih dari sehari.
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

/// Progress bar `█`/`░` lebar tetap, diakhiri persentase satu desimal.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_usage_percent_menghitung_delta_dua_sampel() {
        let prev = CpuSample { idle: 900, total: 1_000 };
        let cur = CpuSample { idle: 950, total: 1_100 };
        assert!((cpu_usage_percent(prev, cur) - 50.0).abs() < 1e-9);
        assert_eq!(cpu_usage_percent(prev, prev), 0.0);
    }

    #[test]
    fn mem_usage_percent_menghitung_dan_menghindari_pembagian_nol() {
        let m = MemInfo { total_kb: 10_000, available_kb: 2_500 };
        assert!((mem_usage_percent(m) - 75.0).abs() < 1e-9);
        assert_eq!(mem_usage_percent(MemInfo { total_kb: 0, available_kb: 0 }), 0.0);
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
}
