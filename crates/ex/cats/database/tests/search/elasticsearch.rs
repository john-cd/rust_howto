// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use elasticsearch::Elasticsearch;
// use elasticsearch::Error;
// use elasticsearch::IndexParts;
// use elasticsearch::SearchParts;
// use elasticsearch::cat::CatIndicesParts;
// use elasticsearch::http::transport::Transport;
// use serde::Deserialize;
// use serde::Serialize;
// use serde_json::json;
// use tokio;

// #[derive(Debug, Serialize, Deserialize)]
// struct MyDocument {
//     id: i32,
//     title: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     // Create an asynchronous Elasticsearch client
//     let transport = Transport::single_node("http://localhost:9200")?;
//     let client = Elasticsearch::new(transport);
//     // OR: let client = Elasticsearch::default();

//     let response = index_document(client).await?;
//     println!("Index response: {:?}", response);

//     let search_result = search_document(client).await?;
//     println!("Search result: {:?}", search_result);

//     let bulk_response = bulk_documents(client).await?;
//     println!("Bulk response: {:?}", bulk_response);

//     cat_indices(client).await?;

//     Ok(())
// }

// async fn index_document(client: Elasticsearch) -> Result<Response, Error> {
//     // Define a document to index
//     let doc = MyDocument {
//         id: 1,
//         title: "Rust with Elasticsearch".to_string(),
//     };
//     // Index the document
//     let response = client
//         .index(IndexParts::IndexId("my_index", "1"))
//         .body(json!(doc)) // Convert the document to JSON using
// serde_json::json!.         .send()
//         .await?;
//     Ok(response)
// }

// async fn search_document(
//     client: Elasticsearch,
// ) -> Result<serde_json::Value, Error> {
//     // Search for the document
//     let search_response = client
//         .search(SearchParts::Index(&["my_index"]))
//         .from(0)
//         .size(10)
//         .body(json!({
//             "query": {
//                 "match": {
//                     "title": "Rust"
//                 }
//             }
//         }))
//         .send()
//         .await?;
//     let search_result = search_response.json::<serde_json::Value>().await?;
//     Ok(search_result)
// }

// // Interact with an Elasticsearch client to perform bulk indexing operations.
// async fn bulk_documents(client: Elasticsearch) -> Result<bool, Error> {
//     let mut body: Vec<JsonBody<_>> = Vec::with_capacity(4);

//     // Add the first operation and document
//     body.push(json!({"index": {"_id": "1"}}).into());
//     body.push(
//         json!({
//             "id": 1,
//             "user": "kimchy",
//             "post_date": "2009-11-15T00:00:00Z",
//             "message": "Trying out Elasticsearch"
//         })
//         .into(),
//     );

//     // Add the second operation and document
//     body.push(json!({"index": {"_id": "2"}}).into());
//     body.push(
//         json!({
//             "id": 2,
//             "user": "forloop",
//             "post_date": "2020-01-08T00:00:00Z",
//             "message": "Bulk indexing with the rust client, yeah!"
//         })
//         .into(),
//     );

//     let response = client
//         .bulk(BulkParts::Index("tweets"))
//         .body(body)
//         .send()
//         .await?;

//     let response_body = response.json::<Value>().await?;
//     let successful = !response_body["errors"].as_bool()?;

//     Ok(successful)
// }

// async fn cat_indices(client: Elasticsearch) -> Result<(), Error> {
//     // Call the `Cat` related APIs.
//     let response = client
//         .cat()
//         .indices(CatIndicesParts::Index(&["*"]))
//         .format("json")
//         .send()
//         .await?;
//     Ok(())
// }

// // #[tokio::test]
// // async fn test() -> Result<(), Box<dyn std::error::Error>> {
// //     // your code ...
// //     Ok(())
// // }

// #[test]
// fn require_external_svc() {
//     main();
// }
// // [P0](https://github.com/john-cd/rust_howto/issues/710)
