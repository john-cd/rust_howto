// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Price European options with the `black_scholes` crate, then recover the
//! volatility from a price (implied volatility).

/// Prices a call and a put, then solves for the volatility that reproduces
/// the call price.
fn main() {
    let spot = 100.0; // Price of the underlying.
    let strike = 105.0;
    let rate = 0.03; // Continuously compounded risk-free rate.
    let sigma = 0.25; // Annualized volatility.
    let maturity = 0.5; // Years to expiry.

    let call = black_scholes::call(spot, strike, rate, sigma, maturity);
    let put = black_scholes::put(spot, strike, rate, sigma, maturity);
    println!("call = {call:.4}, put = {put:.4}");

    // Put-call parity: C - P = S - K * exp(-r * T).
    let parity = spot - strike * (-rate * maturity).exp();
    assert!((call - put - parity).abs() < 1e-6);

    // Implied volatility: price -> volatility. `Err` carries the last
    // iterate when the solver did not converge.
    let iv = black_scholes::call_iv(call, spot, strike, rate, maturity)
        .expect("the solver converges for a valid price");
    println!("implied volatility = {iv:.4}");
    assert!((iv - sigma).abs() < 1e-4);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the black scholes example through its entry point.
    #[test]
    fn test_black_scholes_example_entry_point() {
        main();
    }

    #[test]
    fn put_implied_volatility_round_trips() {
        let put = black_scholes::put(100.0, 95.0, 0.02, 0.3, 1.0);
        let iv = black_scholes::put_iv(put, 100.0, 95.0, 0.02, 1.0)
            .expect("converges");
        assert!((iv - 0.3).abs() < 1e-4);
    }
}
