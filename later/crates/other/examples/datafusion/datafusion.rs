#![allow(dead_code)]
#![cfg(feature = "datafusion")]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example registers an in-memory Arrow table and groups its rows with
//! both SQL and DataFusion's `DataFrame` API.
//!
//! `DataFusion` is an extensible query execution framework, written in Rust,
//! that uses Apache Arrow as its in-memory format. This means it's very
//! efficient for columnar data processing and can easily share data with
//! other Arrow-compatible systems (like Python's `PyArrow`, Spark, etc.)
//! without expensive serialization/deserialization.
//!
//! - SQL Support: We can query data registered within `DataFusion` using
//!   standard SQL.
//! - `DataFrame` API: It also provides a programmatic `DataFrame` API (similar
//!   in concept to Pandas or Polars) for building query plans step-by-step in
//!   Rust code.
//!
//! Both query APIs calculate the same total amount for each department.

use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use datafusion::arrow::array::ArrayRef;
use datafusion::arrow::array::Int64Array;
use datafusion::arrow::array::StringArray;
use datafusion::arrow::datatypes::DataType;
use datafusion::arrow::datatypes::Field;
use datafusion::arrow::datatypes::Schema;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::datasource::MemTable;
use datafusion::execution::context::SessionContext;
use datafusion::functions_aggregate::expr_fn::sum;
use datafusion::prelude::col;

/// Registers an in-memory table and compares SQL and DataFrame aggregations.
async fn run_query() -> Result<()> {
    let schema = Schema::new(vec![
        Field::new("department", DataType::Utf8, false),
        Field::new("amount", DataType::Int64, false),
    ]);
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![
            Arc::new(StringArray::from(vec![
                "engineering",
                "engineering",
                "sales",
            ])) as ArrayRef,
            Arc::new(Int64Array::from(vec![10, 20, 5])) as ArrayRef,
        ],
    )?;

    let table = MemTable::try_new(Arc::new(schema), vec![vec![batch]])?;
    let context = SessionContext::new();
    context.register_table("sales", Arc::new(table))?;

    let sql_batches = context
        .sql(
            "SELECT department, SUM(amount) AS total_amount \
             FROM sales GROUP BY department ORDER BY department",
        )
        .await?
        .collect()
        .await?;
    assert_grouped_totals(&sql_batches)?;

    let dataframe_batches = context
        .table("sales")
        .await?
        .aggregate(
            vec![col("department")],
            vec![sum(col("amount")).alias("total_amount")],
        )?
        .collect()
        .await?;
    assert_grouped_totals(&dataframe_batches)?;

    Ok(())
}

/// Checks the department totals returned by either query API.
fn assert_grouped_totals(batches: &[RecordBatch]) -> Result<()> {
    let mut totals = Vec::new();
    for batch in batches {
        let departments = batch
            .column(0)
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                anyhow!("query returned a non-string department column")
            })?;
        let amounts = batch
            .column(1)
            .as_any()
            .downcast_ref::<Int64Array>()
            .ok_or_else(|| {
                anyhow!("query returned a non-integer total column")
            })?;
        for row in 0..batch.num_rows() {
            totals
                .push((departments.value(row).to_owned(), amounts.value(row)));
        }
    }
    totals.sort_unstable();
    assert_eq!(
        totals,
        vec![("engineering".to_owned(), 30), ("sales".to_owned(), 5)]
    );
    Ok(())
}

/// Runs the asynchronous queries on a single-threaded Tokio runtime.
fn main() -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .build()?
        .block_on(run_query())
}
// ANCHOR_END: example

/// Runs the example and propagates query or runtime errors.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that both DataFusion APIs return the grouped totals.
    #[test]
    fn grouped_queries_return_the_same_totals() -> Result<()> {
        main()
    }
}
