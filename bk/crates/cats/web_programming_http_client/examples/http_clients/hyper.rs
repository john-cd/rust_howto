#![allow(dead_code)]
// ANCHOR: example
// // COMING SOON
// ANCHOR_END: example
//! Example of using the `hyper` low-level HTTP client to fetch a URL.
//!
//! This example demonstrates how to:
//! - Connect to a remote server using TCP.
//! - Perform an HTTP GET request.
//! - Stream the response body to stdout.

use anyhow::Context;
use http_body_util::BodyExt;
use http_body_util::Empty;
use hyper::Request;
// use hyper::body::Body;
use hyper::body::Bytes;
// use hyper::http::response::Response;
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse our URL...
    let url = "http://httpbin.org/ip".parse::<hyper::Uri>()?;

    fetch_url(url).await?;

    println!("\n\nDone!");
    Ok(())
}

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review NOW](https://github.com/john-cd/rust_howto/issues/859)
// review <https://hyper.rs/>
// reference <https://github.com/hyperium/hyper/blob/master/examples/client.rs>
