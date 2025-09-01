#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates writing to and reading from a file.

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

fn write_to_file() -> Result<(), io::Error> {
    // The simplest way is to write a slice as the entire contents of a file
    // with `fs::write`. This function will create a file if it does not
    // exist, and will entirely replace its contents if it does.
    let path_str = "temp/lines.txt";
    fs::write(path_str, b"Lorem ipsum")?;

    // Or open a file in write-only mode, then write.
    // `create` creates a file if it does not exist, and truncates it if it
    // does:
    let mut output = File::create(path_str)?;

    // Write an entire buffer to the file:
    output.write_all(b"Writing some bytes.\n")?;
    // Or write a formatted string to the file using e.g. the `write!` macro:
    let text = "Rust 💖 Fun\n";
    write!(&mut output, "Writing: {text}")?;

    // Or open the file in append mode:
    let mut file = OpenOptions::new()
        .append(true) // Set the append mode to true.
        .create(true) // Create the file if it doesn't exist.
        .open(path_str)?;

    // `writeln!` writes a (plain or formatted) string followed by a newline:
    writeln!(file, "This is a new line appended to the file.")?;
    Ok(())
}

fn read_from_file() -> Result<(), io::Error> {
    let path_str = "temp/lines.txt";

    // Read the entire contents of a file into a string.
    // The contents of the file must be valid UTF-8.
    let message: String = fs::read_to_string(path_str)?;
    println!("`read_to_string`:\n {}", message);

    // Or read the entire contents of a file into a bytes vector:
    let data: Vec<u8> = fs::read("temp/lines.txt")?;
    assert_eq!(data[0..3], [87, 114, 105]);

    // Or open the file for reading:
    let input = File::open(path_str)?;
    // Call a method of the `Read` trait,
    // or add buffering:
    let buffered = BufReader::new(input);
    // Iterate over the lines of this reader.
    // The `?` operator propagates the error if one occurs.
    for line in buffered.lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn main() -> Result<(), io::Error> {
    write_to_file()?;
    read_from_file()?;
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
