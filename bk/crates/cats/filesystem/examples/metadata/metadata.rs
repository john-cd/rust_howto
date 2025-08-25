#![allow(dead_code)]
// ANCHOR: example
use std::fs;

fn main() -> anyhow::Result<()> {
    // Create a few folders and files for the next step.
    let path = "./temp/files/subfolder";
    fs::create_dir_all(path)?; // Creates 'temp', 'temp/files', and 'temp/files/subfolder' if they don't exist.
    fs::write("./temp/files/file1.txt", "Hello")?;
    fs::write("./temp/files/subfolder/file2.log", "World")?;

    // Show permissions for files and folders within `temp/files/`.
    // `read_dir` returns an iterator over the directory's entries.
    // `flatten` is used to skip over any entries that resulted in an error.
    for entry in (fs::read_dir("./temp/files")?).flatten() {
        let path = entry.path();
        if let Ok(metadata) = entry.metadata() {
            println!("{} {:?}", path.display(), metadata.permissions());
        } else {
            println!("{}", path.display());
        }
    }
    fs::remove_dir_all("./temp/files")?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
