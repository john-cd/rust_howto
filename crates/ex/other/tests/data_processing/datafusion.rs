// // ANCHOR: example
// // ANCHOR_END: example

// // use datafusion::prelude::*;
// use datafusion::arrow::datatypes::{DataType, Field, Schema};
// use datafusion::arrow::record_batch::RecordBatch;
// use datafusion::execution::context::SessionContext;
// use tokio::runtime::Runtime;
// use std::sync::Arc;
// use arrow::array::ArrayRef;
// use datafusion::datasource::MemTable;

// // Set up a simple in-memory table and runs a SQL query on it.

// #[tokio::main]
// async fn main() -> datafusion::error::Result<()> {
//     let runtime = Runtime::new()?;

//     // Create a simple schema and data
//     let schema = Schema::new(vec![
//         Field::new("name", DataType::Utf8, false),
//         Field::new("age", DataType::Int32, false),
//     ]);

//     let data = vec![
//         vec!["Alice".to_string(), "30".to_string()],
//         vec!["Bob".to_string(), "40".to_string()],
//     ];

//     // Convert data into RecordBatch
//     let record_batches = vec![RecordBatch::try_new(
//         Arc::new(schema.clone()),
//         vec![
//             Arc::new(data.iter().map(|r|
// r[0].clone()).collect::<ArrayRef>()),
// Arc::new(data.iter().map(|r|
// r[1].parse::<i32>().unwrap()).collect::<ArrayRef>()),         ],
//     )?];

//     // Create a MemoryTable with the schema and data
//     let table = MemTable::try_new(Arc::new(schema), vec![record_batches])?;

//     // Create a session context
//     let mut ctx = SessionContext::new();

//     // Register the table
//     ctx.register_table("people", Arc::new(table))?;

//     // Run a SQL query
//     let df = ctx.sql("SELECT name, age FROM people WHERE age > 35")?;
//     let results = df.collect().await?;

//     // Print the results
//     for batch in results {
//         println!("{:?}", batch);
//     }

//     Ok(())
// }

// #[test]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/883)
