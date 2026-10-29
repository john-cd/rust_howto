// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example copies text to and reads text from the system clipboard with
//! the current `arboard` builder API.

use anyhow::Context;
use arboard::Clipboard;

/// Writes a sample string to the clipboard and reads it back.
fn main() -> anyhow::Result<()> {
    let mut clipboard =
        Clipboard::new().context("create a system clipboard handle")?;
    let text = "Hello, clipboard!";

    clipboard
        .set()
        .text(text)
        .context("write text to the system clipboard")?;
    let clipboard_text = clipboard
        .get()
        .text()
        .context("read text from the system clipboard")?;
    anyhow::ensure!(
        clipboard_text == text,
        "clipboard text did not round-trip"
    );
    println!("Text read from the clipboard: {clipboard_text}");

    Ok(())
}
// ANCHOR_END: example

/// Runs the clipboard example and returns any clipboard access error.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Tests that require access to the desktop clipboard.
#[cfg(test)]
mod tests {
    use anyhow::Context;
    use arboard::Clipboard;

    use super::main;

    /// Verifies writing and reading text through the system clipboard.
    ///
    /// Run manually on a desktop to replace its current clipboard text:
    /// `cargo test -p gui --example clipboard clipboard_text_round_trip --
    /// --ignored`.
    #[test]
    #[ignore = "requires a desktop clipboard and overwrites its current text"]
    fn clipboard_text_round_trip() -> anyhow::Result<()> {
        let mut clipboard =
            Clipboard::new().context("create a system clipboard handle")?;
        let text = "clipboard test value";

        clipboard
            .set()
            .text(text)
            .context("write text to the system clipboard")?;
        let clipboard_text = clipboard
            .get()
            .text()
            .context("read text from the system clipboard")?;
        assert_eq!(clipboard_text, text);
        Ok(())
    }

    #[test]
    #[ignore = "needs a desktop session with a system clipboard"]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
