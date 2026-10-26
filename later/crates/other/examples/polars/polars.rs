// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Example: filter and aggregate inline data with a lazy Polars query.
//!
//! The input is built in memory with `df!`. The lazy query filters rows,
//! groups by region, sums revenue, sorts the result, and collects it.

use polars::df;
use polars::prelude::DataFrame;
use polars::prelude::IntoLazy;
use polars::prelude::col;
use polars::prelude::lit;

/// Filters sales and returns sorted revenue totals by region.
fn summarize_sales() -> anyhow::Result<DataFrame> {
    let sales = df![
        "region" => ["north", "north", "south", "south", "north", "west"],
        "amount" => [120_i64, 80, 200, 50, 130, 10],
    ]?;

    let summary = sales
        .lazy()
        .filter(col("amount").gt_eq(lit(50)))
        .group_by([col("region")])
        .agg([col("amount").sum().alias("revenue")])
        .sort(["region"], Default::default())
        .collect()?;

    Ok(summary)
}

/// Runs the example and prints the grouped revenue totals.
fn main() -> anyhow::Result<()> {
    let summary = summarize_sales()?;
    println!("Revenue by region:\n{summary}");
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::summarize_sales;

    /// Verifies the lazy query filters, groups, and sorts its inline data.
    #[test]
    fn lazy_query_has_expected_shape_and_revenue() -> anyhow::Result<()> {
        let summary = summarize_sales()?;

        assert_eq!(summary.shape(), (2, 2));
        assert_eq!(summary.column("region")?.str()?.get(0), Some("north"));
        assert_eq!(summary.column("revenue")?.i64()?.get(0), Some(330));
        Ok(())
    }
}
