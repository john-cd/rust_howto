#![allow(dead_code)]
// ANCHOR: example
use std::env;

use meilisearch_sdk::client::Client;
use meilisearch_sdk::indexes::Index;
use meilisearch_sdk::search::SearchResults;
use meilisearch_sdk::task_info::TaskInfo;
use meilisearch_sdk::tasks::Task;
use serde::Deserialize;
use serde::Serialize;

/// A document that can be indexed in MeiliSearch.
#[derive(Debug, Serialize, Deserialize)]
struct MyDocument {
    id: usize,
    title: String,
    content: String,
}

/// This is an example of how to use the MeiliSearch Rust SDK to index and
/// search documents.
///
/// It requires a running MeiliSearch instance.
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let meilisearch_url =
        env::var("MEILISEARCH_URL").unwrap_or("http://localhost:7700".into());
    let meilisearch_api_key: Option<String> = env::var("MEILI_MASTER_KEY").ok();

    // Create a client and connect to MeiliSearch.
    let client = Client::new(meilisearch_url, meilisearch_api_key).unwrap();

    // Create an index.
    let index: Index = client.index("my_index");

    // Define a list of documents to index.
    let docs = vec![
        MyDocument {
            id: 1,
            title: "First Document".to_string(),
            content: "This is the first document content.".to_string(),
        },
        MyDocument {
            id: 2,
            title: "Second Document".to_string(),
            content: "This is the second document content.".to_string(),
        },
        MyDocument {
            id: 3,
            title: "Rust Programming".to_string(),
            content: "Learning the Rust programming language.".to_string(),
        },
    ];

    // Index the documents.
    // If the index does not exist, Meilisearch creates it when you first add
    // the documents.
    let task: TaskInfo = index.add_documents(&docs, Some("id")).await?;
    println!("Indexing task: {task:?}");

    // Wait for the indexing task to complete.
    let status = index.wait_for_task(task, None, None).await?;
    println!("Indexing status: {status:?}");
    assert!(matches!(status, Task::Succeeded { .. }));

    // Perform a search query.
    let query = "Rust";
    let search_results: SearchResults<MyDocument> = index
        .search()
        .with_query(query)
        .execute::<MyDocument>()
        .await?;

    // Print the search results.
    println!("Search results for '{query}':");
    for hit in search_results.hits {
        println!("{:?}", hit.result);
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_external_svc() -> anyhow::Result<()> {
    unsafe {
        // Refer to the compose*.yaml files.
        std::env::set_var(
            "MEILISEARCH_URL",
            "http://rust_howto_dev-meilisearch-1:7700",
        );
    }
    main()?;
    Ok(())
}
