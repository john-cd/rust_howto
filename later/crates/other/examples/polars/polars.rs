// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Example: aggregate sales by region with a Polars lazy query.
//!
//! The example builds DataFrames with `df!`, runs a lazy filter, projection,
//! aggregation, and sort over inline data.

use polars::df;
use polars::prelude::DataFrame;
use polars::prelude::IntoLazy;
use polars::prelude::col;
use polars::prelude::lit;

/// Filters sales and aggregates the remaining revenue by region.
fn summarize_sales() -> anyhow::Result<DataFrame> {
    let sales = df![
        "region" => ["north", "north", "south", "south", "north", "west"],
        "product" => ["tea", "coffee", "tea", "coffee", "tea", "coffee"],
        "amount" => [120_i64, 80, 200, 50, 130, 10],
    ]?;

    let summary = sales
        .lazy()
        .filter(col("amount").gt_eq(lit(50)))
        .select([col("region"), col("amount")])
        .group_by([col("region")])
        .agg([col("amount").sum().alias("revenue")])
        .sort(["region"], Default::default())
        .collect()?;

    Ok(summary)
}

/// Runs the example and prints the regional revenue totals.
///
/// # Errors
///
/// Returns an error if Polars cannot evaluate the query.
fn main() -> anyhow::Result<()> {
    let summary = summarize_sales()?;
    println!("Revenue by region:\n{summary}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the Polars example.
///
/// # Errors
///
/// Returns an error if Polars cannot evaluate the query.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::summarize_sales;

    /// Verifies the lazy query's result shape and expected regional revenue.
    #[test]
    fn lazy_query_returns_expected_regional_revenue() -> anyhow::Result<()> {
        let summary = summarize_sales()?;

        assert_eq!(summary.shape(), (2, 2));
        assert_eq!(summary.column("revenue")?.i64()?.get(0), Some(330));
        Ok(())
    }
}
