// TODO Reviewed ??-??-2026
#![allow(dead_code)]
// ANCHOR: example
//! This example shows how to POST a file to a server.

use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use tokio::fs;

/// Reads a file and POSTs its bytes to an endpoint.
///
/// # Errors
///
/// Returns an error if the file cannot be read, the request fails, the server
/// returns an unsuccessful status, or the response body cannot be read.
async fn post_file(path: impl AsRef<Path>, endpoint: &str) -> Result<String> {
    let path = path.as_ref();
    let contents = fs::read(path).await.with_context(|| {
        format!("failed to read upload file {}", path.display())
    })?;
    let response = reqwest::Client::new()
        .post(endpoint)
        .body(contents)
        .send()
        .await?
        .error_for_status()?;
    Ok(response.text().await?)
}

/// Uploads the sample file to httpbin and prints its response.
///
/// # Errors
///
/// Returns any file, request, HTTP status, or response body error from
/// `post_file`.
#[tokio::main]
async fn main() -> Result<()> {
    let response_text =
        post_file("temp/message", "https://httpbin.org/post").await?;
    println!("Response from httpbin: {response_text}");
    Ok(())
}

// ANCHOR_END: example

/// Runs the file upload example.
///
/// # Errors
///
/// Returns any error from the example's asynchronous entry point.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Read;
    use std::io::Write;
    use std::net::SocketAddr;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::thread::JoinHandle;
    use std::time::Duration;

    use anyhow::Context;
    use anyhow::Result;
    use reqwest::StatusCode;

    use super::post_file;

    /// Starts one local HTTP server response and captures the request bytes.
    fn start_server(
        response: &'static [u8],
    ) -> Result<(SocketAddr, JoinHandle<io::Result<Vec<u8>>>)> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let address = listener.local_addr()?;
        let server = std::thread::spawn(move || -> io::Result<Vec<u8>> {
            let (mut stream, _) = listener.accept()?;
            stream.set_read_timeout(Some(Duration::from_secs(5)))?;
            let request = read_request(&mut stream)?;
            stream.write_all(response)?;
            Ok(request)
        });

        Ok((address, server))
    }

    /// Joins the local server and reports its I/O error or panic.
    fn join_server(server: JoinHandle<io::Result<Vec<u8>>>) -> Result<Vec<u8>> {
        server
            .join()
            .map_err(|_| anyhow::anyhow!("HTTP test server panicked"))?
            .context("HTTP test server failed")
    }

    /// Reads one complete HTTP request, including a length-delimited or chunked
    /// body.
    fn read_request(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
        let mut request = Vec::new();
        let mut buffer = [0; 1024];

        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            let chunk = buffer.get(..bytes_read).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "HTTP read exceeded the request buffer",
                )
            })?;
            request.extend_from_slice(chunk);

            let Some(header_end) =
                request.windows(4).position(|window| window == b"\r\n\r\n")
            else {
                continue;
            };
            let header_bytes = request.get(..header_end).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "HTTP header boundary was outside the request",
                )
            })?;
            let headers = String::from_utf8_lossy(header_bytes);
            let body_start = header_end.checked_add(4).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "HTTP header length overflowed",
                )
            })?;
            let content_length = headers.lines().find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().ok())
                    .flatten()
            });
            if let Some(length) = content_length {
                let body_end =
                    body_start.checked_add(length).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "HTTP request body length overflowed",
                        )
                    })?;
                if request.len() >= body_end {
                    break;
                }
            } else {
                let is_chunked = headers.lines().any(|line| {
                    let Some((name, value)) = line.split_once(':') else {
                        return false;
                    };
                    name.eq_ignore_ascii_case("transfer-encoding")
                        && value.trim().eq_ignore_ascii_case("chunked")
                });
                if is_chunked
                    && request
                        .get(body_start..)
                        .is_some_and(|body| body.ends_with(b"0\r\n\r\n"))
                {
                    break;
                }
                if !is_chunked {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "HTTP request omitted a supported body length",
                    ));
                }
            }
        }

        Ok(request)
    }

    /// Verifies that file bytes are posted and a successful response is
    /// returned.
    #[tokio::test]
    async fn posts_file_bytes_to_local_server() -> Result<()> {
        let (address, server) = start_server(
            b"HTTP/1.1 200 OK\r\nContent-Length: 8\r\nConnection: close\r\n\r\naccepted",
        )?;
        let directory = tempfile::tempdir()?;
        let message_path = directory.path().join("message");
        tokio::fs::write(&message_path, b"Hello\0").await?;

        let response =
            post_file(&message_path, &format!("http://{address}/post")).await;
        let request = join_server(server)?;

        assert_eq!(response?, "accepted");
        assert!(request.starts_with(b"POST /post HTTP/1.1\r\n"));
        assert!(request.ends_with(b"Hello\0"));
        Ok(())
    }

    /// Verifies that a missing upload file returns a contextual read error.
    #[tokio::test]
    async fn missing_file_returns_read_error() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let missing_path = directory.path().join("missing-message");

        let error =
            match post_file(&missing_path, "http://127.0.0.1:1/post").await {
                Ok(_) => {
                    anyhow::bail!("missing upload file unexpectedly succeeded")
                }
                Err(error) => error,
            };

        assert!(error.to_string().contains("missing-message"));
        Ok(())
    }

    /// Verifies that reqwest request construction errors propagate to the
    /// caller.
    #[tokio::test]
    async fn invalid_endpoint_returns_request_error() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let message_path = directory.path().join("message");
        tokio::fs::write(&message_path, b"Hello").await?;

        let error = match post_file(&message_path, "not a URL").await {
            Ok(_) => anyhow::bail!("invalid endpoint unexpectedly succeeded"),
            Err(error) => error,
        };

        assert!(error.downcast_ref::<reqwest::Error>().is_some());
        Ok(())
    }

    /// Verifies that an unsuccessful HTTP response becomes an error.
    #[tokio::test]
    async fn unsuccessful_status_returns_request_error() -> Result<()> {
        let (address, server) = start_server(
            b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 8\r\nConnection: close\r\n\r\nrejected",
        )?;
        let directory = tempfile::tempdir()?;
        let message_path = directory.path().join("message");
        tokio::fs::write(&message_path, b"Hello").await?;

        let response =
            post_file(&message_path, &format!("http://{address}/post")).await;
        let _request = join_server(server)?;
        let error = match response {
            Ok(_) => anyhow::bail!("HTTP 500 response unexpectedly succeeded"),
            Err(error) => error,
        };

        let status = error
            .downcast_ref::<reqwest::Error>()
            .and_then(|error| error.status());
        assert_eq!(status, Some(StatusCode::INTERNAL_SERVER_ERROR));
        Ok(())
    }
}
