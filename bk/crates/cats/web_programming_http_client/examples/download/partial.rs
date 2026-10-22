// TODO Reviewed ??-??-2026
#![allow(dead_code)]
// ANCHOR: example
//! Partial download example.

use std::fs::File;
use std::str::FromStr;

use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use reqwest::StatusCode;
use reqwest::header::CONTENT_LENGTH;
use reqwest::header::HeaderValue;
use reqwest::header::RANGE;

/// An iterator that yields `Range` headers.
struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
    finished: bool,
}

impl PartialRangeIter {
    /// Creates a new `PartialRangeIter`.
    ///
    /// # Arguments
    ///
    /// * `start` - The start of the range.
    /// * `end` - The end of the range.
    /// * `buffer_size` - The size of each chunk.
    pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
        if buffer_size == 0 {
            Err(anyhow!(
                "invalid buffer_size, give a value greater than zero."
            ))?;
        }
        Ok(PartialRangeIter {
            start,
            end,
            buffer_size,
            finished: start > end,
        })
    }
}

/// Implements the `Iterator` trait for `PartialRangeIter`.
impl Iterator for PartialRangeIter {
    type Item = HeaderValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let chunk_end = self
                .start
                .saturating_add(u64::from(self.buffer_size) - 1)
                .min(self.end);
            let header = HeaderValue::from_str(&format!(
                "bytes={}-{}",
                self.start, chunk_end
            ))
            .expect("string provided by format!");
            if chunk_end == self.end {
                self.finished = true;
            } else {
                self.start = chunk_end + 1;
            }
            Some(header)
        }
    }
}

/// This function downloads a file from a given URL in chunks,
/// using the `PartialRangeIter` to generate the `Range` headers.
fn main() -> Result<()> {
    let url = "https://httpbin.org/range/102400?duration=2";
    const CHUNK_SIZE: u32 = 10240;

    let client = reqwest::blocking::Client::new();
    let response = client.head(url).send()?;
    if !response.status().is_success() {
        bail!("HEAD request failed: {}", response.status());
    }
    let length = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or(anyhow!("response doesn't include the content length"))?;
    let length = u64::from_str(length.to_str()?)
        .map_err(|_| anyhow!("invalid Content-Length header"))?;

    std::fs::create_dir_all("temp")?;
    let mut output_file = File::create("temp/download.bin")?;

    println!("starting download...");
    if length > 0 {
        for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
            println!("range {range:?}");
            let mut response = client.get(url).header(RANGE, range).send()?;

            let status = response.status();
            if status != StatusCode::PARTIAL_CONTENT {
                bail!("Unexpected server response: {status}")
            }
            std::io::copy(&mut response, &mut output_file)?;
        }
    }

    println!("Finished with success!");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterates_inclusive_ranges_without_gaps_or_overflow() -> anyhow::Result<()>
    {
        let ranges = PartialRangeIter::new(0, 24, 10)?
            .map(|header| header.to_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(ranges, ["bytes=0-9", "bytes=10-19", "bytes=20-24"]);

        let final_range = PartialRangeIter::new(u64::MAX - 1, u64::MAX, 10)?
            .next()
            .expect("non-empty range")
            .to_str()?
            .to_owned();
        assert_eq!(final_range, format!("bytes={}-{}", u64::MAX - 1, u64::MAX));
        assert!(PartialRangeIter::new(1, 0, 10)?.next().is_none());
        Ok(())
    }

    #[ignore = "Needs review"]
    #[test]
    fn require_network() -> anyhow::Result<()> {
        use std::fs;
        if !fs::exists("temp")? {
            fs::create_dir("temp")?;
        }
        main()?;
        Ok(())
    }
}
// [review; flaky test](https://github.com/john-cd/rust_howto/issues/176)
