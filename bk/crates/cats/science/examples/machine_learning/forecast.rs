// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Forecast a seasonal time series with `augurs`: MSTL removes the seasonal
//! component, and an exponential smoothing (ETS) model forecasts the trend.
//!
//! A time series is a sequence of values measured over time, such as daily
//! sales. Seasonality is a pattern that repeats at a fixed period, for example
//! weekly. MSTL splits a series into trend, seasonal parts and a remainder, so
//! the forecast can model each part.

use augurs::Fit as _;
use augurs::Predict as _;
use augurs::ets::AutoETS;
use augurs::mstl::MSTLModel;

/// A synthetic series: a linear trend plus a weekly pattern.
fn series(length: usize) -> Vec<f64> {
    (0..length)
        .map(|i| {
            let weekly = [0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 0.0][i % 7];
            10.0 + 0.1 * (i as f64) + weekly
        })
        .collect()
}

/// Fits the model and forecasts two weeks ahead, with a 95% interval.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let y = series(70);
    let (training, held_out) = y.split_at(56);
    let trend = AutoETS::non_seasonal().into_trend_model();
    let model = MSTLModel::new(vec![7], trend);
    let fitted = model.fit(training)?;

    let horizon = 14;
    let forecast = fitted.predict(horizon, 0.95)?;
    let intervals = forecast
        .intervals
        .as_ref()
        .ok_or("forecast intervals were not returned")?;
    println!("point forecast: {:.2?}", forecast.point);

    assert_eq!(forecast.point.len(), horizon);
    assert_eq!(intervals.lower.len(), horizon);
    assert_eq!(intervals.upper.len(), horizon);
    for ((lo, point), hi) in intervals
        .lower
        .iter()
        .zip(&forecast.point)
        .zip(&intervals.upper)
    {
        assert!(lo.is_finite() && point.is_finite() && hi.is_finite());
        assert!(lo <= point && point <= hi);
    }
    // Evaluate later timestamps only; the model never sees these observations.
    let mae = held_out
        .iter()
        .zip(&forecast.point)
        .map(|(actual, predicted)| (actual - predicted).abs())
        .sum::<f64>()
        / horizon as f64;
    let naive_mae = held_out
        .iter()
        .enumerate()
        .map(|(i, actual)| (actual - training[49 + i % 7]).abs())
        .sum::<f64>()
        / horizon as f64;
    println!("Held-out MAE: {mae:.4}; seasonal-naive MAE: {naive_mae:.4}");
    assert!(
        mae < naive_mae,
        "forecast must beat repeating the last training week"
    );
    Ok(())
}
// ANCHOR_END: example

/// Fits the model and forecasts two weeks ahead, with a 95% interval.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    main().map_err(|e| anyhow::anyhow!("{e}"))
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Beats a seasonal-naive baseline on the next two unseen weeks.
    #[test]
    fn test_forecast_example_entry_point() {
        main().expect("the model fits");
    }
}
