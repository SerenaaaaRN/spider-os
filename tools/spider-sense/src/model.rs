//! Struktur data hasil parsing `/proc`.

/// Sampel agregasi baris `cpu` dari `/proc/stat` (hitungan kumulatif sejak boot).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuSample {
    pub idle: u64,
    pub total: u64,
}

/// `MemTotal` dan `MemAvailable` dari `/proc/meminfo`, dalam kB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemInfo {
    pub total_kb: u64,
    pub available_kb: u64,
}

/// Gabungan seluruh metrik pada satu waktu sampling.
#[derive(Debug, Clone)]
pub struct SystemSample {
    pub cpu: CpuSample,
    pub mem: MemInfo,

    #[expect(dead_code)]
    pub cpu_model: String,
    pub kernel: String,
    pub uptime_secs: u64,
    pub load: (f64, f64, f64),
    pub proc_count: usize,
}
