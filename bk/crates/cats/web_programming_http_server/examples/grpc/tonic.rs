#![allow(dead_code)]
// ANCHOR: example
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
//! tonic = "0.12"
//! prost = "0.13"
//!
//! [build-dependencies]
//! tonic-build = "0.12"
//! ```

// Imports from the module created by `tonic` (see below):
use helloworld::HelloReply;
use helloworld::HelloRequest;
use helloworld::greeter_server::Greeter;
use helloworld::greeter_server::GreeterServer;
// A gRPC request and metadata from an RPC call:
use tonic::Request;
// A gRPC response and metadata from an RPC call:
use tonic::Response;
// A gRPC status describing the result of an RPC call:
use tonic::Status;
use tonic::transport::Server;

/// The module containing generated code from the proto file.
pub mod helloworld {
    tonic::include_proto!("helloworld");
}

/// Greeter service implementation.
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

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        // Create a router:
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the greeting logic by calling the service implementation
    /// directly.
    #[tokio::test]
    async fn test_greeter_logic() {
        let greeter = MyGreeter::default();
        let request = Request::new(HelloRequest {
            name: "Rustacean".to_string(),
        });

        let response = greeter
            .say_hello(request)
            .await
            .expect("gRPC method failed");
        assert_eq!(response.into_inner().message, "Hello Rustacean!");
    }
}
