// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Downloads a file in verified byte ranges.
//!
//! HTTP range requests (the `Range: bytes=0-99` header) ask the server for part
//! of a file. They allow resuming interrupted downloads and fetching in
//! parallel. Servers that support them answer with status 206 (Partial
//! Content).

use std::io;
use std::io::Write;
use std::ops::RangeInclusive;
use std::path::Path;
use std::str::FromStr as _;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use reqwest::StatusCode;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use reqwest::header::CONTENT_RANGE;
use reqwest::header::ETAG;
use reqwest::header::HeaderValue;
use reqwest::header::IF_RANGE;
use reqwest::header::RANGE;

/// An iterator that yields adjacent, inclusive byte ranges.
struct PartialRangeIter {
    /// The first byte of the next range.
    start: u64,

    /// The last byte of the interval.
    end: u64,

    /// The maximum size of a range.
    buffer_size: u32,

    /// Whether all the ranges were yielded.
    finished: bool,
}

impl PartialRangeIter {
    /// Creates a range iterator for the inclusive interval from `start` to
    ///
    /// # Arguments
    ///
    /// `end`.
    ///
    /// An interval with `start > end` is empty.
    ///
    /// # Errors
    ///
    /// Returns an error if `buffer_size` is zero.
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
        if buffer_size == 0 {
            bail!("buffer_size must be greater than zero");
        }
        Ok(Self {
            start,
            end,
            buffer_size,
            finished: start > end,
        })
    }
}

/// Iterates over consecutive byte ranges without gaps or overlaps.
impl Iterator for PartialRangeIter {
    type Item = RangeInclusive<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let chunk_end = self
            .start
            .saturating_add(u64::from(self.buffer_size) - 1)
            .min(self.end);
        let range = self.start..=chunk_end;
        if chunk_end == self.end {
            self.finished = true;
        } else {
            self.start = chunk_end + 1;
        }
        Some(range)
    }
}

/// Downloads a URL to a writer using verified HTTP byte ranges.
///
/// # Errors
///
/// Returns an error if the server omits or provides invalid length metadata,
/// does not return `206 Partial Content` for a range, returns a mismatched or
/// incomplete range, or if a request or write fails.
fn download_to<W: Write>(url: &str, chunk_size: u32, output: &mut W) -> Result<()> {
    if chunk_size == 0 {
        bail!("chunk_size must be greater than zero");
    }

    let client = Client::new();
    let head_response = client.head(url).send()?.error_for_status()?;
    let content_length = head_response
        .headers()
        .get(CONTENT_LENGTH)
        .context("HEAD response omitted Content-Length")?
        .to_str()
        .context("HEAD response Content-Length was not valid text")?;
    let content_length = u64::from_str(content_length)
        .context("HEAD response Content-Length was not a valid unsigned integer")?;
    // A weak validator cannot establish byte-for-byte identity across requests.
    let etag = head_response
        .headers()
        .get(ETAG)
        .context("HEAD response omitted a strong ETag")?
        .clone();
    let tag = etag.to_str().context("ETag was not valid text")?;
    if !tag.starts_with('"') || !tag.ends_with('"') || tag.len() < 2 {
        bail!("HEAD response did not provide a strong ETag");
    }

    if content_length == 0 {
        return Ok(());
    }

    for range in PartialRangeIter::new(0, content_length - 1, chunk_size)? {
        let start = *range.start();
        let end = *range.end();
        let range_header = HeaderValue::from_str(&format!("bytes={start}-{end}"))?;
        let mut response = client
            .get(url)
            .header(RANGE, range_header)
            .header(IF_RANGE, etag.clone())
            .send()?;
        let status = response.status();
        if status != StatusCode::PARTIAL_CONTENT {
            bail!("Expected 206 Partial Content for bytes {start}-{end}, got {status}");
        }
        if response.headers().get(ETAG) != Some(&etag) {
            bail!("ETag changed or was omitted for bytes {start}-{end}");
        }

        let content_range = response
            .headers()
            .get(CONTENT_RANGE)
            .context("206 response omitted Content-Range")?
            .to_str()
            .context("206 response Content-Range was not valid text")?;
        let expected_content_range = format!("bytes {start}-{end}/{content_length}");
        if content_range != expected_content_range {
            bail!("Expected Content-Range {expected_content_range}, got {content_range}");
        }

        let expected_bytes = end
            .checked_sub(start)
            .and_then(|range_length| range_length.checked_add(1))
            .context("generated byte range had an invalid length")?;
        let copied_bytes = io::copy(&mut response, output)?;
        if copied_bytes != expected_bytes {
            bail!(
                "Expected {expected_bytes} bytes for range {start}-{end}, received {copied_bytes}"
            );
        }
    }

    Ok(())
}

/// Publishes a complete download only when every range has been validated.
fn download_file(url: &str, chunk_size: u32, destination: &Path) -> Result<()> {
    let parent = destination.parent().context("destination has no parent")?;
    let mut staged = tempfile::NamedTempFile::new_in(parent)?;
    download_to(url, chunk_size, staged.as_file_mut())?;
    staged.as_file_mut().sync_all()?;
    staged.persist_noclobber(destination)?;
    Ok(())
}

/// Downloads a sample file in chunks into a temporary file.
///
/// # Errors
///
/// Returns any directory, file, network, response, or write error.
fn main() -> Result<()> {
    /// Maximum number of bytes requested in each download chunk.
    const CHUNK_SIZE: u32 = 4;
    let content = b"0123456789";
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let url = format!("http://{}/resource", listener.local_addr()?);
    let server = std::thread::spawn(move || -> io::Result<()> {
        use io::Read as _;
        for _ in 0..4 {
            let (mut stream, _) = listener.accept()?;
            let mut request = [0; 2048];
            let count = stream.read(&mut request)?;
            let request = String::from_utf8_lossy(&request[..count]);
            if request.starts_with("HEAD ") {
                stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\nETag: \"sample-v1\"\r\nConnection: close\r\n\r\n")?;
            } else {
                let range = request
                    .lines()
                    .find_map(|line| line.strip_prefix("range: bytes="))
                    .ok_or_else(|| io::Error::other("range header missing"))?;
                let (start, end) = range
                    .trim()
                    .split_once('-')
                    .ok_or_else(|| io::Error::other("invalid range"))?;
                let start: usize = start.parse().map_err(io::Error::other)?;
                let end: usize = end.parse().map_err(io::Error::other)?;
                let body = content
                    .get(start..=end)
                    .ok_or_else(|| io::Error::other("invalid bounds"))?;
                write!(
                    stream,
                    "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\nContent-Range: bytes {start}-{end}/10\r\nETag: \"sample-v1\"\r\nConnection: close\r\n\r\n",
                    body.len()
                )?;
                stream.write_all(body)?;
            }
        }
        Ok(())
    });
    let directory = tempfile::tempdir()?;
    let destination = directory.path().join("download.bin");

    println!("starting download...");
    let result = download_file(&url, CHUNK_SIZE, &destination);
    server
        .join()
        .map_err(|_| anyhow::anyhow!("local server panicked"))??;
    result?;
    assert_eq!(std::fs::read(&destination)?, content);
    println!("Finished with success!");
    Ok(())
}
// ANCHOR_END: example

/// Runs the partial-download example.
///
/// # Errors
///
/// Returns any error from the example's file or HTTP operations.
pub fn run() -> Result<()> {
    main()
}

/// Verifies adjacent inclusive ranges, empty intervals, and the integer boundary.
#[cfg(test)]
mod tests {
    use std::io;
    use std::io::Read as _;
    use std::io::Write as _;
    use std::net::TcpListener;
    use std::net::TcpStream;
    use std::thread::JoinHandle;
    use std::time::Duration;
    use std::time::Instant;

    use anyhow::Context as _;
    use anyhow::Result;
    use reqwest::StatusCode;

    use super::PartialRangeIter;
    use super::download_file;
    use super::download_to;

    /// Starts a loopback server that serves the requested content and records requests.
    ///
    /// # Arguments
    ///
    /// * `content` - Bytes returned for the requested ranges.
    /// * `range_status` - Status code sent for range requests.
    /// * `request_count` - Number of client requests the server accepts.
    ///
    /// # Errors
    ///
    /// Returns an error if the loopback listener cannot bind or its address
    /// cannot be read. Errors inside the server thread are returned by its
    /// `JoinHandle`.
    fn start_range_server(
        content: Vec<u8>,
        range_status: StatusCode,
        request_count: usize,
        changed_after_first_range: bool,
        truncate_last_range: bool,
    ) -> Result<(String, JoinHandle<io::Result<Vec<String>>>)> {
        let listener = TcpListener::bind("127.0.0.1:0")?;
        let address = listener.local_addr()?;
        listener.set_nonblocking(true)?;

        let server = std::thread::spawn(move || -> io::Result<Vec<String>> {
            let mut requests = Vec::with_capacity(request_count);
            for request_index in 0..request_count {
                let mut stream = accept_with_timeout(&listener)?;
                stream.set_nonblocking(false)?;
                stream.set_read_timeout(Some(Duration::from_secs(5)))?;
                let request = read_request(&mut stream)?;
                requests.push(request.clone());

                if request.starts_with("HEAD ") {
                    let headers = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nETag: \"sample-v1\"\r\nConnection: close\r\n\r\n",
                        content.len()
                    );
                    stream.write_all(headers.as_bytes())?;
                    continue;
                }

                let (start, end) = parse_range_header(&request)?;
                if range_status == StatusCode::PARTIAL_CONTENT {
                    let start = usize::try_from(start)
                        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
                    let end = usize::try_from(end)
                        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
                    let body = content.get(start..=end).ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "requested test range was outside the content",
                        )
                    })?;
                    let etag = if changed_after_first_range && request_index > 1 {
                        "sample-v2"
                    } else {
                        "sample-v1"
                    };
                    let headers = format!(
                        "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\nContent-Range: bytes {start}-{end}/{}\r\nETag: \"{etag}\"\r\nConnection: close\r\n\r\n",
                        body.len(),
                        content.len()
                    );
                    stream.write_all(headers.as_bytes())?;
                    if truncate_last_range && request_index == request_count - 1 {
                        stream.write_all(&body[..body.len() - 1])?;
                    } else {
                        stream.write_all(body)?;
                    }
                } else {
                    let headers = format!(
                        "HTTP/1.1 {range_status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                        content.len()
                    );
                    stream.write_all(headers.as_bytes())?;
                    stream.write_all(&content)?;
                }
            }
            Ok(requests)
        });

        Ok((format!("http://{address}/resource"), server))
    }

    /// Accepts a local request until the server's bounded wait expires.
    fn accept_with_timeout(listener: &TcpListener) -> io::Result<TcpStream> {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            match listener.accept() {
                Ok((stream, _)) => return Ok(stream),
                Err(error)
                    if error.kind() == io::ErrorKind::WouldBlock && Instant::now() < deadline =>
                {
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(error) => return Err(error),
            }
        }
    }

    /// Reads one HTTP request header block from a local client.
    fn read_request(stream: &mut TcpStream) -> io::Result<String> {
        let mut request = Vec::new();
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "client closed before sending an HTTP header block",
                ));
            }
            let chunk = buffer.get(..bytes_read).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "HTTP read exceeded the request buffer",
                )
            })?;
            request.extend_from_slice(chunk);
            if request.windows(4).any(|window| window == b"\r\n\r\n") {
                return String::from_utf8(request)
                    .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error));
            }
        }
    }

    /// Extracts the inclusive `Range` bounds sent by the client.
    fn parse_range_header(request: &str) -> io::Result<(u64, u64)> {
        let range = request
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("range").then_some(value.trim())
            })
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "GET request omitted its Range header",
                )
            })?;
        let range = range.strip_prefix("bytes=").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Range header did not use byte units",
            )
        })?;
        let (start, end) = range.split_once('-').ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Range header omitted its start or end",
            )
        })?;
        let start = start
            .parse::<u64>()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let end = end
            .parse::<u64>()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        if start > end {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Range header start exceeded its end",
            ));
        }
        Ok((start, end))
    }

    /// Joins the server and returns its captured requests.
    fn join_server(server: JoinHandle<io::Result<Vec<String>>>) -> Result<Vec<String>> {
        server
            .join()
            .map_err(|_| anyhow::anyhow!("local range server panicked"))?
            .context("local range server failed")
    }

    /// Returns the value of a request's Range header, if present.
    fn range_header(request: &str) -> Option<&str> {
        request.lines().find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("range").then_some(value.trim())
        })
    }

    /// Verifies adjacent inclusive ranges, empty intervals, and the integer boundary.
    #[test]
    fn iterator_produces_inclusive_non_overlapping_ranges() -> Result<()> {
        let ranges = PartialRangeIter::new(0, 24, 10)?
            .map(|range| format!("bytes={}-{}", range.start(), range.end()))
            .collect::<Vec<_>>();
        assert_eq!(ranges, ["bytes=0-9", "bytes=10-19", "bytes=20-24"]);

        let final_range = PartialRangeIter::new(u64::MAX - 1, u64::MAX, 10)?
            .next()
            .context("the maximum-boundary interval should not be empty")?;
        assert_eq!(
            format!("bytes={}-{}", final_range.start(), final_range.end()),
            format!("bytes={}-{}", u64::MAX - 1, u64::MAX)
        );
        assert!(PartialRangeIter::new(1, 0, 10)?.next().is_none());
        assert!(PartialRangeIter::new(0, 1, 0).is_err());
        Ok(())
    }

    /// Verifies range headers and exact output bytes against a local server.
    #[test]
    fn downloads_all_ranges_without_gaps_or_overlaps() -> Result<()> {
        let content = b"0123456789".to_vec();
        let (url, server) = start_range_server(
            content.clone(),
            StatusCode::PARTIAL_CONTENT,
            4,
            false,
            false,
        )?;
        let mut output = Vec::new();

        let download_result = download_to(&url, 4, &mut output);
        let requests = join_server(server)?;
        download_result?;

        assert_eq!(output, content);
        assert_eq!(requests.len(), 4);
        assert!(
            requests
                .first()
                .is_some_and(|request| { request.starts_with("HEAD /resource HTTP/1.1\r\n") })
        );
        let ranges = requests
            .iter()
            .skip(1)
            .map(|request| range_header(request).context("GET request omitted its Range header"))
            .collect::<Result<Vec<_>>>()?;
        assert_eq!(ranges, ["bytes=0-3", "bytes=4-7", "bytes=8-9"]);
        Ok(())
    }

    /// Verifies an empty file needs only HEAD and leaves the output empty.
    #[test]
    fn empty_file_does_not_send_range_requests() -> Result<()> {
        let (url, server) =
            start_range_server(Vec::new(), StatusCode::PARTIAL_CONTENT, 1, false, false)?;
        let mut output = Vec::new();

        download_to(&url, 4, &mut output)?;
        let requests = join_server(server)?;

        assert!(output.is_empty());
        assert_eq!(requests.len(), 1);
        assert!(
            requests
                .first()
                .is_some_and(|request| { request.starts_with("HEAD /resource HTTP/1.1\r\n") })
        );
        Ok(())
    }

    /// Verifies that a server which ignores Range and returns 200 is rejected.
    #[test]
    fn rejects_non_partial_range_response() -> Result<()> {
        let content = b"abcd".to_vec();
        let (url, server) = start_range_server(content, StatusCode::OK, 2, false, false)?;
        let mut output = Vec::new();

        let download_result = download_to(&url, 2, &mut output);
        let requests = join_server(server)?;
        let error = download_result
            .err()
            .context("a 200 response to a range request was accepted")?;

        assert_eq!(requests.len(), 2);
        assert!(error.to_string().contains("206 Partial Content"));
        assert!(output.is_empty());
        Ok(())
    }

    /// An equal-length replacement cannot be combined with the first chunk.
    #[test]
    fn changed_etag_preserves_existing_destination() -> Result<()> {
        let (url, server) = start_range_server(
            b"abcdefgh".to_vec(),
            StatusCode::PARTIAL_CONTENT,
            3,
            true,
            false,
        )?;
        let dir = tempfile::tempdir()?;
        let destination = dir.path().join("download.bin");
        std::fs::write(&destination, b"original")?;

        let result = download_file(&url, 4, &destination);
        let requests = join_server(server)?;
        let error = result
            .err()
            .context("changed representation was accepted")?;
        assert!(error.to_string().contains("ETag changed"));
        assert_eq!(std::fs::read(&destination)?, b"original");
        assert_eq!(requests.len(), 3);
        assert!(requests[1].contains("if-range: \"sample-v1\""));
        assert!(requests[2].contains("if-range: \"sample-v1\""));
        Ok(())
    }

    /// A short final response cannot publish a partial file.
    #[test]
    fn truncated_range_does_not_publish() -> Result<()> {
        let (url, server) = start_range_server(
            b"abcdefgh".to_vec(),
            StatusCode::PARTIAL_CONTENT,
            3,
            false,
            true,
        )?;
        let dir = tempfile::tempdir()?;
        let destination = dir.path().join("download.bin");
        let result = download_file(&url, 4, &destination);
        join_server(server)?;
        assert!(result.is_err());
        assert!(!destination.exists());
        Ok(())
    }

    /// Runs the example against its loopback fixture.
    #[test]
    fn main_downloads_loopback() -> Result<()> {
        super::main()
    }
}
