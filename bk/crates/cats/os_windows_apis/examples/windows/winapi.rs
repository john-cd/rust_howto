// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Displays a message box through the raw `winapi` bindings.

#![cfg(target_os = "windows")]

use std::ptr::null_mut;

use winapi::um::winuser::MB_OK;
use winapi::um::winuser::MessageBoxW;

/// Encodes a string as UTF-16 and appends the NUL required by Win32 APIs.
fn nul_terminated_wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Displays a message box using NUL-terminated UTF-16 strings.
fn main() {
    let message = nul_terminated_wide("Hello from winapi!");
    let caption = nul_terminated_wide("Windows API example");

    // SAFETY: Both strings are NUL-terminated and stay alive for the call;
    // a null owner window is valid for a standalone message box.
    unsafe {
        MessageBoxW(null_mut(), message.as_ptr(), caption.as_ptr(), MB_OK);
    }
}
// ANCHOR_END: example

/// Runs the message-box example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies UTF-16 strings end in exactly one NUL terminator.
    #[test]
    fn message_box_strings_are_nul_terminated() {
        let message = nul_terminated_wide("Hello");

        assert_eq!(
            message,
            [
                b'H' as u16,
                b'e' as u16,
                b'l' as u16,
                b'l' as u16,
                b'o' as u16,
                0
            ]
        );
    }

    /// Runs the interactive message-box call on a Windows desktop.
    #[test]
    #[ignore = "displays a Windows message box"]
    fn displays_message_box() {
        main();
    }
}
