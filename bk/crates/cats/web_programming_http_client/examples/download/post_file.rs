// TODO Reviewed ??-??-2026
#![allow(dead_code)]
// ANCHOR: example
//! This example shows how to POST a file to a server.

use std::path::Path;

use anyhow::Result;
use tokio::fs;

async fn post_file(path: impl AsRef<Path>, endpoint: &str) -> Result<String> {
    let contents = fs::read_to_string(path).await?;
    let client = reqwest::Client::new();
    let response = client.post(endpoint).body(contents).send().await?;
    Ok(response.text().await?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let response_text =
        post_file("temp/message", "https://httpbin.org/post").await?;
    println!("Response from httpbin: {response_text}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::io::Write;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn test_main() -> anyhow::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let address = listener.local_addr()?;
        let server = std::thread::spawn(move || -> std::io::Result<Vec<u8>> {
            let (mut stream, _) = listener.accept()?;
            stream.set_read_timeout(Some(Duration::from_secs(5)))?;
            let request = read_request(&mut stream)?;
            stream.write_all(
                b"HTTP/1.1 200 OK\r\nContent-Length: 8\r\nConnection: close\r\n\r\naccepted",
            )?;
            Ok(request)
        });

        let directory = tempfile::tempdir()?;
        let message_path = directory.path().join("message");
        tokio::fs::write(&message_path, "Hello").await?;
        let response =
            post_file(&message_path, &format!("http://{address}/post")).await?;
        assert_eq!(response, "accepted");

        let request = String::from_utf8(
            server.join().expect("HTTP test server panicked")?,
        )?;
        assert!(request.starts_with("POST /post HTTP/1.1\r\n"));
        assert!(request.ends_with("Hello"));
        Ok(())
    }

    fn read_request(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
        let mut request = Vec::new();
        let mut buffer = [0; 1024];

        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..bytes_read]);

            let Some(header_end) =
                request.windows(4).position(|window| window == b"\r\n\r\n")
            else {
                continue;
            };
            let headers = String::from_utf8_lossy(&request[..header_end]);
            let content_length = headers.lines().find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().ok())
                    .flatten()
            });
            if let Some(length) = content_length {
                if request.len() >= header_end + 4 + length {
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
                    && request[header_end + 4..].ends_with(b"0\r\n\r\n")
                {
                    break;
                }
                if !is_chunked {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "HTTP request omitted a supported body length",
                    ));
                }
            }
        }

        Ok(request)
    }
}
