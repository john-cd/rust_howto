#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates how to use the `open` crate to open files and URLs using the
//! default program associated with the file type.

fn main() {
    //  The `that` function attempts to open the specified path with the
    // appropriate application. If an error occurs during the opening
    // process, it will be printed to the standard error stream.
    if let Err(e) = open::that("example.txt") {
        eprintln!("Failed to open file: {e}");
    }

    // Open a URL using the default web browser.
    if let Err(e) = open::that("https://www.rust-lang.org") {
        eprintln!("Failed to open URL: {e}");
    }
    // OR: open::with("http://rust-lang.org", "firefox")?;

    // Depending on the platform and system configuration, launchers can block.
    // To be sure they don't, use `that_in_background()` or
    // `that_detached` instead.

    // Get the commands that would be used to try to open the specified path.
    // This is useful for understanding how the `open` crate would handle a
    // given path. The commands are printed to the standard output stream.
    let path = "http://rust-lang.org";
    open::commands(path).iter().for_each(|x| println!("{x:?}"));
}
// ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// [how to test properly](https://github.com/john-cd/rust_howto/issues/1004)
