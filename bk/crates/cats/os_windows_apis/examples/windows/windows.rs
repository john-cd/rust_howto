// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `windows` crate to call the
//! `MessageBoxW` function from the Windows API.
//! It displays a simple message box with the text "Hello, windows!" and
//! the title "Greetings".

#![cfg(target_os = "windows")]

use windows::Win32::UI::WindowsAndMessaging::MB_OK;
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows::core::w;

/// Displays a message box saying "Hello, windows!".
///
/// # Errors
///
/// Returns the Windows error if the message box cannot be displayed.
fn main() -> std::io::Result<()> {
    // SAFETY: The `w!` macro supplies static, NUL-terminated UTF-16 strings;
    // a null owner window is valid for a standalone message box.
    let result = unsafe { MessageBoxW(None, w!("Hello, windows!"), w!("Greetings"), MB_OK) };
    if result.0 == 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}
// ANCHOR_END: example

/// Displays a message box saying "Hello, windows!".
///
/// # Errors
///
/// Returns the Windows error if the message box cannot be displayed.
pub fn run() -> std::io::Result<()> {
    main()
}

/// Displays a message box when explicitly run on a Windows desktop.
#[cfg(test)]
mod tests {
    use super::*;

    /// Displays a message box when explicitly run on a Windows desktop.
    #[test]
    #[ignore = "displays a Windows message box"]
    fn displays_message_box() {
        main().expect("message box should display successfully");
    }
}
