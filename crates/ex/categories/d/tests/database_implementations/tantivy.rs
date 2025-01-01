// // ANCHOR: example

// // Here’s a basic example of how to use the tantivy crate, which is a
// full-text search engine library written in Rust. This example will
// demonstrate how to create an index, add documents, and perform a search.

// use tantivy::schema::*;
// use tantivy::{doc, Index, ReloadPolicy};

// fn main() -> tantivy::Result<()> {
//     // Define the schema for your documents
//     let mut schema_builder = Schema::builder();
//     schema_builder.add_text_field("title", TEXT | STORED);
//     schema_builder.add_text_field("body", TEXT);
//     let schema = schema_builder.build();

//     // Create a new index in the specified directory
//     let index = Index::create_in_ram(schema.clone());

//     // Create an index writer with a buffer size
//     let mut index_writer = index.writer(50_000_000)?;

//     // Add documents to the index
//     index_writer.add_document(doc!(
//         schema.get_field("title").unwrap() => "Document 1",
//         schema.get_field("body").unwrap() => "This is the body of document
// 1",     ));
//     index_writer.add_document(doc!(
//         schema.get_field("title").unwrap() => "Document 2",
//         schema.get_field("body").unwrap() => "This is the body of document
// 2",     ));

//     // Commit the changes
//     index_writer.commit()?;

//     // Create a searcher
//     let reader = index
//         .reader_builder()
//         .reload_policy(ReloadPolicy::OnCommit)
//         .try_into()?;
//     let searcher = reader.searcher();

//     // Define a query parser
//     let query_parser = tantivy::query::QueryParser::for_index(&index,
// vec![schema.get_field("body").unwrap()]);

//     // Parse the query
//     let query = query_parser.parse_query("body:document")?;

//     // Search for documents that match the query
//     let top_docs = searcher.search(&query,
// &tantivy::collector::TopDocs::with_limit(10))?;

//     // Print the search results
//     for (score, doc_address) in top_docs {
//         let retrieved_doc = searcher.doc(doc_address)?;
//         println!("Document found: {:?}, score: {}",
// schema.to_json(&retrieved_doc), score);     }

//     Ok(())
// }

// // ANCHOR_END: example

// #[test]
// #[ignore = "not yet implemented"]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/724)
