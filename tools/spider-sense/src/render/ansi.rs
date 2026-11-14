//! Palet warna (256-color, keluarga hangat) dan escape sequence ANSI.

pub(crate) const RED: &str = "\x1b[1;31m";
pub(crate) const RESET: &str = "\x1b[0m";
pub(crate) const BAR_WIDTH: usize = 20;

/// Maroon — identitas utama.
pub(crate) const MAROON: &str = "\x1b[38;5;88m";
pub(crate) const ROSE: &str = "\x1b[38;5;174m";
pub(crate) const CREAM: &str = "\x1b[38;5;223m";
pub(crate) const SAND: &str = "\x1b[38;5;180m";
pub(crate) const DUSK: &str = "\x1b[38;5;138m";
