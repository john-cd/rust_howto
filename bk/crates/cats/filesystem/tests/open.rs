#![allow(dead_code)]
// ANCHOR: example
use open::that;

fn main() {
    // Open a file using the default program
    if let Err(e) = that("example.txt") {
        eprintln!("Failed to open file: {}", e);
    }

    // Open a URL using the default web browser
    if let Err(e) = that("https://www.rust-lang.org") {
        eprintln!("Failed to open URL: {}", e);
    }
    // OR: open::with("http://rust-lang.org", "firefox")?;

    // Depending on the platform and system configuration, launchers can block.
    // If you want to be sure they don't, use that_in_background() or
    // that_detached instead.

    // Get the commands to try open the path with.
    let path = "http://rust-lang.org";
    open::commands(path)
        .iter()
        .for_each(|x| println!("{:?}", x));
}
// ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// [how to test properly](https://github.com/john-cd/rust_howto/issues/1004)
