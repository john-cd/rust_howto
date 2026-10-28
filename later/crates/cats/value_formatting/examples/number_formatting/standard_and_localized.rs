// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates standard Rust format syntax and locale-aware integer grouping.

use num_format::Locale;
use num_format::ToFormattedString;

/// Prints standard and locale-aware number formats.
fn main() {
    let aligned = format!("{:>10.2}", 12.5);
    let scientific = format!("{:.3e}", 1250.0);
    let hexadecimal = format!("{:#x}", 255);
    let binary = format!("{:08b}", 42);
    let english = 1_234_567_i64.to_formatted_string(&Locale::en);
    let german = 1_234_567_i64.to_formatted_string(&Locale::de);

    assert_eq!(aligned, "     12.50");
    assert_eq!(scientific, "1.250e3");
    assert_eq!(hexadecimal, "0xff");
    assert_eq!(binary, "00101010");
    assert_eq!(english, "1,234,567");
    assert_eq!(german, "1.234.567");

    println!("aligned={aligned}, scientific={scientific}");
    println!("hex={hexadecimal}, binary={binary}");
    println!("English={english}, German={german}");
}

// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies stable standard formatting and locale grouping outputs.
    #[test]
    fn standard_and_locale_formats_match_expected_strings() {
        assert_eq!(format!("{:>10.2}", 12.5), "     12.50");
        assert_eq!(format!("{:.3e}", 1250.0), "1.250e3");
        assert_eq!(format!("{:#x}", 255), "0xff");
        assert_eq!(format!("{:08b}", 42), "00101010");
        assert_eq!(1_234_567_i64.to_formatted_string(&Locale::en), "1,234,567");
        assert_eq!(1_234_567_i64.to_formatted_string(&Locale::de), "1.234.567");
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
