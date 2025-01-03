// // ANCHOR: example
// use hyper::Client;
// use hyper::Uri;
// use hyper_tls::HttpsConnector;
// use tokio::runtime::Runtime;

// fn main() {
//     // Create a new runtime
//     let rt = Runtime::new().unwrap();

//     rt.block_on(async {
//         // Create an HTTPS connector
//         let https = HttpsConnector::new();
//         let client = Client::builder().build::<_, hyper::Body>(https);

//         // Create a request to fetch data from an HTTPS endpoint
//         let uri: Uri = "https://www.example.com".parse().unwrap();
//         match client.get(uri).await {
//             Ok(response) => {
//                 println!("Response status: {}", response.status());
//             }
//             Err(e) => {
//                 eprintln!("Error: {}", e);
//             }
//         }
//     });
// }
// // ANCHOR_END: example

// #[test]
// #[ignore = "not yet implemented"]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/696)
