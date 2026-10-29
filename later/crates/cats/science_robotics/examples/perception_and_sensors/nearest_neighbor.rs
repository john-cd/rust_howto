//! Finds the closest measurement in a small three-dimensional point cloud.

// ANCHOR: example
use anyhow::Result;
use kiddo::ImmutableKdTree;
use kiddo::SquaredEuclidean;

/// Builds a fixed point-cloud index and checks the nearest point and distance.
fn main() -> Result<()> {
    let points = [[0.0_f64, 0.0, 0.0], [2.0, 0.0, 0.0], [8.0, 8.0, 8.0]];
    let tree = ImmutableKdTree::new_from_slice(&points)?;
    let nearest = tree
        .query(&[2.25, 0.0, 0.0])
        .nearest_one::<SquaredEuclidean<f64>>()
        .execute();
    assert_eq!(nearest.item, 1);
    assert!((nearest.distance - 0.0625).abs() < f64::EPSILON);
    println!(
        "Nearest point index: {}, squared distance: {}",
        nearest.item, nearest.distance
    );
    Ok(())
}
// ANCHOR_END: example

/// Runs the nearest-neighbor lookup without a sensor or graphics device.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Selects the expected point and reports squared Euclidean distance.
    #[test]
    fn finds_the_nearest_point_in_three_dimensions() -> Result<()> {
        main()
    }
}
