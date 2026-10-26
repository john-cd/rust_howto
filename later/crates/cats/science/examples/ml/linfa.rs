#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Predict numeric values with a linear regression model from `linfa-linear`.

use std::error::Error;

use linfa::dataset::Dataset;
use linfa::prelude::Predict as _;
use linfa::traits::Fit as _;
use linfa_linear::LinearRegression;

/// Fits a linear model and predicts two values outside the training range.
///
/// # Errors
///
/// Returns an error if fitting the linear model fails.
fn regression_predictions() -> Result<Vec<f64>, Box<dyn Error>> {
    let features = ndarray_016::array![[1.0], [2.0], [3.0], [4.0], [5.0]];
    let targets = ndarray_016::array![1.1, 2.2, 2.8, 3.95, 5.1];
    let dataset = Dataset::new(features, targets);
    let model = LinearRegression::default().fit(&dataset)?;
    let test_data = ndarray_016::array![[6.0], [7.0]];

    Ok(model.predict(&test_data).to_vec())
}

/// Runs the regression example and prints its predictions.
fn main() -> Result<(), Box<dyn Error>> {
    let predictions = regression_predictions()?;
    println!("Predictions: {predictions:?}");
    Ok(())
}
// ANCHOR_END: example

pub fn run() {
    main().expect("the Linfa regression example failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the regression model predicts the expected numeric range.
    #[test]
    fn regression_predicts_values_near_the_linear_trend()
    -> Result<(), Box<dyn Error>> {
        let predictions = regression_predictions()?;

        assert_eq!(predictions.len(), 2);
        assert!((predictions[0] - 6.0).abs() < 0.2);
        assert!((predictions[1] - 7.0).abs() < 0.2);
        Ok(())
    }
}
