// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example: query inline sales data with Polars and join regional targets.
//!
//! The example builds DataFrames with `df!`, runs a lazy filter, projection,
//! aggregation, and sort, then joins the totals to an inline target table.

use polars::df;
use polars::prelude::DataFrame;
use polars::prelude::IntoLazy;
use polars::prelude::JoinArgs;
use polars::prelude::JoinType;
use polars::prelude::col;
use polars::prelude::lit;

/// Filters sales, aggregates revenue by region, and joins regional targets.
fn summarize_and_join_sales() -> anyhow::Result<(DataFrame, DataFrame)> {
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

    let targets = df![
        "region" => ["north", "south", "west"],
        "target" => [300_i64, 260, 100],
    ]?;

    let joined = summary
        .clone()
        .lazy()
        .join(
            targets.lazy(),
            [col("region")],
            [col("region")],
            JoinArgs::new(JoinType::Inner),
        )
        .sort(["region"], Default::default())
        .collect()?;

    Ok((summary, joined))
}

/// Runs the example and prints the aggregate and joined data frames.
///
/// # Errors
///
/// Returns an error if Polars cannot evaluate the query.
fn main() -> anyhow::Result<()> {
    let (summary, joined) = summarize_and_join_sales()?;
    println!("Revenue by region:\n{summary}\nJoined with targets:\n{joined}");
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
    use super::summarize_and_join_sales;

    /// Verifies the aggregation values and joined row count for inline data.
    #[test]
    fn aggregate_and_join_return_expected_results() -> anyhow::Result<()> {
        let (summary, joined) = summarize_and_join_sales()?;

        assert_eq!(summary.shape(), (2, 2));
        assert_eq!(summary.column("revenue")?.i64()?.get(0), Some(330));
        assert_eq!(joined.height(), 2);
        assert_eq!(joined.column("target")?.i64()?.get(0), Some(300));
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
