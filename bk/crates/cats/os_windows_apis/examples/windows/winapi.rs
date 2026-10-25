#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Displays a message box through the raw `winapi` bindings.

#![cfg(target_os = "windows")]

use std::ptr::null_mut;

use winapi::um::winuser::MB_OK;
use winapi::um::winuser::MessageBoxW;

/// Displays a message box using NUL-terminated UTF-16 strings.
fn main() {
    let message = "Hello from winapi!\0".encode_utf16().collect::<Vec<_>>();
    let caption = "Windows API example\0".encode_utf16().collect::<Vec<_>>();

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

    #[test]
    #[ignore = "displays a Windows message box"]
    fn displays_message_box() {
        main();
    }
}
// [review / test](https://github.com/john-cd/rust_howto/issues/822)
