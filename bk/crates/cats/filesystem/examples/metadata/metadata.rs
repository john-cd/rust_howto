#![allow(dead_code)]
// ANCHOR: example
use std::fs;
use std::time::UNIX_EPOCH;

fn inspect_file<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<()> {
    let path = path.as_ref();

    // Attempt to get metadata for the given path:
    let metadata = fs::metadata(path)?;

    println!("Metadata for: {}", path.display());
    println!(
        "  Type: {}",
        if metadata.is_dir() {
            "Directory"
        } else if metadata.is_file() {
            "File"
        } else {
            "Other"
        }
    );
    println!("  Size: {} bytes", metadata.len());
    println!("  Read-only: {}", metadata.permissions().readonly());

    // Handle the modification time:
    match metadata.modified() {
        Ok(mod_time) => {
            let duration = mod_time.duration_since(UNIX_EPOCH)?;
            println!(
                "Last modified: {} seconds since Unix epoch",
                duration.as_secs()
            );
        }
        Err(e) => eprintln!("Could not retrieve modification time: {}", e),
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Create a few folders and files for the next step.
    let path = "./temp/files/subfolder";
    fs::create_dir_all(path)?; // Creates 'temp', 'temp/files', and 'temp/files/subfolder' if they don't exist.
    fs::write("./temp/files/file1.txt", "Hello")?;
    fs::write("./temp/files/subfolder/file2.log", "World")?;

    // Show permissions for files and folders directly within `temp/files/`.
    // `read_dir` returns an iterator over the directory's entries.
    // `flatten` is used to skip over any entries that resulted in an error.
    for entry in (fs::read_dir("./temp/files")?).flatten() {
        let path = entry.path();
        inspect_file(path)?;
        // You could also use `entry.metadata()` directly in this case.
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
