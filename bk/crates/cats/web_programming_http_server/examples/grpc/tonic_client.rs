// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example shows how to create a simple gRPC client using the `tonic`
//! crate to interact with the `Greeter` server.
//!
//! To run this example, ensure the `Greeter` server is already running.

use std::net::SocketAddr;

use helloworld::HelloRequest;
use helloworld::greeter_client::GreeterClient;

/// The module containing generated code from the proto file.
pub mod helloworld {
    tonic::include_proto!("helloworld");
}

/// Sends a greeting request to `address` and returns the reply.
///
/// # Errors
/// Returns an error if the server cannot be reached or the request fails.
async fn greet(address: SocketAddr, name: &str) -> anyhow::Result<String> {
    let mut client =
        GreeterClient::connect(format!("http://{address}")).await?;
    let request = tonic::Request::new(HelloRequest {
        name: name.to_owned(),
    });
    let response = client.say_hello(request).await?;
    Ok(response.into_inner().message)
}

/// Sends a request to the gRPC server and prints the reply.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = "[::1]:50051".parse()?;
    let message = greet(address, "Rustacean").await?;
    println!("Greeting from server: {message}");

    Ok(())
}
// ANCHOR_END: example

/// Boilerplate to expose the example logic to the crate's main harness.
#[expect(dead_code, reason = "needs a running `Greeter` server")]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the client helper can call a server on a loopback socket.
    #[tokio::test]
    async fn grpc_client_receives_greeting_and_shuts_down_server()
    -> anyhow::Result<()> {
        use tokio::net::TcpListener;
        use tokio::sync::oneshot;
        use tokio_stream::wrappers::TcpListenerStream;

        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
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

        let call_result = greet(address, "Integration").await;
        let shutdown_result = shutdown_sender.send(());
        let server_result = server.await;

        assert_eq!(call_result?, "Hello Integration!");
        shutdown_result.map_err(|()| {
            anyhow::anyhow!("gRPC server exited before shutdown")
        })?;
        server_result??;
        Ok(())
    }

    /// Runs the example against the separately started demo server.
    #[test]
    #[ignore = "needs the tonic server example listening on [::1]:50051"]
    fn test_main_entry_point() {
        main().expect("the example should succeed when the server is running");
    }
}
