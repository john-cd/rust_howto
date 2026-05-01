#![allow(dead_code)]
// ANCHOR: example
//! Download a file from a URL and save it to a temporary directory.

use std::fs::File;
use std::io::copy;

use anyhow::Result;
use tempfile::Builder;

/// Downloads a file from a URL and saves it to a temporary directory.
///
/// This function demonstrates how to use the `reqwest` crate to download a file
/// and the `tempfile` crate to create a temporary directory.
#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|mut segments| segments.next_back())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{fname}'");
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{fname:?}'");
        File::create(fname)?
    };
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn require_network() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review; noplayground because of network use. rewrite?](https://github.com/john-cd/rust_howto/issues/225)
