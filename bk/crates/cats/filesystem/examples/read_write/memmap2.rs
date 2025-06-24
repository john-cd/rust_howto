#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates reading from a memory-mapped file.

use std::fs::File;
use std::io::Error;
use std::io::Write;

use memmap2::Mmap;

fn main() -> Result<(), Error> {
    write!(
        File::create("temp/content.txt")?,
        "My hovercraft is full of eels!"
    )?;
    // Open the file for reading.
    let file = File::open("temp/content.txt")?;
    // Create a memory map of the file. This is unsafe because it relies on the
    // file not being modified externally.
    let map = unsafe { Mmap::map(&file)? };
    // Print the memory map (for debugging purposes).
    println!("{:?}", map);
    // Define some random indexes into the memory map.
    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    // Assert that a slice of the memory map matches the expected bytes.
    assert_eq!(&map[3..13], b"hovercraft");
    // Collect bytes from the memory map at the random indexes.
    let random_bytes: Vec<u8> =
        random_indexes.iter().map(|&idx| map[idx]).collect();
    assert_eq!(&random_bytes[..], b"My loaf!");
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
