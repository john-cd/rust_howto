// ANCHOR: example
use arboard::Clipboard;

// The `arboard` crate interacts with the clipboard.
fn main() {
    // Create a new Clipboard instance
    let mut clipboard = Clipboard::new().unwrap();

    // Set some text to the clipboard
    clipboard.set_text("Hello, clipboard!").unwrap();
    println!("Text copied to clipboard.");

    // Get the text from the clipboard
    let clipboard_text = clipboard.get_text().unwrap();
    println!("Text from clipboard: {}", clipboard_text);
}
// ANCHOR_END: example

// Requires a clipboard on Linux; otherwise,
// "Unknown error while interacting with the clipboard: X11 server connection
// timed out because it was unreachable"
