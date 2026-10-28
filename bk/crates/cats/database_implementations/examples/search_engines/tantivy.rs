// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Create an in-memory Tantivy index and run a simple full-text search.

use tantivy::Index;
use tantivy::TantivyDocument;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::STORED;
use tantivy::schema::Schema;
use tantivy::schema::TEXT;
use tantivy::schema::document::Document as _;

/// Creates an in-memory index, adds documents to it and searches it.
fn main() -> tantivy::Result<()> {
    let mut schema_builder = Schema::builder();
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let body = schema_builder.add_text_field("body", TEXT | STORED);
    let schema = schema_builder.build();

    let index = Index::create_in_ram(schema.clone());
    let mut index_writer = index.writer(50_000_000)?;

    index_writer.add_document(doc!(
        title => "The Old Man and the Sea",
        body => "A short novel about persistence at sea"
    ))?;
    index_writer.add_document(doc!(
        title => "Rust Search",
        body => "Building a search index with Tantivy"
    ))?;
    index_writer.commit()?;

    let reader = index.reader()?;
    reader.reload()?;
    let searcher = reader.searcher();

    let query = QueryParser::for_index(&index, vec![title, body])
        .parse_query("search OR sea")?;
    let top_docs =
        searcher.search(&query, &TopDocs::with_limit(10).order_by_score())?;
    assert_eq!(top_docs.len(), 2);

    for (score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("score {score}: {}", retrieved_doc.to_json(&schema));
    }

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> tantivy::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the tantivy example through its entry point.
    #[test]
    fn test_tantivy_example_entry_point() -> tantivy::Result<()> {
        main()
    }
}
