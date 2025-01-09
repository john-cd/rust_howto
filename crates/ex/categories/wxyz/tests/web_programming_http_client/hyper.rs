// // ANCHOR: example
// use hyper::Client;
// use hyper::Request;
// use hyper::body::Body;
// use hyper::http::response::Response;

// async fn fetch_url(url: &str) -> Result<Response<Body>, hyper::Error> {
//     let client = Client::new();

//     // Build the request
//     let req = Request::builder()
//         .method("GET")
//         .uri(url)
//         .header("User-Agent", "hyper")
//         .body(Body::empty())
//         .expect("Request::builder failed!");

//     // Send the request and wait for the response
//     let res = client.request(req).await?;

//     Ok(res)
// }

// #[tokio::main]
// async fn main() {
//     let url = "http://example.com";

//     match fetch_url(url).await {
//         Ok(response) => {
//             println!("Response: {:?}", response);
//         }
//         Err(e) => {
//             eprintln!("Error: {:?}", e);
//         }
//     }
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P0](https://github.com/john-cd/rust_howto/issues/859)
// // TODO https://hyper.rs/
