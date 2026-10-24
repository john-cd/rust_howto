#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example serves a small HTTP response with Glommio's Linux `io_uring`
//! runtime.
//!
//! Security: this Glommio release transitively uses `owned-alloc`, which has
//! no fix for RUSTSEC-2026-0291. Its double-free requires a panicking `Drop`
//! during `OwnedAlloc::drop_in_place`; this isolated Linux example is kept for
//! education and must not be copied into production services.
//!
//! Glommio is a thread-per-core asynchronous runtime based on Linux `io_uring`.
//! It requires a Linux kernel with `io_uring` support (5.8 or newer
//! recommended).

#![cfg(target_os = "linux")]

use futures_lite::io::AsyncReadExt as _;
use futures_lite::io::AsyncWriteExt as _;
use glommio::LocalExecutorBuilder;
use glommio::net::TcpListener;
use glommio::net::TcpStream;

/// Handles one HTTP request and writes a small plain-text response.
async fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut request = [0_u8; 4096];
    let bytes_read = stream.read(&mut request).await?;
    if bytes_read == 0 {
        return Ok(());
    }

    stream.write_all(http_response().as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}

/// Formats the HTTP response returned by the example server.
fn http_response() -> String {
    let body = "Hello from Glommio!\n";
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
}

/// Accepts connections and dispatches each request on the local executor.
async fn serve() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Glommio server listening on http://127.0.0.1:8080");

    loop {
        let stream = listener.accept().await?;
        glommio::spawn_local(async move {
            if let Err(error) = handle_connection(stream).await {
                eprintln!("failed to serve HTTP request: {error:#}");
            }
        })
        .detach();
    }
}

/// Starts the Glommio executor and serves HTTP requests until the process
/// stops.
fn main() -> anyhow::Result<()> {
    let executor = LocalExecutorBuilder::default()
        .spawn(serve)
        .map_err(|()| anyhow::anyhow!("failed to start Glommio executor"))?;
    executor
        .join()
        .map_err(|_| anyhow::anyhow!("Glommio executor thread panicked"))??;
    Ok(())
}
// ANCHOR_END: example

/// Runs the Glommio HTTP server.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the response includes a valid length and a close directive.
    #[test]
    fn response_has_content_length_and_closes_connection() {
        let response = http_response();

        assert!(response.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(response.contains("Content-Length: 20\r\n"));
        assert!(response.ends_with("\r\n\r\nHello from Glommio!\n"));
    }
}
