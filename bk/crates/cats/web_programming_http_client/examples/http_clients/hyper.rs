// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of using the `hyper` low-level HTTP client to fetch a URL.
//!
//! This example demonstrates how to:
//! - Connect to a remote server using TCP.
//! - Perform an HTTP GET request.
//! - Stream the response body to stdout.

use anyhow::Context as _;
use http_body_util::BodyExt as _;
use http_body_util::Empty;
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::io;
use tokio::io::AsyncWriteExt as _;
use tokio::net::TcpStream;

/// Fetches the content of a URL using the `hyper` HTTP client.
///
/// This function establishes a TCP connection to the specified host,
/// sends an HTTP GET request to the given URL, and streams the response
/// body to standard output.
async fn fetch_url(url: hyper::Uri) -> anyhow::Result<()> {
    let host = url.host().context("uri has no host")?;
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{host}:{port}");

    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {err:?}");
        }
    });

    let authority = url
        .authority()
        .ok_or(anyhow::anyhow!("`authority` failed."))?
        .clone();

    // Build the request:
    let path = url.path();
    let req = Request::builder()
        .method("GET")
        .uri(path)
        .header(hyper::header::HOST, authority.as_str())
        //.header("User-Agent", "hyper")
        .body(Empty::<Bytes>::new())?;

    // Send the request and wait for the response:
    let mut res = sender.send_request(req).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            io::stdout().write_all(chunk).await?;
        }
    }
    // Or asynchronously aggregate the chunks of the body:
    // let body = res.collect().await?.aggregate();

    Ok(())
}

/// Fetches a URL with `hyper` and prints the response.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse our URL...
    let url = "http://httpbin.org/ip".parse::<hyper::Uri>()?;

    fetch_url(url).await?;

    println!("\n\nDone!");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "needs network"]
    fn require_network() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// The response of a local server is fetched.
    #[tokio::test]
    async fn fetches_from_a_local_server() -> anyhow::Result<()> {
        use tokio::io::AsyncReadExt as _;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await?;
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await?;
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
                .await?;
            anyhow::Ok(String::from_utf8_lossy(&buffer[..n]).into_owned())
        });
        fetch_url(format!("http://127.0.0.1:{port}/ip").parse()?).await?;
        let request = server.await??;
        assert!(request.starts_with("GET /ip HTTP/1.1"), "{request}");
        assert!(request.to_lowercase().contains("host: 127.0.0.1"));
        Ok(())
    }

    /// A URI without a host is rejected.
    #[tokio::test]
    async fn uri_without_host_is_rejected()
    -> Result<(), Box<dyn std::error::Error>> {
        assert!(fetch_url("/just/a/path".parse()?).await.is_err());
        Ok(())
    }
}
