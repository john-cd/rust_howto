#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Forecast a seasonal time series with `augurs`: MSTL removes the seasonal
//! component, and an exponential smoothing (ETS) model forecasts the trend.

use augurs::Fit as _;
use augurs::Predict as _;
use augurs::ets::AutoETS;
use augurs::mstl::MSTLModel;

/// A synthetic series: a linear trend plus a weekly pattern.
fn series() -> Vec<f64> {
    (0..56)
        .map(|i| {
            let weekly = [0.0, 1.0, 2.0, 3.0, 2.0, 1.0, 0.0][i % 7];
            10.0 + 0.1 * (i as f64) + weekly
        })
        .collect()
}

/// Fits the model and forecasts two weeks ahead, with a 95% interval.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let y = series();
    let trend = AutoETS::non_seasonal().into_trend_model();
    let model = MSTLModel::new(vec![7], trend);
    let fitted = model.fit(&y)?;

    let horizon = 14;
    let forecast = fitted.predict(horizon, 0.95)?;
    let intervals = forecast.intervals.as_ref().expect("requested");
    println!("point forecast: {:.2?}", forecast.point);

    assert_eq!(forecast.point.len(), horizon);
    for ((lo, point), hi) in intervals
        .lower
        .iter()
        .zip(&forecast.point)
        .zip(&intervals.upper)
    {
        assert!(lo <= point && point <= hi);
    }
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main().map_err(|e| anyhow::anyhow!("{e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main().expect("the model fits");
    }
}
