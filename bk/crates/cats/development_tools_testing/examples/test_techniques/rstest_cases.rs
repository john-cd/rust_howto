// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Parameterized tests and fixtures with `rstest`.
//!
//! Table-driven tests run one test body with many input/expected pairs.
//! `rstest` generates a separate test per case, so a failure names exactly
//! which case broke.

/// Parses a duration such as `"90s"`, `"5m"` or `"2h"` into seconds.
fn parse_seconds(text: &str) -> Option<u64> {
    let (number, unit) = text.split_at(text.len().checked_sub(1)?);
    let number: u64 = number.parse().ok()?;
    let factor = match unit {
        "s" => 1,
        "m" => 60,
        "h" => 3600,
        _ => return None,
    };
    number.checked_mul(factor)
}
// ANCHOR_END: example

/// Parses a few durations; the tests below are parameterized.
/// Parses a representative duration with the example parser.
pub fn run() -> Option<u64> {
    parse_seconds("1m")
}

/// Runs the example.
fn main() {
    for text in ["90s", "5m", "2h", "5x"] {
        println!("{text:?} -> {:?}", parse_seconds(text));
    }
}

#[cfg(test)]
mod tests {
    use rstest::fixture;
    use rstest::rstest;

    use super::*;

    // One test function expands to one test per `#[case]` row; a failure
    // names the row, e.g. `tests::parses::case_2`.

    /// Converts supported duration strings to seconds and rejects invalid suffixes.
    #[rstest]
    #[case("90s", Some(90))]
    #[case("5m", Some(300))]
    #[case("2h", Some(7200))]
    #[case("5x", None)]
    #[case("", None)]
    fn parses(#[case] input: &str, #[case] expected: Option<u64>) {
        assert_eq!(parse_seconds(input), expected);
    }

    // A fixture builds shared test data; tests receive it by argument name.

    /// Supplies representative durations for parameterized tests.
    #[fixture]
    fn inputs() -> Vec<&'static str> {
        vec!["1s", "1m", "1h"]
    }

    /// Checks that durations increase as the unit gets larger.
    #[rstest]
    fn units_increase(inputs: Vec<&str>) {
        let seconds: Vec<u64> = inputs.iter().filter_map(|s| parse_seconds(s)).collect();
        assert!(seconds.is_sorted());
    }

    // `#[values]` runs every combination.

    /// Returns zero seconds for zero with every supported duration suffix.
    #[rstest]
    fn zero_is_zero(#[values("0s", "0m", "0h")] input: &str) {
        assert_eq!(parse_seconds(input), Some(0));
    }

    /// Runs the example through its entry point without error.
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
