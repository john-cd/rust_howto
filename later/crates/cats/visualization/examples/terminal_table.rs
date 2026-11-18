// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Renders an exact-value report with tabled.

use tabled::builder::Builder;
use tabled::settings::Style;

/// Renders the given readings with explicit units and decimal precision.
fn report(readings: &[(&str, f64)]) -> String {
    let mut builder = Builder::new();
    builder.push_record(["Station", "Distance (m)"]);
    for &(name, metres) in readings {
        builder.push_record([name.to_owned(), format!("{metres:.1}")]);
    }
    let mut table = builder.build();
    table.with(Style::ascii()).to_string()
}

/// Prints a table of sample measurements.
fn main() {
    println!("{}", report(&[("North", 12.5), ("South", 8.0)]));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Keeps units with the column and presents each row's measured value.
    #[test]
    fn includes_units_and_values() {
        let table = report(&[("North", 12.5), ("South", 8.0)]);
        assert!(table.contains("Distance (m)"));
        assert!(
            table
                .lines()
                .any(|line| line.contains("North") && line.contains("12.5"))
        );
        assert!(
            table
                .lines()
                .any(|line| line.contains("South") && line.contains("8.0"))
        );
    }
}
