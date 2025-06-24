#![allow(dead_code)]
// #![cfg(feature = "datafusion")]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to use DataFusion to set up a simple
// //! in-memory table and run a SQL query on it.
// //!
// //! DataFusion is an extensible query execution framework, written in Rust,
// //! that uses Apache Arrow as its in-memory format. This means it's very
// //! efficient for columnar data processing and can easily share data with
// //! other Arrow-compatible systems (like Python's PyArrow, Spark, etc.)
// //! without expensive serialization/deserialization.
// //!
// //! - SQL Support: You can query data registered within DataFusion using
// //! standard SQL.
// //! - DataFrame API: It also provides a programmatic DataFrame
// //! API (similar in concept to Pandas or Polars) for building query plans
// //! step-by-step in Rust code.
// //!
// //! The following creates a table named "people" with columns "name" and
// //! "age", inserts some data, and then runs a query to select people older
// //! than 35.

// use std::sync::Arc;

// use arrow::array::ArrayRef;
// use datafusion::arrow::datatypes::DataType;
// use datafusion::arrow::datatypes::Field;
// use datafusion::arrow::datatypes::Schema;
// use datafusion::arrow::record_batch::RecordBatch;
// use datafusion::datasource::MemTable;
// use datafusion::execution::context::SessionContext;
// use tokio::runtime::Runtime;

// #[tokio::main]
// async fn main() -> datafusion::error::Result<()> {
//     let runtime = Runtime::new()?;

//     // Create a simple schema and data.
//     let schema = Schema::new(vec![
//         Field::new("name", DataType::Utf8, false),
//         Field::new("age", DataType::Int32, false),
//     ]);

//     let data = vec![
//         vec!["Alice".to_string(), "30".to_string()],
//         vec!["Bob".to_string(), "40".to_string()],
//     ];

//     // Convert data into `RecordBatch`.
//     let record_batches = vec![RecordBatch::try_new(
//         Arc::new(schema.clone()),
//         vec![
//             Arc::new(data.iter().map(|r|
// r[0].clone()).collect::<ArrayRef>()),             Arc::new(
//                 data.iter()
//                     .map(|r| r[1].parse::<i32>().unwrap())
//                     .collect::<ArrayRef>(),
//             ),
//         ],
//     )?];

//     // Create a `MemoryTable` with the schema and data.
//     let table = MemTable::try_new(Arc::new(schema), vec![record_batches])?;

//     // Create a session context.
//     let mut ctx = SessionContext::new();

//     // Register the table.
//     ctx.register_table("people", Arc::new(table))?;

//     // Run a SQL query.
//     let df = ctx.sql("SELECT name, age FROM people WHERE age > 35")?;
//     let results = df.collect().await?;

//     // Print the results.
//     for batch in results {
//         println!("{:?}", batch);
//     }

//     Ok(())
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/883)
