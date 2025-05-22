//#![cfg(feature = "polars")]
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
//! # Polars Data Processing Example
//!
//! This example demonstrates basic data processing operations using the Polars
//! library.
//!
//! In `Cargo.toml`, add:
//!
//! ```toml
//! [dependencies]
//! polars = "0.48.1"
//! ```
//!
//! `polars` has a large list of default features, therefore you may want to cherry-pick
//! required features:
//!
//! ```toml
//! polars = { version = "0.48.1", default-features = false, features = ["<what you need>"] }
//! ```

// use std::fs::File;

// use polars::prelude::*;

// fn main() -> Result<()> {
//     // Read a CSV file into a DataFrame.
//     let file = File::open("temp/data.csv")?;
//     let df = CsvReader::new(file)
//         .infer_schema(None)
//         .has_header(true)
//         .finish()?;

//     // Display the first few rows of the DataFrame.
//     println!("DataFrame:\n{}", df.head(Some(5)));

//     // Perform some data manipulation.
//     let df_filtered = df.filter(&df["some_column"].gt_eq(100))?;
//     println!("Filtered DataFrame:\n{}", df_filtered);

//     let df_selected = df_filtered.select(&["some_column",
// "another_column"])?;     println!("Selected Columns:\n{}", df_selected);

//     // Group by a column and aggregate.
//     let df_grouped = df_selected
//         .groupby("some_column")?
//         .agg(&[("another_column", &["sum", "mean"])])?;
//     println!("Grouped DataFrame:\n{}", df_grouped);

//     Ok(())
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/885)
