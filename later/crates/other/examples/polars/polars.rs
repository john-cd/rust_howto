#![allow(dead_code)]
// ANCHOR: example
//! # Polars Data Processing Example
//!
//! This example demonstrates basic data processing operations using the Polars
//! library.
//!
//! In `Cargo.toml`, add:
//!
//! ```toml
//! [dependencies]
//! polars = "0.49.1" # or latest
//! ```
//!
//! `polars` has a large list of default features, therefore we may want to
//! cherry-pick required features:
//!
//! ```toml
//! polars = { version = "0.49.1", default-features = false, features = ["lazy", "csv"] }
//! ```

use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    // Read a CSV file into a DataFrame.
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("temp/data.csv".into()))?
        .finish()?;

    // Display the first few rows of the DataFrame.
    println!("DataFrame:\n{}", df.head(Some(5)));

    // Perform some data manipulation.
    let df_filtered = df
        .lazy()
        .filter(col("some_column").gt_eq(lit(100)))
        .collect()?;
    println!("Filtered DataFrame:\n{df_filtered}");

    let df_selected = df_filtered
        .lazy()
        .select([col("some_column"), col("another_column")])
        .collect()?;
    println!("Selected Columns:\n{df_selected}");

    // Group by a column and aggregate.
    let df_grouped = df_selected
        .lazy()
        .group_by([col("some_column")])
        .agg([
            col("another_column").sum().alias("sum"),
            col("another_column").mean().alias("mean"),
        ])
        .collect()?;
    println!("Grouped DataFrame:\n{df_grouped}");

    Ok(())
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() -> anyhow::Result<()> {
        use std::fs;
        use std::io::Write;
        if !fs::exists("temp")? {
            fs::create_dir("temp")?;
        }
        let mut file = fs::File::create("temp/data.csv")?;
        writeln!(file, "some_column,another_column")?;
        writeln!(file, "150,10.5")?;
        writeln!(file, "50,5.0")?;
        writeln!(file, "120,20.0")?;
        writeln!(file, "150,15.5")?;

        main()?;
        Ok(())
    }
}
