extern crate nalgebra;
extern crate serde_json;

use nalgebra::DMatrix;

fn main() -> Result<(), std::io::Error> {
    let row_slice: Vec<i32> = (1..5001).collect();
    let matrix = DMatrix::from_row_slice(50, 100, &row_slice);

    // serialize matrix
    let serialized_matrix = serde_json::to_string(&matrix)?;

    // deserialize matrix
    let deserialized_matrix: DMatrix<i32> =
        serde_json::from_str(&serialized_matrix)?;

    // verify that `deserialized_matrix` is equal to `matrix`
    assert!(deserialized_matrix == matrix);

    Ok(())
}
