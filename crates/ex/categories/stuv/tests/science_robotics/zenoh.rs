// // ANCHOR: example
// use std::sync::Arc;

// use zenoh::Config;
// use zenoh::Error;
// use zenoh::Result;
// use zenoh::Session;
// // ZBytes & encoding
// use zenoh::bytes::{Encoding, ZBytes};
// use zenoh::config::*;
// // key_expr & selector
// use zenoh::key_expr::{KeyExpr, OwnedKeyExpr, keyexpr};
// // publisher & subscriber
// use zenoh::pubsub::{Publisher, Subscriber};
// // quality of service
// use zenoh::qos::{CongestionControl, Priority};
// // query & queryable & selectors
// use zenoh::query::{
//     ConsolidationMode, Parameters, Query, QueryConsolidation, QueryTarget,
//     Queryable, Reply, Selector,
// };
// // sample
// use zenoh::sample::Sample;
// // session
// use zenoh::session::Session;



// // Demonstrate basic publish-subscribe functionality with Zenoh.
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     // Create a Zenoh session
//     let config = Config::default();
//     let session = zenoh::open(config).await?;

//     // Publish a value
//     let key = KeyExpr::from("my/topic");
//     let value = Value::from("Hello, Zenoh!");
//     session.put(&key, &value).await?;
//     // or: session.put("key/expression", "value").await?;
//     println!("Published: {}", value);

//     // Subscribe to a topic
//     let key_expr = KeyExpr::from("my/topic");
//     let subscriber = session.declare_subscriber(&key_expr).await?;

//     // Receive and print incoming messages
//     loop {
//         match subscriber.recv_async().await {
//             Ok(sample) => {
//                 println!("Received: {:?}", sample);
//             }
//             Err(e) => {
//                 eprintln!("Error: {}", e);
//                 break;
//             }
//         }
//     }

//     session.close().await?;
//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/845)
// // TODO https://github.com/eclipse-zenoh/zenoh/tree/main/examples
// // https://drive.google.com/file/d/1ETSLz2ouJ2o9OpVvEoXrbGcCvpF4TwJy/view
// // compare to MQTT (Message Queueing Telemetry Transport) and DDS (Data
// // Distribution Service)
