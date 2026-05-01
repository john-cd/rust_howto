#![allow(dead_code)]
#![cfg(target_os = "windows")]
//! This example demonstrates how to use the `windows` crate to call the
//! `MessageBoxW` function from the Windows API.
//! It displays a simple message box with the text "Hello, windows!" and
//! the title "Greetings".

use windows::Win32::UI::WindowsAndMessaging::MB_OK;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows::core::w;

// ANCHOR: example
/// Simple Windows application that displays a message box
/// saying "Hello, windows!".
pub fn main() {
    unsafe {
        MessageBoxW(None, w!("Hello, windows!"), w!("Greetings"), MB_OK);
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Displays a message box"]
    fn test() {
        main();
    }
}
