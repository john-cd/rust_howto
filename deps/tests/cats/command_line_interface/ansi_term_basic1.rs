// ANCHOR: example
use ansi_term::Style;

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
