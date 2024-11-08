// ANCHOR: example
use std::fs::File;
use std::io::Error;
use std::io::Write;

use memmap2::Mmap;

fn main() -> Result<(), Error> {
    write!(
        File::create("temp/content.txt")?,
        "My hovercraft is full of eels!"
    )?;
    let file = File::open("temp/content.txt")?;
    let map = unsafe { Mmap::map(&file)? };

    let random_indexes = [0, 1, 2, 19, 22, 10, 11, 29];
    assert_eq!(&map[3..13], b"hovercraft");
    let random_bytes: Vec<u8> =
        random_indexes.iter().map(|&idx| map[idx]).collect();
    assert_eq!(&random_bytes[..], b"My loaf!");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
