#![allow(dead_code)]
// ANCHOR: example
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

// Tonic is a gRPC over HTTP/2 library.

// Add the following to your `Cargo.toml`:
// [dependencies]
// tonic = <tonic-version>
// prost = <prost-version>
//
// [build-dependencies]
// tonic-build = <tonic-version>

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
        // Create a router
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
// Look for additional examples in https://github.com/hyperium/tonic/tree/master/examples
// ANCHOR_END: example

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// [P1](https://github.com/john-cd/rust_howto/issues/870) need to write full integration test, testing the endpoint then stopping the server
