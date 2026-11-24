//! Antarmuka CLI: identitas program, teks bantuan, validasi sumber data.

use std::env;
use std::io;
use std::path::Path;

use crate::procfs::ProcFS;

pub(crate) fn env_user() -> String {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "spider".into())
}

pub(crate) fn env_host() -> String {
    env::var("HOSTNAME")
        .or_else(|_| env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "spideros".into())
}

pub(crate) fn env_shell() -> String {
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

pub(crate) fn env_terminal() -> String {
    env::var("TERM")
        .or_else(|_| env::var("TERM_PROGRAM"))
        .unwrap_or_else(|_| "unknown".into())
}

/// `neofetch` memanggil binary ini lewat symlink. Nama pemanggil yang
/// menentukan mode default, bukan hardcode di `main`.
fn is_neofetch_invocation(arg0: &str) -> bool {
    Path::new(arg0)
        .file_name()
        .and_then(|name| name.to_str())
        == Some("neofetch")
}

/// Mode awal: `neofetch` → satu frame (prompt balik), lainnya → live.
pub(crate) fn default_watch() -> bool {
    !is_neofetch_invocation(&env::args().next().unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::is_neofetch_invocation;

    #[test]
    fn neofetch_name_is_snapshot() {
        assert!(is_neofetch_invocation("neofetch"));
        assert!(is_neofetch_invocation("/usr/local/bin/neofetch"));
    }

    #[test]
    fn other_names_are_live() {
        assert!(!is_neofetch_invocation("spider-sense"));
        assert!(!is_neofetch_invocation("/usr/local/bin/spider-sense"));
        assert!(!is_neofetch_invocation(""));
    }
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

pub(crate) fn print_help() {
    println!("{}", HELP_TEXT.trim_end());
}

pub(crate) fn print_version() {
    println!("{VERSION_LINE}");
}

/// `Err` bila root data tidak ada, dengan instruksi env `SPIDEROS_PROC_ROOT`.
pub(crate) fn validate_root(procfs: &ProcFS) -> io::Result<()> {
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
