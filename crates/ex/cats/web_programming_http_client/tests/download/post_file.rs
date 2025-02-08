// ANCHOR: example
use std::fs::File;
use std::io::Read;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut file = File::open("temp/message")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let paste_api = "https://paste.rs";
    let client = reqwest::Client::new();
    let res = client.post(paste_api).body(contents).send().await?;
    let response_text = res.text().await?;
    println!("Your paste is located at: {}", response_text);
    Ok(())
}
// ANCHOR_END: example

use std::io::Write;

#[test]
fn require_network() -> anyhow::Result<()> {
    if !std::fs::exists("temp")? {
        std::fs::create_dir("temp")?;
    }
    let mut f = File::create("temp/message")?;
    f.write_all(b"Hello")?;
    main()?;
    Ok(())
}
