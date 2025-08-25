#![allow(dead_code)]
// #![cfg(feature = "tantivy")]
// // ANCHOR: example
// use tantivy::DocAddress;
// use tantivy::Index;
// use tantivy::ReloadPolicy;
// use tantivy::Score;
// use tantivy::doc;
// use tantivy::schema::Document as TantivyDocument;
// use tantivy::schema::STORED;
// use tantivy::schema::Schema;
// use tantivy::schema::TEXT;

// /// `tantivy` is a Lucene-like full-text search engine library written in
// /// Rust.
// /// This example will demonstrate how to create an index, add documents,
// /// and perform a search.
// fn main() -> tantivy::Result<()> {
//     // Define the schema for your documents.
//     // Tantivy has a very strict schema. You need to specify in advance,
//     // whether a field is indexed or not, stored or not, and RAM-based or
//     // not: `let mut schema_builder = Schema::builder();`

//     schema_builder.add_text_field("title", TEXT | STORED);
//     // `TEXT` means the field should be tokenized and indexed,
//     // along with its term frequency and term positions.
//     // `STORED` means that the field will also be saved
//     // in a compressed, row-oriented key-value store.
//     // This store is useful to reconstruct the
//     // documents that were selected during the search phase.
//     schema_builder.add_text_field("body", TEXT);

//     // You may also use:
//     // add_u64_field, add_bool_field, add_date_field, add_ip_addr_field,
//     // add_facet_field, add_bytes_field, add_json_field... e.g.
//     // let num_stars_options =
//     //     NumericOptions::default().set_stored().set_indexed();
//     // schema_builder.add_u64_field("num_stars", num_stars_options);
//     // Or simpler: schema_builder.add_u64_field("num_stars", INDEXED |
//     // STORED);

//     let schema = schema_builder.build();

//     // Create a new index in the specified directory.
//     // This index will be allocated in anonymous memory. This is useful for
//     // indexing small set of documents for testing or for a temporary
//     // in-memory index.
//     let index = Index::create_in_ram(schema.clone());
//     // OR: let index = Index::create_in_dir(index_path, schema.clone())?;

//     // Create a multithreaded index writer, specify a buffer size in bytes:
//     let mut index_writer = index.writer(50_000_000)?;

//     // Add documents to the index.
//     index_writer.add_document(doc!(
//         schema.get_field("title").unwrap() => "Document 1",
//         schema.get_field("body").unwrap() => "This is the body of document
// 1", ))?;
//     index_writer.add_document(doc!(
//         schema.get_field("title").unwrap() => "Document 2",
//         schema.get_field("body").unwrap() => "This is the body of document
// 2", ))?;

//     // Commit the changes.
//     index_writer.commit()?;
//     // We need to call .commit() explicitly to force the
//     // index_writer to finish processing the documents in the queue,
//     // flush the current index to the disk, and advertise
//     // the existence of new documents.

//     // Create a reader.
//     let reader = index
//         .reader_builder()
//         .reload_policy(ReloadPolicy::OnCommitWithDelay)
//         // The index is reloaded within milliseconds after a new commit is
// available.         .try_into()?;
//     // OR: let reader = index.reader()?;

//     // A searcher points to a snapshotted, immutable version of the index.
//     // You will typically create one reader for the entire lifetime of your
//     // program, and acquire a new searcher for every single request.
//     let searcher = reader.searcher();

//     // Define a query parser that can interpret human queries:
//     let query_parser = tantivy::query::QueryParser::for_index(
//         &index,
//         vec![schema.get_field("body").unwrap()], /* Set of default fields
//                                                   * used to search if no
//                                                   * field is specifically
//                                                   * defined in the query. */
//     );

//     // Parse the query coming e.g. from the search bar.
//     let query = query_parser.parse_query("body:document")?;

//     // Search for documents that match the query:
//     let top_docs: Vec<(Score, DocAddress)> = searcher
//         .search(&query, &tantivy::collector::TopDocs::with_limit(10))?;

//     // Print the search results:
//     for (score, doc_address) in top_docs {
//         // Retrieve the actual content of documents given its `doc_address`.
//         let retrieved_doc = searcher.doc(doc_address)?;
//         println!(
//             "Document found: {:?}, score: {}",
//             retrieved_doc.to_json(&schema),
//             score
//         );
//         // We can also get an explanation to understand how a found document
//         // got its score.
//         let explanation = query.explain(&searcher, doc_address)?;
//         println!("{}", explanation.to_pretty_json());
//     }

//     // Delete all documents:
//     index_writer.delete_all_documents()?;
//     index_writer.commit()?;

//     Ok(())
// }
// // Adapted from <https://docs.rs/tantivy/0.22.0/>.
// // See also examples e.g. <https://tantivy-search.github.io/examples/basic_search.html>.
// // ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
