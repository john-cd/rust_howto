// TODO Reviewed ??-??-2026
#![allow(dead_code)]
// ANCHOR: example
use nu_ansi_term::Style;

/// Basic example of using `nu-ansi-term` to print bold text.
fn main() {
    println!(
        "{} and this is not",
        Style::new().bold().paint("This is Bold")
    );
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
    fn test_main() {
        main();
    }
}
