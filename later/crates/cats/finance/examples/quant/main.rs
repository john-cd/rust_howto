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
