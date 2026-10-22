#![allow(dead_code)]
// ANCHOR: example
//! Example of using the `reqwest` crate to create and delete a GitHub Gist.
//!
//! TODO Reviewed ??-??-2026

use std::collections::HashMap;
use std::env;

use anyhow::Context as _;
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;

/// The authenticated GitHub API endpoint for creating a gist.
const GISTS_ENDPOINT: &str = "https://api.github.com/gists";

/// Represents the JSON body used to create a gist.
#[derive(Serialize, Debug)]
struct Post<'a> {
    /// The description of the gist.
    description: &'a str,
    /// Whether the gist is public.
    public: bool,
    /// The files of the gist, by name.
    files: HashMap<&'a str, Content<'a>>,
}

/// Represents the content of a file within a gist.
#[derive(Serialize, Debug)]
struct Content<'a> {
    /// The text of the file.
    content: &'a str,
}

/// Represents the identifying fields returned for a created gist.
#[derive(Deserialize, Debug)]
struct Gist {
    /// The identifier of the gist.
    id: String,
    /// The URL of the gist.
    html_url: String,
}

/// Creates and then deletes a public gist using the supplied client and token.
async fn create_and_delete_gist(
    client: &Client,
    endpoint: &str,
    token: &str,
    gist_body: &Post<'_>,
) -> Result<()> {
    let gist: Gist = client
        .post(endpoint)
        .bearer_auth(token)
        .json(gist_body)
        .send()
        .await
        .context("sending the gist creation request")?
        .error_for_status()
        .context("creating the gist")?
        .json()
        .await
        .context("reading the created gist response")?;

    println!("Created gist at {}", gist.html_url);

    let delete_url = format!("{endpoint}/{}", gist.id);
    let response = client
        .delete(&delete_url)
        .bearer_auth(token)
        .send()
        .await
        .context("sending the gist deletion request")?;
    let status = response.status();
    response.error_for_status().context("deleting the gist")?;

    println!("Gist {} deleted with status {status}", gist.id);
    Ok(())
}

/// Runs the example with the GitHub token configured in the environment.
#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("GH_TOKEN").context(
        "GH_TOKEN is required; configure a fine-grained token with Gists user permission (write)",
    )?;
    let gist_body = Post {
        description: "A short-lived gist created by the Rust How-To example",
        public: true,
        files: HashMap::from([(
            "main.rs",
            Content {
                content: "fn main() { println!(\"hello world!\"); }\n",
            },
        )]),
    };

    create_and_delete_gist(&Client::new(), GISTS_ENDPOINT, &token, &gist_body)
        .await
}

// ANCHOR_END: example

/// Runs the example from the APIs command-line dispatcher.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::io::Read as _;
    use std::io::Write as _;
    use std::io::{self};
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::thread::JoinHandle;
    use std::thread::{self};
    use std::time::Duration;

    use anyhow::Result;
    use anyhow::anyhow;
    use reqwest::Client;

    use super::Content;
    use super::Post;
    use super::create_and_delete_gist;

    /// Holds one request captured by the local HTTP server.
    struct RecordedRequest {
        request_line: String,
        headers: String,
        body: String,
    }

    /// Starts a local HTTP server that responds to the supplied requests in
    /// order.
    fn start_server(
        responses: Vec<(u16, &'static str)>,
    ) -> Result<(String, JoinHandle<Result<Vec<RecordedRequest>>>)> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let address = listener.local_addr()?;
        let server = thread::spawn(move || {
            let mut requests = Vec::with_capacity(responses.len());
            for (status, body) in responses {
                let (mut stream, _) = listener.accept()?;
                stream.set_read_timeout(Some(Duration::from_secs(5)))?;
                let request = read_request(&mut stream)?;
                let reason = match status {
                    201 => "Created",
                    204 => "No Content",
                    403 => "Forbidden",
                    500 => "Internal Server Error",
                    _ => "OK",
                };
                write!(
                    stream,
                    "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )?;
                stream.write_all(body.as_bytes())?;
                requests.push(request);
            }
            Ok(requests)
        });

        Ok((format!("http://{address}/gists"), server))
    }

    /// Reads one HTTP/1.1 request and its declared body from a TCP stream.
    fn read_request(stream: &mut TcpStream) -> io::Result<RecordedRequest> {
        let mut bytes = Vec::new();
        let mut buffer = [0_u8; 1024];
        let header_end = loop {
            let read = stream.read(&mut buffer)?;
            if read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "connection closed before request headers were complete",
                ));
            }
            let chunk = buffer.get(..read).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "read length exceeds request buffer",
                )
            })?;
            bytes.extend_from_slice(chunk);
            if let Some(end) = find_header_end(&bytes) {
                break end;
            }
        };

        let header_bytes = bytes.get(..header_end).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "header delimiter is outside the request",
            )
        })?;
        let headers = std::str::from_utf8(header_bytes)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?
            .to_owned();
        let content_length = headers
            .lines()
            .skip(1)
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then_some(value.trim())
            })
            .map(|value| {
                value.parse::<usize>().map_err(|error| {
                    io::Error::new(io::ErrorKind::InvalidData, error)
                })
            })
            .transpose()?
            .unwrap_or(0);
        let body_start = header_end + 4;
        let body_end = body_start + content_length;
        while bytes.len() < body_end {
            let read = stream.read(&mut buffer)?;
            if read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "connection closed before request body was complete",
                ));
            }
            let chunk = buffer.get(..read).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "read length exceeds request buffer",
                )
            })?;
            bytes.extend_from_slice(chunk);
        }

        let request_line = headers.lines().next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "missing request line")
        })?;
        let body_bytes = bytes.get(body_start..body_end).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "declared body is outside the request",
            )
        })?;
        let body = std::str::from_utf8(body_bytes).map_err(|error| {
            io::Error::new(io::ErrorKind::InvalidData, error)
        })?;

        Ok(RecordedRequest {
            request_line: request_line.to_owned(),
            headers: headers.clone(),
            body: body.to_owned(),
        })
    }

    /// Finds the position of the CRLF delimiter that ends HTTP headers.
    fn find_header_end(bytes: &[u8]) -> Option<usize> {
        bytes.windows(4).position(|window| window == b"\r\n\r\n")
    }

    /// Builds the deterministic gist payload used by the local server tests.
    fn test_gist_body() -> Post<'static> {
        Post {
            description: "test gist",
            public: true,
            files: std::collections::HashMap::from([(
                "main.rs",
                Content {
                    content: "fn main() {}",
                },
            )]),
        }
    }

    /// Joins the local server and returns the requests it captured.
    fn finish_server(
        server: JoinHandle<Result<Vec<RecordedRequest>>>,
    ) -> Result<Vec<RecordedRequest>> {
        server
            .join()
            .map_err(|_| anyhow!("local HTTP fixture thread panicked"))?
    }

    /// Verifies token authorization, the gist JSON, and the create/delete
    /// sequence.
    #[tokio::test]
    async fn creates_and_deletes_gist_with_authorization_and_json() -> Result<()>
    {
        let (endpoint, server) = start_server(vec![
            (
                201,
                r#"{"id":"gist-123","html_url":"https://gist.github.com/gist-123"}"#,
            ),
            (204, ""),
        ])?;

        create_and_delete_gist(
            &Client::new(),
            &endpoint,
            "test-token",
            &test_gist_body(),
        )
        .await?;

        let requests = finish_server(server)?;
        assert_eq!(requests.len(), 2);
        let post_request = requests
            .first()
            .ok_or_else(|| anyhow!("the create request was not captured"))?;
        let delete_request = requests
            .get(1)
            .ok_or_else(|| anyhow!("the delete request was not captured"))?;
        assert!(
            post_request
                .request_line
                .starts_with("POST /gists HTTP/1.1")
        );
        assert!(post_request.headers.lines().any(|header| {
            header.eq_ignore_ascii_case("authorization: bearer test-token")
        }));
        let payload: serde_json::Value =
            serde_json::from_str(&post_request.body)?;
        assert_eq!(
            payload
                .get("description")
                .and_then(serde_json::Value::as_str),
            Some("test gist")
        );
        assert_eq!(
            payload.get("public").and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert_eq!(
            payload
                .get("files")
                .and_then(|files| files.get("main.rs"))
                .and_then(|file| file.get("content"))
                .and_then(serde_json::Value::as_str),
            Some("fn main() {}")
        );
        assert!(
            delete_request
                .request_line
                .starts_with("DELETE /gists/gist-123 HTTP/1.1")
        );
        assert!(delete_request.headers.lines().any(|header| {
            header.eq_ignore_ascii_case("authorization: bearer test-token")
        }));

        Ok(())
    }

    /// Verifies that a failed create response is returned and no delete is
    /// attempted.
    #[tokio::test]
    async fn returns_create_status_failure_without_deleting() -> Result<()> {
        let (endpoint, server) = start_server(vec![(500, "{}")])?;

        let error = create_and_delete_gist(
            &Client::new(),
            &endpoint,
            "test-token",
            &test_gist_body(),
        )
        .await
        .expect_err("server error must fail gist creation");
        assert!(format!("{error:#}").contains("500"));
        assert_eq!(finish_server(server)?.len(), 1);

        Ok(())
    }

    /// Verifies that a failed delete response is returned after the delete
    /// request is sent.
    #[tokio::test]
    async fn returns_delete_status_failure() -> Result<()> {
        let (endpoint, server) = start_server(vec![
            (
                201,
                r#"{"id":"gist-123","html_url":"https://gist.github.com/gist-123"}"#,
            ),
            (403, "{}"),
        ])?;

        let error = create_and_delete_gist(
            &Client::new(),
            &endpoint,
            "test-token",
            &test_gist_body(),
        )
        .await
        .expect_err("forbidden response must fail gist deletion");
        assert!(format!("{error:#}").contains("403"));
        let requests = finish_server(server)?;
        assert_eq!(requests.len(), 2);
        assert!(
            requests
                .get(1)
                .ok_or_else(|| anyhow!("the delete request was not captured"))?
                .request_line
                .starts_with("DELETE /gists/gist-123 HTTP/1.1")
        );

        Ok(())
    }
}
// [review; rewrite so that a username and password are not required?](https://github.com/john-cd/rust_howto/issues/178)
