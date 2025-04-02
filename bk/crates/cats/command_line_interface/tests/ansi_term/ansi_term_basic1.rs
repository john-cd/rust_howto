// ANCHOR: example
use ansi_term::Style;

/// Basic example of using ansi_term to print bold text.
fn main() {
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
