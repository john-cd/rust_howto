//! Estimates a European call payoff with a reproducible random stream.

// ANCHOR: example
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_distr::Distribution;
use rand_distr::StandardNormal;

/// Discounts simulated call payoffs for one year under geometric Brownian
/// motion.
fn estimate_call(volatility: f64, seed: u64) -> f64 {
    let mut rng = StdRng::seed_from_u64(seed);
    let paths = 100_000_u32;
    let spot = 100.0;
    let strike = 100.0;
    let rate = 0.05_f64;
    let drift = rate - 0.5 * volatility * volatility;
    let mut total = 0.0;
    for _ in 0..paths {
        let shock: f64 = StandardNormal.sample(&mut rng);
        let terminal = spot * (drift + volatility * shock).exp();
        total += (terminal - strike).max(0.0);
    }
    (-rate).exp() * total / f64::from(paths)
}

/// Prints a seeded estimate; model inputs and the one-year horizon are fixed.
fn main() {
    println!("Estimated call value: {:.4}", estimate_call(0.20, 7));
}
// ANCHOR_END: example

/// Runs the seeded simulation without market data or network access.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the zero-volatility limit against a deterministic discounted
    /// payoff.
    #[test]
    fn zero_volatility_matches_the_deterministic_payoff() {
        let expected = 100.0 - 100.0 * (-0.05_f64).exp();
        assert!((estimate_call(0.0, 7) - expected).abs() < 1e-9);
    }

    /// Compares a seeded estimate with the analytic Black-Scholes reference
    /// value.
    #[test]
    fn sampled_price_approximates_the_analytic_price() {
        assert!((estimate_call(0.20, 7) - 10.450_583_572_185_565).abs() < 0.25);
    }

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() {
        main();
    }

    #[test]
    fn same_seed_gives_the_same_estimate() {
        assert_eq!(estimate_call(0.20, 7), estimate_call(0.20, 7));
    }

    #[test]
    fn higher_volatility_raises_the_call_value() {
        assert!(estimate_call(0.40, 7) > estimate_call(0.20, 7));
    }
}
