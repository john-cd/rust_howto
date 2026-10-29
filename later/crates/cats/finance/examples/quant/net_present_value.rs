//! Discounts a fixed sequence of cash flows with `financial`.

// ANCHOR: example
/// Keeps the time-zero investment separate from discounted future payments.
fn main() {
    let initial_investment = -1000.0;
    let future_payments = [600.0, 600.0];
    let present_value =
        initial_investment + financial::npv(0.10, &future_payments);
    let expected = -1000.0 + 600.0 / 1.10 + 600.0 / 1.10_f64.powi(2);
    assert!((present_value - expected).abs() < 1e-9);
    println!("Net present value: {present_value:.2}");
}
// ANCHOR_END: example

/// Runs the fixed cash-flow calculation.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    /// Compares the crate's timing convention with direct discounting.
    #[test]
    fn discounts_future_payments_from_an_initial_outflow() {
        super::main();
        assert!(
            (financial::npv(0.0, &[-1000.0, 600.0, 600.0]) - 200.0).abs()
                < 1e-9
        );
    }
}
