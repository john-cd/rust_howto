// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Sum a batch of matrices along the first axis of a rank-three tensor.

use ndarray::Array2;
use ndarray::Array3;
use ndarray::Axis;
use ndarray::array;

/// Sums corresponding cells across the batch axis.
fn sum_batches(batch: &Array3<i32>) -> Array2<i32> {
    batch.sum_axis(Axis(0))
}

/// Prints the cell-wise sum of two matrices.
fn main() {
    let batch = array![[[1, 2], [3, 4]], [[10, 20], [30, 40]]];
    let sum = sum_batches(&batch);
    println!("{sum}");
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Reducing the batch axis preserves the row and column axes.
    #[test]
    fn sums_corresponding_cells() {
        let batch = array![[[1, 2], [3, 4]], [[10, 20], [30, 40]]];
        assert_eq!(sum_batches(&batch), array![[11, 22], [33, 44]]);
    }
}
