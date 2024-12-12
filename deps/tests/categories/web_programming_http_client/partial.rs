// ANCHOR: example
use std::fs::File;
use std::str::FromStr;

use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use reqwest::StatusCode;
use reqwest::header::CONTENT_LENGTH;
use reqwest::header::HeaderValue;
use reqwest::header::RANGE;

struct PartialRangeIter {
    start: u64,
    end: u64,
    buffer_size: u32,
}

impl PartialRangeIter {
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
        })
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(
                self.buffer_size as u64,
                self.end - self.start + 1,
            );
            Some(
                HeaderValue::from_str(&format!(
                    "bytes={}-{}",
                    prev_start,
                    self.start - 1
                ))
                .expect("string provided by format!"),
            )
        }
    }
}

fn main() -> Result<()> {
    let url = "https://httpbin.org/range/102400?duration=2";
    const CHUNK_SIZE: u32 = 10240;

    let client = reqwest::blocking::Client::new();
    let response = client.head(url).send()?;
    let length = response
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or(anyhow!("response doesn't include the content length"))?;
    let length = u64::from_str(length.to_str()?)
        .map_err(|_| anyhow!("invalid Content-Length header"))?;

    if !std::fs::exists("temp")? {
        std::fs::create_dir("temp")?;
    }
    let mut output_file = File::create("temp/download.bin")?;

    println!("starting download...");
    for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
        println!("range {:?}", range);
        let mut response = client.get(url).header(RANGE, range).send()?;

        let status = response.status();
        if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT)
        {
            bail!("Unexpected server response: {}", status)
        }
        std::io::copy(&mut response, &mut output_file)?;
    }

    let content = response.text()?;
    std::io::copy(&mut content.as_bytes(), &mut output_file)?;

    println!("Finished with success!");
    Ok(())
}
// ANCHOR_END: example

// Requires network access
// TODO P1 flaky test
#[ignore]
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
