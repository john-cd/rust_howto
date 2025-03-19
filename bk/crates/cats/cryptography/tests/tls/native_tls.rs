// // ANCHOR: example
// use hyper::Uri;
// use hyper::client;
// use hyper_tls::HttpsConnector;
// use tokio::runtime::Runtime;

// fn main() -> anyhow::Result<()> {
//     // Create a new runtime
//     let rt = Runtime::new().unwrap();

//     rt.block_on(async {
//         // Create an HTTPS connector
//         let https = HttpsConnector::new();
//         let client = Client::builder().build::<_, hyper::body::Body>(https);

//         // Create a request to fetch data from an HTTPS endpoint
//         let uri: Uri = "https://www.example.com".parse()?;
//         match client.get(uri).await {
//             Ok(response) => {
//                 println!("Response status: {}", response.status());
//             }
//             Err(e) => {
//                 eprintln!("Error: {}", e);
//             }
//         }
//     });
//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [WIP finish](https://github.com/john-cd/rust_howto/issues/696)
