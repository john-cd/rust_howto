#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates reading from a memory-mapped file.

use std::fs::File;
use std::io::Error;
use std::io::Write;

use memmap2::Mmap;
use memmap2::MmapMut;
use memmap2::MmapOptions;

fn main() -> Result<(), Error> {
    write!(
        File::create("temp/content.txt")?,
        "My hovercraft is full of eels!"
    )?;
    // Open the file for reading:
    let file = File::open("temp/content.txt")?;
    // Create a read-only memory map of the file. This is unsafe because it
    // relies on the file not being modified externally:
    let map = unsafe { Mmap::map(&file)? };
    // `Mmap` is `Deref<Target = [u8]>` and therefore can be used like a slice.
    // Assert that a slice of the memory map matches the expected bytes:
    assert_eq!(&map[3..13], b"hovercraft");
    // Collect bytes from the memory map at random indexes:
    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    let random_bytes: Vec<u8> =
        random_indexes.iter().map(|&idx| map[idx]).collect();
    assert_eq!(&random_bytes[..], b"My loaf!");
    // A file-backed memory map remains valid even after the File is dropped.

    // You also create mutable memory-mapped buffers - either file-backed or
    // anonymous - with `MmapMut`.
    // - A file-backed buffer may be used to read from or write to a file.
    // - An anonymous buffer may be used any place that an in-memory byte buffer
    //   is needed.
    let mut mmap_mut: MmapMut = MmapOptions::new().len(13).map_anon()?;
    mmap_mut.copy_from_slice(b"Hello, world!");

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
