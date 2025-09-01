#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates writing to and reading from a file.

use std::fs::File;
use std::fs::OpenOptions;
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

    // Open the file for reading:
    let input = File::open(path)?;
    // Adds buffering:
    let buffered = BufReader::new(input);

    // Iterate over the lines of this reader:
    // The `?` operator propagates the error if one occurs.
    for line in buffered.lines() {
        println!("{}", line?);
    }

    // `read_to_string` returns a `Result<String, Error>`.
    let contents = std::fs::read_to_string(path)?;

    // If successful, print the contents.
    println!("File contents:\n{}", contents);

    // Open the file in append mode.
    let mut file = OpenOptions::new()
        .append(true) // Set the append mode to true.
        .create(true) // Create the file if it doesn't exist.
        .open("output.txt")?;

    // `writeln!` is a macro that writes a formatted string followed by a
    // newline.
    writeln!(file, "This is a new line appended to the file.")?;

    println!("Successfully appended to the file.");

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
