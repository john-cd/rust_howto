// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Price European options with the Black-Scholes formula, using `statrs` for
//! the normal distribution. No finance crate is needed.

use statrs::distribution::ContinuousCDF;
use statrs::distribution::Normal;

/// The inputs of the Black-Scholes model.
struct Inputs {
    /// Price of the underlying asset.
    spot: f64,
    /// Strike price of the option.
    strike: f64,
    /// Time to maturity in years.
    years: f64,
    /// Annualized volatility of the underlying asset.
    volatility: f64,
    /// Continuously compounded risk-free rate.
    rate: f64,
}

/// Returns the prices `(call, put)` of European options without dividends.
fn price(inputs: &Inputs) -> (f64, f64) {
    let Inputs {
        spot,
        strike,
        years,
        volatility,
        rate,
    } = *inputs;
    let n = Normal::standard();
    let vol_sqrt_t = volatility * years.sqrt();
    let d1 = ((spot / strike).ln() + (rate + 0.5 * volatility.powi(2)) * years)
        / vol_sqrt_t;
    let d2 = d1 - vol_sqrt_t;
    let discounted_strike = strike * (-rate * years).exp();
    let call = spot * n.cdf(d1) - discounted_strike * n.cdf(d2);
    let put = discounted_strike * n.cdf(-d2) - spot * n.cdf(-d1);
    (call, put)
}

/// Prices an at-the-money option and prints the result.
fn main() {
    let (call, put) = price(&Inputs {
        spot: 100.0,
        strike: 100.0,
        years: 1.0,
        volatility: 0.2,
        rate: 0.05,
    });
    println!("European call: {call:.4}");
    println!("European put: {put:.4}");
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The textbook inputs: S = 100, K = 100, r = 5%, sigma = 20%, T = 1.
    const TEXTBOOK: Inputs = Inputs {
        spot: 100.0,
        strike: 100.0,
        years: 1.0,
        volatility: 0.2,
        rate: 0.05,
    };

    /// The call price matches the published value of about 10.4506.
    #[test]
    fn call_matches_textbook_value() {
        let (call, put) = price(&TEXTBOOK);
        assert!((call - 10.4506).abs() < 1e-3, "call = {call}");
        assert!((put - 5.5735).abs() < 1e-3, "put = {put}");
    }

    /// Put-call parity: C - P = S - K * exp(-rT).
    #[test]
    fn put_call_parity_holds() {
        let (call, put) = price(&TEXTBOOK);
        let parity = TEXTBOOK.spot
            - TEXTBOOK.strike * (-TEXTBOOK.rate * TEXTBOOK.years).exp();
        assert!((call - put - parity).abs() < 1e-9);
    }

    /// Runs the example through its entry point.
    #[test]
    fn test_rustquant_example_entry_point() {
        main();
    }
}
