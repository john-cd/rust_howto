// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates opening a file dialog using the rfd crate.

use std::fs;
use std::path::Path;

use anyhow::Context;
use rfd::FileDialog;

/// Builds a text-file dialog that starts in the current directory.
fn text_file_dialog() -> anyhow::Result<FileDialog> {
    let current_dir =
        std::env::current_dir().context("read the current directory")?;
    Ok(FileDialog::new()
        .add_filter("Text files", &["txt", "md"])
        .set_title("Choose a Text File")
        .set_directory(current_dir))
}

/// Reads a selected file, returning None when the user cancels the dialog.
fn process_selection(path: Option<&Path>) -> anyhow::Result<Option<String>> {
    path.map(|path| {
        fs::read_to_string(path)
            .with_context(|| format!("read selected file {}", path.display()))
    })
    .transpose()
}

/// Opens the dialog and prints the selected text or a cancellation message.
fn main() -> anyhow::Result<()> {
    let selected_path = text_file_dialog()?.pick_file();
    match process_selection(selected_path.as_deref())? {
        Some(contents) => println!("Selected file contents:\n{contents}"),
        None => println!("File selection cancelled"),
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the file-dialog example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use super::*;

    /// Opens a native file dialog for manual testing.
    #[test]
    #[ignore = "opens a native file dialog"]
    fn test_rfd_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Reads file contents selected in the dialog without opening a dialog.
    #[test]
    fn reads_selected_file_contents() -> anyhow::Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("get a unique temporary-file name")?
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rust-howto-rfd-{}-{timestamp}.txt",
            std::process::id()
        ));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .with_context(|| {
                format!("create temporary file {}", path.display())
            })?;
        file.write_all(b"selected text")
            .context("write temporary file")?;
        drop(file);

        let selection = process_selection(Some(path.as_path()));
        std::fs::remove_file(&path).with_context(|| {
            format!("remove temporary file {}", path.display())
        })?;
        assert_eq!(selection?.as_deref(), Some("selected text"));
        Ok(())
    }

    /// Returns no contents when the user cancels the dialog.
    #[test]
    fn cancellation_returns_no_contents() -> anyhow::Result<()> {
        assert!(process_selection(None)?.is_none());
        Ok(())
    }
}
