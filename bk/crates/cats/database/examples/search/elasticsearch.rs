#![allow(dead_code)]
// ANCHOR: example
//! This module provides examples of interacting with Elasticsearch using the
//! official Rust client.
use elasticsearch::BulkParts;
use elasticsearch::Elasticsearch;
use elasticsearch::Error;
use elasticsearch::IndexParts;
use elasticsearch::SearchParts;
use elasticsearch::cat::CatIndicesParts;
use elasticsearch::http::request::JsonBody;
use elasticsearch::http::response::Response;
use elasticsearch::http::transport::Transport;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use serde_json::json;

/// Represents a document that can be indexed in Elasticsearch.
#[derive(Debug, Serialize, Deserialize)]
struct MyDocument {
    id: i32,
    title: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create an asynchronous Elasticsearch client
    let url = std::env::var("ELASTIC_URL")
        .unwrap_or_else(|_| "http://localhost:9200".into());
    let transport = Transport::single_node(&url)?;
    let client = Elasticsearch::new(transport);
    // OR: let client = Elasticsearch::default();

    let response: Response = index_document(&client).await?;
    println!("Index response: {:?}", response);

    let search_result: serde_json::Value = search_document(&client).await?;
    println!("Search result: {:?}", search_result);

    let bulk_response: bool = bulk_documents(&client).await?;
    println!("Bulk response: {:?}", bulk_response);

    cat_indices(client).await?;

    Ok(())
}

/// Indexes a single document into Elasticsearch.
async fn index_document(client: &Elasticsearch) -> Result<Response, Error> {
    // Define a document to index
    let doc = MyDocument {
        id: 1,
        title: "Rust with Elasticsearch".to_string(),
    };
    // Index the document
    let response = client
        .index(IndexParts::IndexId("my_index", "1"))
        .body(json!(doc))
        .send()
        .await?;
    Ok(response)
}

/// Searches for documents in Elasticsearch based on a query.
async fn search_document(
    client: &Elasticsearch,
) -> Result<serde_json::Value, Error> {
    // Search for the document
    let search_response = client
        .search(SearchParts::Index(&["my_index"]))
        .from(0)
        .size(10)
        .body(json!({
            "query": {
                "match": {
                    "title": "Rust"
                }
            }
        }))
        .send()
        .await?;
    let search_result = search_response.json::<serde_json::Value>().await?;
    Ok(search_result)
}

/// Performs bulk indexing of multiple documents into Elasticsearch.
async fn bulk_documents(client: &Elasticsearch) -> anyhow::Result<bool> {
    let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4);

    // Add the first operation and document
    body.push(json!({"index": {"_id": "1"}}).into());
    body.push(
        json!({
            "id": 1,
            "user": "user1",
            "post_date": "2025-02-06T00:00:00Z",
            "message": "Trying out Elasticsearch"
        })
        .into(),
    );

    // Add the second operation and document
    body.push(json!({"index": {"_id": "2"}}).into());
    body.push(
        json!({
            "id": 2,
            "user": "user2",
            "post_date": "2025-02-07T00:00:00Z",
            "message": "Bulk indexing with the rust client!"
        })
        .into(),
    );

    let response = client
        .bulk(BulkParts::Index("tweets"))
        .body(body)
        .send()
        .await?;

    let response_body = response.json::<Value>().await?;
    let successful =
        !(response_body["errors"].as_bool().ok_or(anyhow::anyhow!(
            "bulk_documents: response could not be converted to bool"
        ))?);

    Ok(successful)
}

/// Retrieves and displays information about indices in Elasticsearch.
async fn cat_indices(client: Elasticsearch) -> Result<Response, Error> {
    // Call the `Cat` related APIs.
    let response = client
        .cat()
        .indices(CatIndicesParts::Index(&["*"]))
        .format("json")
        .send()
        .await?;
    Ok(response)
}
// ANCHOR_END: example

/// Test function that requires an external Elasticsearch service.
#[test]
fn require_external_svc() -> anyhow::Result<()> {
    unsafe {
        // Refer to the compose*.yaml files
        std::env::set_var(
            "ELASTIC_URL",
            "http://rust_howto_dev-elasticsearch-1:9200",
        );
    }
    main()?;
    Ok(())
}
// [review fix heavy test; secure the connection](https://github.com/john-cd/rust_howto/issues/710)
