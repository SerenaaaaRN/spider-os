//! Kontrol terminal: enable ANSI di Windows (FFI kernel32) dan kursor.

use std::io::{self, Write};

/// Aktifkan virtual terminal processing di conhost; no-op di non-Windows.
#[cfg(windows)]
pub(crate) fn enable_virtual_terminal() {
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

#[cfg(not(windows))]
pub(crate) fn enable_virtual_terminal() {}

/// Tampilkan kursor kembali (`ESC[?25h`).
pub(crate) fn show_cursor() {
    print!("\x1b[?25h");
    let _ = io::stdout().flush();
}
