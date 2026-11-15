// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Discounts a fixed sequence of cash flows with `financial`.
//!
//! Money today is worth more than the same money later, because it can earn
//! interest. Net present value (NPV) discounts each future cash flow by a rate
//! and sums them, so an investment can be judged in today's money: a positive
//! NPV means it earns more than the discount rate.

/// Keeps the time-zero investment separate from discounted future payments.
fn main() {
    let initial_investment = -1000.0;
    let future_payments = [600.0, 600.0];
    let present_value = initial_investment + financial::npv(0.10, &future_payments);
    let expected = -1000.0 + 600.0 / 1.10 + 600.0 / 1.10_f64.powi(2);
    assert!((present_value - expected).abs() < 1e-9);
    println!("Net present value: {present_value:.2}");
}
// ANCHOR_END: example

/// Runs the fixed cash-flow calculation.
/// Discounts a fixed sequence of cash flows with `financial`.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {

    /// Discounts future cash flows against an initial investment.
    #[test]
    fn discounts_future_payments_from_an_initial_outflow() {
        super::main();
        assert!((financial::npv(0.0, &[-1000.0, 600.0, 600.0]) - 200.0).abs() < 1e-9);
    }
}
