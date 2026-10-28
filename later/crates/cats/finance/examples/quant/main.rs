//! Runs the quantitative finance examples.
//!
//! TODO Reviewed ??-??-2026.

mod black_scholes;
mod rolling_stats;
mod rustquant;
mod trading_days;

/// Runs the quant examples.
fn main() {
    rustquant::run();
    black_scholes::run();
    rolling_stats::run();
    trading_days::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
