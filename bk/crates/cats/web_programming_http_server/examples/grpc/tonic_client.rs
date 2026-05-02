#![allow(dead_code)]
// ANCHOR: example
//! This example shows how to create a simple gRPC client using the `tonic`
//! crate to interact with the `Greeter` server.
//!
//! To run this example, ensure the `Greeter` server is already running.

use helloworld::greeter_client::GreeterClient;
use helloworld::HelloRequest;

/// The module containing generated code from the proto file.
pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to the gRPC server using the address defined in the server example.
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // Prepare a request containing the name to be greeted.
    let request = tonic::Request::new(HelloRequest {
        name: "Rustacean".into(),
    });

    // Execute the `say_hello` remote procedure call.
    let response = client.say_hello(request).await?;

    println!("Greeting from server: {}", response.into_inner().message);

    Ok(())
}
// ANCHOR_END: example

/// Boilerplate to expose the example logic to the crate's main harness.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Full-stack integration test that spawns the server from the sibling `tonic`
    /// module and connects to it using the client.
    #[tokio::test]
    async fn test_grpc_full_stack() -> anyhow::Result<()> {
        let port = 50052;
        let addr = format!("[::1]:{port}").parse().unwrap();
        let client_url = format!("http://[::1]:{port}");

        // 1. Spawn the server logic from the sibling module in the background
        tokio::spawn(async move {
            let greeter = crate::tonic::MyGreeter::default();
            tonic::transport::Server::builder()
                .add_service(crate::tonic::helloworld::greeter_server::GreeterServer::new(greeter))
                .serve(addr)
                .await
                .unwrap();
        });

        // 2. Wait briefly for the server to start listening
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // 3. Run client logic against the test server
        let mut client = GreeterClient::connect(client_url).await?;
        let request = tonic::Request::new(HelloRequest { name: "Integration".into() });
        let response = client.say_hello(request).await?;

        assert_eq!(response.into_inner().message, "Hello Integration!");
        Ok(())
    }
}
