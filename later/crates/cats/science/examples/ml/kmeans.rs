// ANCHOR: example
//! Cluster two groups of synthetic points with seeded Linfa K-means.
//!
//! Machine learning learns patterns from data. K-means is unsupervised
//! clustering: it groups points into K clusters without labels, by repeatedly
//! moving K centres to the mean of their nearest points. Uses include customer
//! segmentation and image colour reduction.

use std::collections::BTreeSet;
use std::error::Error;

use linfa::dataset::DatasetBase;
use linfa::prelude::Predict as _;
use linfa::traits::Fit as _;
use linfa_clustering::KMeans;
use rand_08::SeedableRng as _;
use rand_08::rngs::StdRng;

/// Fits K-means to two separated groups and returns one label per point.
///
/// # Errors
///
/// Returns an error if K-means cannot fit the synthetic dataset.
fn cluster_synthetic_points() -> Result<Vec<usize>, Box<dyn Error>> {
    let records = ndarray_016::array![
        [0.0, 0.1],
        [0.1, 0.0],
        [0.2, 0.1],
        [0.1, 0.2],
        [9.8, 9.9],
        [9.9, 10.0],
        [10.0, 9.8],
        [10.1, 10.1],
    ];
    let dataset = DatasetBase::from(records);
    let model =
        KMeans::params_with_rng(2, StdRng::seed_from_u64(42)).fit(&dataset)?;

    Ok(model.predict(&dataset).to_vec())
}

/// Runs K-means on the synthetic points and reports the discovered clusters.
fn main() -> Result<(), Box<dyn Error>> {
    let labels = cluster_synthetic_points()?;
    let cluster_count = labels.iter().copied().collect::<BTreeSet<_>>().len();
    assert_eq!(labels.len(), 8, "each input point needs a cluster label");
    assert_eq!(cluster_count, 2, "the data contains two separated groups");
    println!("Cluster labels: {labels:?}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main().expect("the seeded Linfa K-means example failed");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies K-means assigns every point to one of two clusters.
    #[test]
    fn seeded_kmeans_labels_every_point_in_two_clusters()
    -> Result<(), Box<dyn Error>> {
        let labels = cluster_synthetic_points()?;
        let cluster_count =
            labels.iter().copied().collect::<BTreeSet<_>>().len();

        assert_eq!(labels.len(), 8);
        assert_eq!(cluster_count, 2);
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
