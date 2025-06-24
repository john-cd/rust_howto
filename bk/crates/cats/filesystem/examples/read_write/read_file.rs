#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates writing to and reading from a file.

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::Write;

fn main() -> Result<(), Error> {
    let path = "temp/lines.txt";

    // Create a file if it does not exist, and will truncate it if it does.
    let mut output = File::create(path)?;
    // Write to the file.
    write!(&mut output, "Rust\n💖\nFun")?;

    // Open the file for reading.
    let input = File::open(path)?;
    // Adds buffering.
    let buffered = BufReader::new(input);

    // Iterate over the lines of this reader.
    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    use std::fs;
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    main()?;
    Ok(())
}
