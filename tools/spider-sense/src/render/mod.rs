//! Renderer ANSI — menyusun frame, baris info, dan kontrol layar.
//!
//! [`Renderer`] murni membentuk teks; pemanggil yang mengurus kapan
//! teks dicetak dan di-flush.

mod ansi;
mod art;
mod term;

pub(crate) use term::{enable_virtual_terminal, show_cursor};

use std::io::{self, Write};

use ansi::{BAR_WIDTH, CREAM, DUSK, MAROON, RED, RESET, ROSE, SAND};
use art::SPIDER_ART;
use crate::cli::{env_host, env_shell, env_terminal, env_user};
use crate::metrics::{format_uptime, render_bar};
use crate::model::SystemSample;

/// Merender satu frame ke teks ANSI; `colored == false` mematikan warna saja.
pub struct Renderer {
    colored: bool,
}

impl Renderer {
    #[must_use]
    pub fn new(colored: bool) -> Self {
        Self { colored }
    }

    fn color<'a>(&self, code: &'a str) -> &'a str {
        if self.colored {
            code
        } else {
            ""
        }
    }

    fn bar(&self, percent: f64) -> String {
        render_bar(percent, BAR_WIDTH)
    }

    /// Frame ala neofetch: word art kiri, info kanan. Tiap baris diakhiri
    /// `ESC[K` dan frame ditutup `ESC[J` agar watch bisa menimpa tanpa
    /// clear penuh (anti-kedip).
    pub fn frame(&self, cpu_pct: f64, mem_pct: f64, s: &SystemSample) -> String {
        let info = self.info_lines(cpu_pct, mem_pct, s);
        let arts = SPIDER_ART.iter().chain(std::iter::repeat(&""));
        let lines = info.iter().map(Some).chain(std::iter::repeat(None));
        let mut out = String::new();

        for (art, info_line) in arts.zip(lines).take(SPIDER_ART.len().max(info.len())) {
            let left = if art.is_empty() {
                String::new()
            } else {
                format!("{}{art}", self.color(MAROON))
            };
            let pad = 55usize.saturating_sub(Self::visible_width(&left));
            let line = info_line.map(String::as_str).unwrap_or("");
            out.push_str(&format!("  {left}{}{line}\x1b[K\n", " ".repeat(pad)));
        }
        out.push_str("\x1b[J");
        out
    }

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
        v.push(format!(
            "{}{c_cream}{}{c_reset}",
            label("Kernel"),
            if s.kernel.is_empty() { "N/A" } else { &s.kernel }
        ));
        v.push(format!("{}{c_cream}{}{c_reset}", label("Uptime"), format_uptime(s.uptime_secs)));
        v.push(format!("{}{c_cream}{}{c_reset}", label("Procs"), s.proc_count));
        v.push(format!(
            "{}{c_cream}{:.2}, {:.2}, {:.2}{c_reset}",
            label("Load"),
            s.load.0,
            s.load.1,
            s.load.2
        ));
        v.push(format!("{}{c_cream}{}{c_reset}", label("Shell"), env_shell()));
        v.push(format!("{}{c_cream}{}{c_reset}", label("Terminal"), env_terminal()));
        v.push(format!(
            " {c_sand}{:<8}{c_reset}[{c_cream}{}{c_reset}] {:.1}%",
            "CPU",
            self.bar(cpu_pct),
            cpu_pct
        ));
        v.push(format!(
            " {c_sand}{:<8}{c_reset}[{c_cream}{}{c_reset}] {:.1}%",
            "RAM",
            self.bar(mem_pct),
            mem_pct
        ));
        v.push(self.status_line(cpu_pct, mem_pct));
        v.push(String::new());
        v.push(self.color_blocks());
        v
    }

    /// Status dari metrik tertinggi: <40 Tenang, 40..=75 Waspada, >75 Bahaya.
    fn status_line(&self, cpu_pct: f64, mem_pct: f64) -> String {
        let c_bold = self.color("\x1b[1m");
        let c_reset = self.color(RESET);
        let peak = cpu_pct.max(mem_pct);
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

    fn color_blocks(&self) -> String {
        if !self.colored {
            return "███ ███ ███ ███ ███ ███".to_string();
        }
        let maroon = "███ ".repeat(3).trim_end().to_string();
        let rose = "███ ".repeat(3).trim_end().to_string();
        let cream = "███ ".repeat(3).trim_end().to_string();
        format!("{MAROON}{maroon}{RESET}  {ROSE}{rose}{RESET}  {CREAM}{cream}{RESET}")
    }

    /// Lebar teks yang terlihat — mengabaikan escape sequence ANSI
    /// agar padding kolom info tidak molor.
    pub(crate) fn visible_width(text: &str) -> usize {
        let mut count = 0;
        let mut in_escape = false;
        for c in text.chars() {
            if in_escape {
                if c.is_ascii_alphabetic() {
                    in_escape = false;
                }
            } else if c == '\x1b' {
                in_escape = true;
            } else {
                count += 1;
            }
        }
        count
    }

    /// Clear penuh + sembunyikan kursor (SEKALI di awal watch).
    pub(crate) fn init_screen(&self) {
        print!("\x1b[2J\x1b[?25l");
        let _ = io::stdout().flush();
    }

    /// Kursor ke home tanpa clear — anti-kedip.
    pub(crate) fn home_cursor(&self) {
        print!("\x1b[H");
    }

    /// Masuk alternate screen (`ESC[?1049h`) — scrollback tidak tercemar.
    pub(crate) fn enter_alt_screen(&self) {
        print!("\x1b[?1049h");
        let _ = io::stdout().flush();
    }

    /// Keluar alternate screen dan pulihkan terminal.
    pub(crate) fn leave_alt_screen(&self) {
        print!("\x1b[?1049l");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_width_mengabaikan_kode_warna_ansi() {
        assert_eq!(Renderer::visible_width("abc"), 3);
        assert_eq!(Renderer::visible_width("\x1b[1;31mabc\x1b[0m"), 3);
        assert_eq!(Renderer::visible_width("\x1b[38;5;88m█\x1b[0m"), 1);
        assert_eq!(Renderer::visible_width(""), 0);
    }
}
