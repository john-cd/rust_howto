// ANCHOR: example
//! Demonstrates opening a file dialog using the `rfd` crate.

use rfd::FileDialog;

pub fn main() {
    let file = FileDialog::new()
        .add_filter("Text files", &["txt"])
        .set_title("Choose a Text File")
        .pick_file();

    match file {
        Some(path) => println!("Selected file: {path:?}"),
        None => println!("No file selected"),
    }
}
// ANCHOR_END: example
// [how to test](https://github.com/john-cd/rust_howto/issues/785)
