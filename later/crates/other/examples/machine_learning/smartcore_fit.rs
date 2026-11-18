// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Fits a classifier on training rows and evaluates separate, unseen rows.

use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;

/// Fits two synthetic classes separated by the sign of the coordinate sum.
///
/// # Errors
///
/// Returns an error if matrix construction, model fitting, or prediction fails.
fn evaluate() -> anyhow::Result<(Vec<i32>, f64)> {
    let training = DenseMatrix::from_2d_array(&[
        &[-2.0, -2.0],
        &[-1.0, -1.0],
        &[-2.0, 1.0],
        &[1.0, -2.0],
        &[1.0, 1.0],
        &[2.0, 2.0],
        &[2.0, -1.0],
        &[-1.0, 2.0],
    ])?;
    let training_labels = vec![0, 0, 0, 0, 1, 1, 1, 1];
    let held_out = DenseMatrix::from_2d_array(&[
        &[-3.0, 0.0],
        &[0.0, -3.0],
        &[-0.75, -0.75],
        &[3.0, 0.0],
        &[0.0, 3.0],
        &[0.75, 0.75],
    ])?;
    let expected = vec![0, 0, 0, 1, 1, 1];
    let model = LogisticRegression::fit(&training, &training_labels, Default::default())?;
    let predicted = model.predict(&held_out)?;
    let score = accuracy(&expected, &predicted);
    Ok((predicted, score))
}

/// Reports accuracy on six held-out synthetic rows.
///
/// # Errors
///
/// Returns an error if fitting or evaluating the model fails.
fn main() -> anyhow::Result<()> {
    let (predicted, score) = evaluate()?;
    println!("Held-out predictions: {predicted:?}; accuracy: {score}");
    anyhow::ensure!(score > 0.9, "classifier failed the synthetic holdout");
    Ok(())
}
// ANCHOR_END: example

/// Runs the held-out classification example.
///
/// # Errors
///
/// Returns an error if fitting or evaluation fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Checks predictions against independently specified labels for unseen rows.
#[cfg(test)]
mod tests {
    use super::*;

    /// Classifies positive and negative sums on rows excluded from training.
    #[test]
    fn predicts_unseen_rows() -> anyhow::Result<()> {
        let (predicted, score) = evaluate()?;
        assert_eq!(predicted, [0, 0, 0, 1, 1, 1]);
        assert_eq!(score, 1.0);
        Ok(())
    }
}
