// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generate a coverage report for a function with three execution paths.

/// Returns the shipping charge in cents for a subtotal in cents, or no quote for an empty order.
fn shipping_cents(subtotal_cents: u32) -> Option<u32> {
    if subtotal_cents == 0 {
        None
    } else if subtotal_cents >= 5_000 {
        Some(0)
    } else {
        Some(499)
    }
}

/// Prints the shipping charge for an order below the free shipping threshold.
fn main() {
    if let Some(charge) = shipping_cents(2_500) {
        println!("Shipping: {charge} cents");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Charges the flat rate just below the free shipping threshold.
    #[test]
    fn charges_below_threshold() {
        assert_eq!(shipping_cents(4_999), Some(499));
    }

    /// Waives the charge at the free shipping threshold.
    #[test]
    fn waives_charge_at_threshold() {
        assert_eq!(shipping_cents(5_000), Some(0));
    }
}
// ANCHOR_END: example
