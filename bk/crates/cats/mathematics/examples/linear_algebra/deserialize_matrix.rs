#![allow(dead_code)]
// ANCHOR: example
use nalgebra::DMatrix;

/// This example demonstrates how to serialize and deserialize a `DMatrix` using
/// `serde_json`.
///
/// It creates a 50x100 matrix, serializes it to a JSON string, deserializes it
/// back, and then verifies that the deserialized matrix is equal to the
/// original matrix.
fn main() -> Result<(), std::io::Error> {
    let row_slice: Vec<i32> = (1..5001).collect();
    let matrix = DMatrix::from_row_slice(50, 100, &row_slice);
    println!("{matrix}");

    // Serialize the matrix.
    let serialized_matrix = serde_json::to_string(&matrix)?;

    // Deserialize the matrix.
    let deserialized_matrix: DMatrix<i32> =
        serde_json::from_str(&serialized_matrix)?;

    // Verify that `deserialized_matrix` is equal to `matrix`.
    assert!(deserialized_matrix == matrix);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
