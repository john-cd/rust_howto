#![allow(dead_code)]
// ANCHOR: example
// // COMING SOON
// ANCHOR_END: example
//! This example shows how to create a simple gRPC server using the `tonic`
//! crate.
//!
//! Tonic is a gRPC over HTTP/2 library.
//!
//! The server implements the `Greeter` service, which has a single method
//! `say_hello`. The `say_hello` method takes a `HelloRequest` and returns a
//! `HelloReply`.
//!
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! tonic = <tonic-version>
//! prost = <prost-version>
//!
//! [build-dependencies]
//! tonic-build = <tonic-version>
//! ```

// Imports from the module created by `tonic` (see below):
// pub struct HelloReply { pub message: String, }
use helloworld::HelloReply;
// pub struct HelloRequest { pub name: String, }
use helloworld::HelloRequest;
// Generated trait containing gRPC methods that should be implemented for
// use with GreeterServer
use helloworld::greeter_server::Greeter;
// Wrapper around the Greeter implementation (below)
use helloworld::greeter_server::GreeterServer;
// A gRPC request and metadata from an RPC call:
use tonic::Request;
// A gRPC response and metadata from an RPC call:
use tonic::Response;
// A gRPC status describing the result of an RPC call:
use tonic::Status;
use tonic::transport::Server;

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = helloworld::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    Server::builder()
        // Create a router:
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
// Look for additional examples in https://github.com/hyperium/tonic/tree/master/examples

// #[test]
// fn require_network() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// [review; need to write full integration test, testing the endpoint then stopping the server](https://github.com/john-cd/rust_howto/issues/870)
