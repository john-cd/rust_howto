//! Uses an R-tree to find a nearby point without a network lookup.
//!
//! Finding the nearest point by checking every point is slow for large data
//! sets. An R-tree groups nearby points into bounding boxes, so a search skips
//! whole groups at once. Databases use the same idea for map queries.

// ANCHOR: example
use rstar::RTree;

/// Bulk-loads planar coordinates and finds the closest point.
fn main() {
    let tree = RTree::bulk_load(vec![[0.0_f64, 0.0], [2.0, 2.0], [8.0, 8.0]]);
    let nearest = tree
        .nearest_neighbor([1.8, 2.0])
        .expect("the tree contains three points");
    assert!((nearest[0] - 2.0).abs() < f64::EPSILON);
    assert!((nearest[1] - 2.0).abs() < f64::EPSILON);
    println!("Closest planar point: {nearest:?}");
}
// ANCHOR_END: example

/// Runs the in-memory spatial-index lookup.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    /// Selects the expected point from an indexed set of planar coordinates.
    #[test]
    fn selects_the_nearest_indexed_point() {
        super::main();
    }
}
