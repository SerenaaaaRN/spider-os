//! Collector — satu-satunya modul yang membaca filesystem, parametrik root.

use std::cell::Cell;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::model::{CpuSample, MemInfo, SystemSample};
use crate::parser::{count_pids, parse_cpu_line, parse_cpuinfo_model, parse_loadavg, parse_meminfo, parse_uptime};

/// Akses `/proc` parametrik: env `SPIDEROS_PROC_ROOT` dipakai sebagai root
/// alternatif (preview fixture di Windows), selain itu `/proc` asli.
///
/// Bila file `stat.b` ada di root, `stat` dan `stat.b` dibaca bergantian
/// agar delta CPU bergerak di preview — di `/proc` asli file ini tidak ada.
pub struct ProcFS {
    root: PathBuf,
    stat_alt: Cell<bool>,
}

impl ProcFS {
    /// Root dari env `SPIDEROS_PROC_ROOT` atau default `/proc`.
    #[must_use]
    pub fn resolve() -> Self {
        match env::var("SPIDEROS_PROC_ROOT") {
            Ok(p) if !p.is_empty() => {
                Self { root: PathBuf::from(p), stat_alt: Cell::new(false) }
            }
            _ => Self { root: PathBuf::from("/proc"), stat_alt: Cell::new(false) },
        }
    }

    /// Root yang sedang dipakai.
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    fn read(&self, name: &str) -> io::Result<String> {
        fs::read_to_string(self.root.join(name))
    }

    fn read_stat(&self) -> io::Result<String> {
        if self.root.join("stat.b").exists() {
            let use_alt = !self.stat_alt.get();
            self.stat_alt.set(!use_alt);
            return self.read(if use_alt { "stat.b" } else { "stat" });
        }
        self.read("stat")
    }

    /// Satu [`SystemSample`] dari seluruh sumber; `Err` hanya bila file
    /// inti gagal dibaca (field pelengkap ditoleransi kosong).
    pub fn sample(&self) -> io::Result<SystemSample> {
        let stat = self.read_stat()?;
        let meminfo = self.read("meminfo")?;
        let cpuinfo = self.read("cpuinfo")?;
        let uptime = self.read("uptime")?;
        let loadavg = self.read("loadavg")?;

        let kernel = self
            .read("sys/kernel/osrelease")
            .or_else(|_| self.read("osrelease"))
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
