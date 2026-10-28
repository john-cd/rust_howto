#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Formats numbers and dates with ICU4X using two different locales.

use std::error::Error;

use icu::datetime::DateTimeFormatter;
use icu::datetime::fieldsets::YMD;
use icu::datetime::input::Date;
use icu::decimal::DecimalFormatter;
use icu::decimal::input::Decimal;
use icu::locale::Locale;
use icu::locale::locale;

/// Formats a decimal with the grouping and digits selected by `locale`.
fn format_number(
    locale: Locale,
    value: &Decimal,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let formatter =
        DecimalFormatter::try_new(locale.into(), Default::default())?;

    Ok(formatter.format_to_string(value))
}

/// Prints the same number and date formatted for US English and German.
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let value = Decimal::from(1_234_567);
    let english = format_number(locale!("en-US"), &value)?;
    let german = format_number(locale!("de-DE"), &value)?;

    println!("en-US: {english}");
    println!("de-DE: {german}");
    let date = Date::try_new_iso(2025, 1, 15)?;
    for locale in [locale!("en-US"), locale!("de-DE")] {
        let formatter =
            DateTimeFormatter::try_new(locale.into(), YMD::medium())?;
        println!("{}", formatter.format(&date));
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the locale-sensitive formatting example.
pub fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that ICU4X applies locale-specific grouping separators.
    #[test]
    fn formats_grouping_for_english_and_german()
    -> Result<(), Box<dyn Error + Send + Sync>> {
        let value = Decimal::from(1_234_567);

        assert_eq!(format_number(locale!("en-US"), &value)?, "1,234,567");
        assert_eq!(format_number(locale!("de-DE"), &value)?, "1.234.567");

        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
