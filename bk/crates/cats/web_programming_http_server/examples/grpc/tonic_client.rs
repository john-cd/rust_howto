// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example shows how to create a simple gRPC client using the `tonic`
//! crate to interact with the `Greeter` server.
//!
//! To run this example, ensure the `Greeter` server is already running.

use helloworld::HelloRequest;
use helloworld::greeter_client::GreeterClient;

/// The module containing generated code from the proto file.
pub mod helloworld {
    tonic::include_proto!("helloworld");
}

/// Sends a request to the gRPC server and prints the reply.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to the gRPC server using the address defined in the server
    // example.
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

    /// Verifies a client can call the sibling gRPC server on an ephemeral port.
    #[tokio::test]
    async fn grpc_client_receives_greeting_and_shuts_down_server()
    -> anyhow::Result<()> {
        use tokio::net::TcpListener;
        use tokio::sync::oneshot;
        use tokio_stream::wrappers::TcpListenerStream;

        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let client_url = format!("http://{address}");
        let (shutdown_sender, shutdown_receiver) = oneshot::channel();
        let server = tokio::spawn(async move {
            tonic::transport::Server::builder()
                .add_service(
                    crate::tonic::helloworld::greeter_server::GreeterServer::new(
                        crate::tonic::MyGreeter::default(),
                    ),
                )
                .serve_with_incoming_shutdown(
                    TcpListenerStream::new(listener),
                    async move {
                        let _ = shutdown_receiver.await;
                    },
                )
                .await
        });

        let call_result = async {
            let mut client = GreeterClient::connect(client_url).await?;
            let request = tonic::Request::new(HelloRequest {
                name: "Integration".into(),
            });
            let response = client.say_hello(request).await?;
            anyhow::Result::<String>::Ok(response.into_inner().message)
        }
        .await;
        let shutdown_result = shutdown_sender.send(());
        let server_result = server.await;

        assert_eq!(call_result?, "Hello Integration!");
        shutdown_result.map_err(|()| {
            anyhow::anyhow!("gRPC server exited before shutdown")
        })?;
        server_result??;
        Ok(())
    }
}
