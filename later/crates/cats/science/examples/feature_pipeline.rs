// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fits preprocessing on training rows and evaluates a regression on later rows.

use anyhow::Result;
use anyhow::ensure;
use linfa::Dataset;
use linfa::prelude::Predict;
use linfa::traits::Fit;
use linfa_linear::LinearRegression;
use ndarray_016::Array1;
use ndarray_016::Array2;
use ndarray_016::Axis;
use ndarray_016::array;

/// Training-set means and population standard deviations in fixed feature order.
#[derive(Debug)]
struct Standardizer {
    mean: Array1<f64>,
    scale: Array1<f64>,
}

impl Standardizer {
    /// Fits finite, nonempty training rows; constant columns use scale one.
    fn fit(rows: &Array2<f64>) -> Result<Self> {
        ensure!(
            rows.nrows() > 0 && rows.ncols() > 0,
            "empty training matrix"
        );
        ensure!(rows.iter().all(|v| v.is_finite()), "non-finite feature");
        let mean = rows.mean_axis(Axis(0)).expect("nonempty row axis");
        let scale = rows
            .var_axis(Axis(0), 0.0)
            .mapv(|v| if v == 0.0 { 1.0 } else { v.sqrt() });
        ensure!(
            mean.iter().chain(scale.iter()).all(|v| v.is_finite()),
            "scaler overflow"
        );
        Ok(Self { mean, scale })
    }

    /// Applies the fitted feature order without changing training statistics.
    fn transform(&self, rows: &Array2<f64>) -> Result<Array2<f64>> {
        ensure!(rows.ncols() == self.mean.len(), "feature count mismatch");
        ensure!(rows.iter().all(|v| v.is_finite()), "non-finite feature");
        let scaled = (rows - &self.mean) / &self.scale;
        ensure!(
            scaled.iter().all(|v| v.is_finite()),
            "scaled feature overflow"
        );
        Ok(scaled)
    }
}

/// Computes RMSE only for aligned, nonempty, finite series.
fn rmse(actual: &[f64], predicted: &[f64]) -> Result<f64> {
    ensure!(
        !actual.is_empty() && actual.len() == predicted.len(),
        "unaligned or empty series"
    );
    ensure!(
        actual.iter().chain(predicted).all(|v| v.is_finite()),
        "non-finite value"
    );
    let mse = actual
        .iter()
        .zip(predicted)
        .map(|(a, p)| (a - p).powi(2))
        .sum::<f64>()
        / actual.len() as f64;
    ensure!(mse.is_finite(), "squared error overflow");
    Ok(mse.sqrt())
}

/// Trains on earlier observations and compares held-out RMSE with a mean baseline.
fn evaluate() -> Result<(f64, f64)> {
    let train = array![[0.0], [1.0], [2.0], [3.0], [4.0]];
    let targets = array![1.0, 3.0, 5.0, 7.0, 9.0];
    let held_out = array![[5.0], [6.0]];
    let truth = [11.0, 13.0];
    let scaler = Standardizer::fit(&train)?;
    let dataset = Dataset::new(scaler.transform(&train)?, targets.clone());
    let model = LinearRegression::default().fit(&dataset)?;
    let predictions = model.predict(&scaler.transform(&held_out)?);
    let baseline = targets.mean().expect("nonempty training targets");
    Ok((
        rmse(&truth, &predictions.to_vec())?,
        rmse(&truth, &[baseline; 2])?,
    ))
}

/// Prints held-out and baseline errors on the same target scale.
fn main() -> Result<()> {
    let (model, baseline) = evaluate()?;
    ensure!(
        model < 1e-8 && model < baseline,
        "regression failed to beat baseline"
    );
    println!("Held-out RMSE: {model:.6}; training-mean baseline: {baseline:.3}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Held-out extremes cannot change fitted training means or scales.
    #[test]
    fn preprocessing_does_not_leak() -> Result<()> {
        let scaler = Standardizer::fit(&array![[1.0, 7.0], [3.0, 7.0]])?;
        let transformed = scaler.transform(&array![[101.0, 7.0]])?;
        assert_eq!(scaler.mean.to_vec(), vec![2.0, 7.0]);
        assert_eq!(scaler.scale.to_vec(), vec![1.0, 1.0]);
        assert_eq!(transformed.row(0).to_vec(), vec![99.0, 0.0]);
        Ok(())
    }

    /// Empty, mismatched and non-finite inputs produce errors rather than metrics.
    #[test]
    fn invalid_data_is_rejected() -> Result<()> {
        assert!(Standardizer::fit(&Array2::zeros((0, 1))).is_err());
        assert!(Standardizer::fit(&array![[f64::NAN]]).is_err());
        let scaler = Standardizer::fit(&array![[1.0], [2.0]])?;
        assert!(scaler.transform(&array![[1.0, 2.0]]).is_err());
        assert!(scaler.transform(&array![[f64::INFINITY]]).is_err());
        assert!(rmse(&[], &[]).is_err());
        assert!(rmse(&[1.0], &[]).is_err());
        assert!(rmse(&[f64::NAN], &[1.0]).is_err());
        assert!(rmse(&[f64::MAX], &[-f64::MAX]).is_err());
        Ok(())
    }

    /// A correctly fitted line predicts held-out values better than the mean baseline.
    #[test]
    fn regression_beats_baseline() -> Result<()> {
        let (model, baseline) = evaluate()?;
        assert!(model < 1e-8 && baseline > 7.0);
        Ok(())
    }
}
