// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Train a SmartCore decision-tree classifier on synthetic data and measure its
//! accuracy on a seeded holdout split.
//!
//! A decision tree classifier predicts a label by asking a series of yes/no
//! questions about the features. It is trained on labelled examples and
//! measured by accuracy on data it predicts. Trees are easy to read but can
//! overfit, which means memorizing the training data instead of generalizing.

use std::error::Error;

use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::metrics::accuracy;
use smartcore::model_selection::train_test_split;
use smartcore::tree::decision_tree_classifier::DecisionTreeClassifier;

/// Creates two linearly separable classes with small, deterministic offsets.
fn synthetic_classification_data() -> (Vec<Vec<f64>>, Vec<u32>) {
    let mut features = Vec::with_capacity(120);
    let mut labels = Vec::with_capacity(120);

    for index in 0..60 {
        let offset = f64::from(index) / 60.0;
        features.push(vec![-2.0 - offset, -1.0 + offset / 4.0]);
        labels.push(0);
        features.push(vec![2.0 + offset, 1.0 - offset / 4.0]);
        labels.push(1);
    }

    (features, labels)
}

/// Trains the classifier and returns its accuracy on a reproducible test split.
///
/// # Errors
///
/// Returns an error if SmartCore cannot build the feature matrix, fit the
/// classifier, or predict the holdout labels.
fn holdout_accuracy() -> Result<f64, Box<dyn Error>> {
    let (features, labels) = synthetic_classification_data();
    let features = DenseMatrix::from_2d_vec(&features)?;
    let (x_train, x_test, y_train, y_test) =
        train_test_split(&features, &labels, 0.25, true, Some(42));

    let classifier =
        DecisionTreeClassifier::fit(&x_train, &y_train, Default::default())?;
    let predictions = classifier.predict(&x_test)?;

    Ok(accuracy(&y_test, &predictions))
}

/// Trains a decision-tree classifier and reports holdout accuracy.
fn main() -> Result<(), Box<dyn Error>> {
    let score = holdout_accuracy()?;
    println!("Holdout accuracy: {score:.2}");
    assert!(
        score >= 0.8,
        "expected at least 80% accuracy, got {score:.2}"
    );
    Ok(())
}
// ANCHOR_END: example

pub fn run() {
    main().expect("the SmartCore classification example failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the seeded holdout split yields accurate predictions.
    #[test]
    fn seeded_classifier_exceeds_accuracy_threshold()
    -> Result<(), Box<dyn Error>> {
        let score = holdout_accuracy()?;

        assert!(
            score >= 0.8,
            "expected at least 80% accuracy, got {score:.2}"
        );
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
