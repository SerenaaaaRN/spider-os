//! # Spider-Sense
//!
//! Monitor sistem CLI SpiderOS: membaca `/proc`, parsing manual,
//! rendering ANSI.
//!
//! Modul: [`model`] (data), [`parser`] (fungsi murni), [`procfs`]
//! (collector), [`metrics`] (rumus), [`sampler`] (delta CPU),
//! [`render`] (ANSI), [`cli`] (identitas program).
//!
//! Preview di Windows: `ProcFS::resolve` memakai env `SPIDEROS_PROC_ROOT`
//! sebagai pengganti `/proc`:
//!
//! ```text
//! $env:SPIDEROS_PROC_ROOT = "tests/fixtures/proc"
//! cargo run -- --watch
//! ```

mod cli;
mod metrics;
mod model;
mod parser;
mod procfs;
mod render;
mod sampler;

use std::env;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use cli::validate_root;
use metrics::mem_usage_percent;
use procfs::ProcFS;
use render::{Renderer, enable_virtual_terminal, show_cursor};
use sampler::CpuSampler;

/// `--once`: sampel awal → 300 ms → sampel kedua → satu frame → keluar.
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

/// `--watch` (default): loop tiap 1 detik dengan alternate screen dan
/// overwrite halus; kegagalan sampling dicetak ke stderr, loop lanjut
/// dengan sampel terakhir yang berhasil.
fn run_watch(colored: bool) -> io::Result<()> {
    let procfs = ProcFS::resolve();
    validate_root(&procfs)?;
    let mut sampler = CpuSampler::new();
    let renderer = Renderer::new(colored);

    sampler.usage(procfs.sample()?.cpu);
    let mut current = procfs.sample()?;

    renderer.enter_alt_screen();
    let result = (|| -> io::Result<()> {
        renderer.init_screen();
        loop {
            let cpu_pct = sampler.usage(current.cpu).unwrap_or(0.0);
            let mem_pct = mem_usage_percent(current.mem);
            renderer.home_cursor();
            print!("{}", renderer.frame(cpu_pct, mem_pct, &current));
            io::stdout().flush()?;
            thread::sleep(Duration::from_secs(1));

            match procfs.sample() {
                Ok(s) => current = s,
                Err(e) => eprintln!(
                    "spider-sense: gagal membaca {}: {e}",
                    procfs.root().display()
                ),
            }
        }
    })();
    renderer.leave_alt_screen();
    result
}

fn main() {
    enable_virtual_terminal();

    let mut colored = true;
    let mut watch = true;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--once" => watch = false,
            "--watch" => watch = true,
            "--no-color" => colored = false,
            "--help" => return cli::print_help(),
            "--version" => return cli::print_version(),
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
