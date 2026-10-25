#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Parameterized tests and fixtures with `rstest`.

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

/// The example runs under `cargo test`.
fn main() {}

#[cfg(test)]
mod tests {
    use rstest::fixture;
    use rstest::rstest;

    use super::*;

    // One test function expands to one test per `#[case]` row; a failure
    // names the row, e.g. `tests::parses::case_2`.
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
    #[fixture]
    fn inputs() -> Vec<&'static str> {
        vec!["1s", "1m", "1h"]
    }

    #[rstest]
    fn units_increase(inputs: Vec<&str>) {
        let seconds: Vec<u64> =
            inputs.iter().filter_map(|s| parse_seconds(s)).collect();
        assert!(seconds.is_sorted());
    }

    // `#[values]` runs every combination.
    #[rstest]
    fn zero_is_zero(#[values("0s", "0m", "0h")] input: &str) {
        assert_eq!(parse_seconds(input), Some(0));
    }
}
