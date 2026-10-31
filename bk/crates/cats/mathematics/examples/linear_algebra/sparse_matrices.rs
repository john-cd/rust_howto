// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates sparse matrices using the `sprs` crate.
//!
//! A matrix is *sparse* when almost all of its entries are zero. Think of a
//! social network: with a million users, each person follows a few hundred
//! others, so a million-by-million "who follows whom" table is nearly empty.
//! Storing every zero would waste memory (here, about 8 terabytes of `f64`s)
//! and multiplying by zero wastes time.
//!
//! A sparse format keeps only the non-zero values and where they are. The
//! *compressed sparse row* (CSR) format used below stores, for each row, the
//! column numbers and values of that row's non-zero entries. Typical uses
//! include graphs, finite-element and physics simulations, text-search
//! indexes and machine-learning features.

use sprs::CsMat;
use sprs::TriMat;
use sprs::prod::mul_acc_mat_vec_csr;

/// Builds a small sparse matrix and computes with it.
fn main() {
    // Collect `(row, column, value)` triplets in a `TriMat`, which is easy to
    // build up entry by entry. Missing entries are implicitly zero.
    let mut triplets = TriMat::new((4, 4));
    triplets.add_triplet(0, 0, 2.0_f64);
    triplets.add_triplet(0, 1, -1.0);
    triplets.add_triplet(1, 0, -1.0);
    triplets.add_triplet(1, 1, 2.0);
    triplets.add_triplet(1, 2, -1.0);
    triplets.add_triplet(2, 1, -1.0);
    triplets.add_triplet(2, 2, 2.0);
    triplets.add_triplet(2, 3, -1.0);
    triplets.add_triplet(3, 2, -1.0);
    triplets.add_triplet(3, 3, 2.0);

    // Convert to CSR, the format used for fast arithmetic.
    let a: CsMat<f64> = triplets.to_csr();
    println!(
        "{}x{} matrix with {} non-zero entries (density {:.0}%)",
        a.rows(),
        a.cols(),
        a.nnz(),
        a.density() * 100.0
    );

    // Read single entries; `get` returns `None` for a stored-zero position.
    println!("a[1][2] = {:?}", a.get(1, 2));
    println!("a[0][3] = {:?}", a.get(0, 3));

    // Iterate over the non-zero entries of each row.
    for (row, row_vec) in a.outer_iterator().enumerate() {
        let entries: Vec<(usize, f64)> =
            row_vec.iter().map(|(col, &value)| (col, value)).collect();
        println!("row {row}: {entries:?}");
    }

    // Multiply the sparse matrix by a dense vector. The product is added to
    // the output vector, which therefore starts at zero.
    let x = vec![1.0, 2.0, 3.0, 4.0];
    let mut y = vec![0.0; 4];
    mul_acc_mat_vec_csr(a.view(), x.as_slice(), y.as_mut_slice());
    println!("A * x = {y:?}");

    // Multiply two sparse matrices; the result is sparse too.
    let a_squared = &a * &a;
    println!("A * A has {} non-zero entries", a_squared.nnz());

    // Transpose, then convert back to CSR (a transposed view is CSC).
    let a_t = a.transpose_view().to_csr();
    println!("A is symmetric: {}", a_t == a);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds the tridiagonal matrix used by the example.
    fn tridiagonal() -> CsMat<f64> {
        let mut triplets = TriMat::new((4, 4));
        for i in 0..4 {
            triplets.add_triplet(i, i, 2.0_f64);
            if i + 1 < 4 {
                triplets.add_triplet(i, i + 1, -1.0);
                triplets.add_triplet(i + 1, i, -1.0);
            }
        }
        triplets.to_csr()
    }

    /// Runs the sparse matrices example through its entry point.
    #[test]
    fn test_sparse_matrices_example_entry_point() {
        main();
    }

    /// Only the non-zero entries are stored.
    #[test]
    fn stores_only_non_zero_entries() {
        let a = tridiagonal();
        assert_eq!(a.nnz(), 10);
        assert_eq!(a.get(1, 2), Some(&-1.0));
        assert_eq!(a.get(0, 3), None);
    }

    /// Sparse-by-dense multiplication matches the hand-computed result.
    #[test]
    fn multiplies_by_dense_vector() {
        let a = tridiagonal();
        let mut y = vec![0.0; 4];
        mul_acc_mat_vec_csr(
            a.view(),
            [1.0, 2.0, 3.0, 4.0].as_slice(),
            y.as_mut_slice(),
        );
        assert_eq!(y, vec![0.0, 0.0, 0.0, 5.0]);
    }

    /// The tridiagonal matrix is symmetric.
    #[test]
    fn is_symmetric() {
        let a = tridiagonal();
        assert_eq!(a.transpose_view().to_csr(), a);
    }
}
