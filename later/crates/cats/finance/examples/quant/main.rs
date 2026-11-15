//! Runs the quantitative finance examples.
//!
//! TODO Reviewed ??-??-2026.

/// Price European options with the `black_scholes` crate, then recover the
mod black_scholes;
/// Estimates a European call payoff with a reproducible random stream.
mod monte_carlo;
/// Discounts a fixed sequence of cash flows with `financial`.
mod net_present_value;
/// Daily returns and a 20-day rolling standard deviation, annualized with
mod rolling_stats;
/// Price European options with the Black-Scholes formula, using `statrs` for
mod rustquant;
/// Count NYSE trading days and step forward by a number of trading days with
mod trading_days;

/// Runs the quant examples.
fn main() -> anyhow::Result<()> {
    rustquant::run();
    black_scholes::run();
    rolling_stats::run();
    trading_days::run()?;
    net_present_value::run();
    monte_carlo::run();
    Ok(())
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
