// ANCHOR: example
//! This example demonstrates how to check whether two file paths refer to the
//! same file or directory.

use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

use same_file::Handle;

/// Check whether we are reading & writing to the same file.
fn main() -> Result<(), Error> {
    // Get a Handle for the standard output.
    let stdout_handle = Handle::stdout()?;

    // Get a Handle for the file we want to read.
    let path_to_read = Path::new("temp/new.txt");
    let handle = Handle::from_path(path_to_read)?;

    // Check if the file handle is the same as the standard output handle.
    if stdout_handle == handle {
        return Err(Error::new(
            ErrorKind::Other,
            "You are reading and writing to the same file!",
        ));
    } else {
        // Print the contents of the file.
        let file = File::open(path_to_read)?;
        let file = BufReader::new(file);
        for (num, line) in file.lines().enumerate() {
            println!("{} : {}", num, line?.to_uppercase());
        }
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    // Preparation: Write some data to a file named "new.txt" in the "temp"
    // directory.
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    std::fs::write("temp/new.txt", b"Lorem ipsum")?;
    main()?;
    Ok(())
}
