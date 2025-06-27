// ANCHOR: example
//! This example demonstrates how to interact with the system clipboard using
//! the `arboard` crate.
//!
//! The `arboard` crate provides a cross-platform way to copy and paste text to
//! and from the clipboard.
//!
//! This example shows how to create a clipboard instance, set text to the
//! clipboard, and get text from the clipboard.
use arboard::Clipboard;

pub fn main() {
    // Create a new Clipboard instance.
    let mut clipboard = Clipboard::new().unwrap();

    // Set some text to the clipboard.
    clipboard.set_text("Hello, clipboard!").unwrap();
    println!("Text copied to clipboard.");

    // Get the text from the clipboard.
    let clipboard_text = clipboard.get_text().unwrap();
    println!("Text from clipboard: {clipboard_text}");
}
// You may also use:  get/set_html/image...

// ANCHOR_END: example

// Requires a clipboard on Linux; otherwise,
// "Unknown error while interacting with the clipboard: X11 server connection
// timed out because it was unreachable"
// [review NOW](https://github.com/john-cd/rust_howto/issues/1155)
