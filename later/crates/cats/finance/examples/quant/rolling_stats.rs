// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Daily returns and a 20-day rolling standard deviation, annualized with
//! the square root of 252 trading days.
//!
//! A return is the percentage change of a price from one day to the next. The
//! standard deviation of returns measures volatility, a common measure of risk.
//! A rolling window recomputes it over the most recent N days, and annualizing
//! scales it to a yearly figure (daily values times the square root of roughly
//! 252 trading days).

use ta_statistics::SingleStatistics;

/// Number of trading days in a year, by convention.
const TRADING_DAYS: f64 = 252.0;

/// Simple daily returns: `p[t] / p[t-1] - 1`.
fn daily_returns(prices: &[f64]) -> Vec<f64> {
    prices.windows(2).map(|w| w[1] / w[0] - 1.0).collect()
}

/// Rolling population standard deviation, written by hand.
fn rolling_std_by_hand(values: &[f64], window: usize) -> Vec<f64> {
    values
        .windows(window)
        .map(|w| {
            let n = w.len() as f64;
            let mean = w.iter().sum::<f64>() / n;
            (w.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n).sqrt()
        })
        .collect()
}

/// Rolling population standard deviation with `ta-statistics`.
fn rolling_std(values: &[f64], window: usize) -> Vec<f64> {
    let mut stats = SingleStatistics::new(window);
    let mut out = Vec::new();
    for (i, v) in values.iter().enumerate() {
        stats.next(*v);
        // Skip until the window is full.
        if i + 1 >= window {
            out.extend(stats.stddev());
        }
    }
    out
}

/// Computes annualized rolling volatility for a synthetic price series.
fn main() {
    // Deterministic, synthetic prices.
    let prices: Vec<f64> = (0..60)
        .map(|i| 100.0 + f64::from(i) * 0.2 + (f64::from(i) * 0.7).sin() * 2.0)
        .collect();
    let returns = daily_returns(&prices);

    let rolling = rolling_std(&returns, 20);
    let by_hand = rolling_std_by_hand(&returns, 20);
    assert_eq!(rolling.len(), by_hand.len());
    for (a, b) in rolling.iter().zip(&by_hand) {
        assert!((a - b).abs() < 1e-9);
    }

    let annualized: Vec<f64> =
        rolling.iter().map(|s| s * TRADING_DAYS.sqrt()).collect();
    println!(
        "latest annualized volatility: {:.2}%",
        annualized.last().copied().unwrap_or_default() * 100.0
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rolling stats example through its entry point.
    #[test]
    fn test_rolling_stats_example_entry_point() {
        main();
    }

    #[test]
    fn returns_of_a_doubling_series() {
        let r = daily_returns(&[1.0, 2.0, 4.0]);
        assert_eq!(r, vec![1.0, 1.0]);
    }
}
