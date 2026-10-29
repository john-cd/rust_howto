//! Runs the quantitative finance examples.
//!
//! TODO Reviewed ??-??-2026.

mod black_scholes;
mod monte_carlo;
mod net_present_value;
mod rolling_stats;
mod rustquant;
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
