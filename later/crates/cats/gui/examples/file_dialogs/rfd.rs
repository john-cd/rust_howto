// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Demonstrates opening a file dialog using the `rfd` crate.

use rfd::FileDialog;

/// Opens a file dialog and prints the selected file.
fn main() {
    let file = FileDialog::new()
        .add_filter("Text files", &["txt"])
        .set_title("Choose a Text File")
        .pick_file();

    match file {
        Some(path) => println!("Selected file: {}", path.display()),
        None => println!("No file selected"),
    }
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires interactive example runtime"]
    fn test_main() {
        main();
    }
}
// [how to test](https://github.com/john-cd/rust_howto/issues/785)
