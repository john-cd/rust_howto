// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Train a logistic regression classifier in Rust with `smartcore`.
//!
//! Logistic regression is a classifier, despite the name: it predicts the
//! probability that an item belongs to a class (spam or not) from its features.
//! Training fits the weights; the quality is measured on data the model has not
//! seen.

use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;

/// Fits a classifier on a small in-memory dataset: class 1 when both
/// features are positive.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = DenseMatrix::from_2d_array(&[
        &[-2.0, -2.0],
        &[-1.0, -1.0],
        &[-1.0, 0.0],
        &[0.0, -1.0],
        &[1.0, 1.0],
        &[2.0, 2.0],
        &[1.0, 0.5],
        &[0.5, 1.0],
    ])?;
    let y: Vec<i32> = vec![0, 0, 0, 0, 1, 1, 1, 1];

    let model = LogisticRegression::fit(&x, &y, Default::default())?;
    let predicted = model.predict(&x)?;
    let score = accuracy(&y, &predicted);
    println!("accuracy: {score}");
    assert!(score > 0.9);
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main().map_err(|e| anyhow::anyhow!("{e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the smartcore fit example through its entry point.
    #[test]
    fn test_smartcore_fit_example_entry_point() {
        main().expect("the model fits");
    }
}
