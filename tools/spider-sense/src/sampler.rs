//! Sampler CPU — baseline + delta dua sampel.

use crate::metrics::cpu_usage_percent;
use crate::model::CpuSample;

/// Menyimpan sampel sebelumnya; sampel pertama hanya baseline (`None`).
pub struct CpuSampler {
    prev: Option<CpuSample>,
}

impl CpuSampler {
    #[must_use]
    pub fn new() -> Self {
        Self { prev: None }
    }

    /// Penggunaan CPU antar sampel; `None` pada pemanggilan pertama.
    pub fn usage(&mut self, cur: CpuSample) -> Option<f64> {
        let pct = self.prev.map(|p| cpu_usage_percent(p, cur));
        self.prev = Some(cur);
        pct
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_sampler_kembalikan_none_di_baseline_lalu_delta() {
        let mut sampler = CpuSampler::new();
        assert_eq!(sampler.usage(CpuSample { idle: 900, total: 1_000 }), None);
        let pct = sampler.usage(CpuSample { idle: 950, total: 1_100 }).unwrap();
        assert!((pct - 50.0).abs() < 1e-9);
    }
}
