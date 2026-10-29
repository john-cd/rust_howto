// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Represent, add, and format money with an explicit currency.

use rusty_money::Money;
use rusty_money::iso;

/// Builds amounts from minor units, adds them, and formats the result.
pub fn run() {
    // The amount is stored as a decimal; the currency travels with it.
    // 1999 minor units of USD is $19.99.
    let price = Money::from_minor(1999, iso::USD);
    let tax = Money::from_minor(160, iso::USD);
    let total = price.add(tax).expect("same currency, no overflow");
    println!("{total}");
    assert_eq!(total.to_string(), "$21.59");

    // Parse user input in the currency's own format.
    let parsed =
        Money::from_str("1,000.50", iso::USD).expect("valid USD amount");
    assert_eq!(parsed.amount().to_string(), "1000.50");

    // The same amount formats according to the currency's conventions.
    let euros = Money::from_minor(123_456, iso::EUR);
    assert_eq!(euros.to_string(), "€1.234,56");

    // Adding different currencies is an error, not a meaningless number.
    let mixed =
        Money::from_minor(100, iso::USD).add(Money::from_minor(100, iso::EUR));
    assert!(mixed.is_err());
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_succeeds() {
        run();
    }
}
