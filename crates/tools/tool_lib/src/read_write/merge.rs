//! Manipulate src/refs/*-refs.md files that contain all the reference
//! definitions for the book.

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use itertools::Itertools;

// Merge the contents of a text file and a Vec of text lines,
// - Create a backup copy of the source text file
// - Sort and deduplicate the merged lines
// - Write back to the same file
pub fn merge<P: AsRef<Path>>(
    filepath: P,
    new_lines: Vec<String>,
) -> Result<()> {
    let filepath = filepath.as_ref();

    // Create a backup copy of the file...
    super::backup(filepath)?;

    let merged_contents: Vec<String>;
    {
        let buf = BufReader::new(File::open(filepath)?);
        merged_contents = buf
            .lines()
            .map(|l| l.unwrap())
            .chain(new_lines)
            .sorted()
            .dedup()
            .map(|s| s + "\n")
            .collect();
    }
    write_each(filepath, &merged_contents)?;
    Ok(())
}

// Vec<String> and &Vec<String> are both AsRef<[u8]>
fn write_each(
    path: impl AsRef<Path>,
    items: impl IntoIterator<Item = impl AsRef<[u8]>>,
) -> Result<()> {
    let mut file = File::create(path)?;
    for i in items {
        file.write_all(i.as_ref())?;
    }
    // Surface any I/O errors that could otherwise be swallowed when
    // the file is closed implicitly by being dropped.
    file.sync_all()?;
    Ok(())
}
