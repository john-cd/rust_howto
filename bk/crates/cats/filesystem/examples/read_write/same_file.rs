#![allow(dead_code)]
// ANCHOR: example
use std::io::Error;

use same_file::Handle;

/// Check whether we are reading & writing to the same file.
fn main() -> Result<(), Error> {
    // Get a `Handle` for the standard input, output, and error:
    let stdin = Handle::stdin()?;
    let stdout = Handle::stdout()?;
    let stderr = Handle::stderr()?;

    // Check if the standard input is the same as the standard output or error
    // handle:
    if (stdin == stdout) | (stdin == stderr) {
        println!("You are reading and writing to the same file!");
        // Return an error to exit the process:
        // return Err(Error::other(
        //     "You are reading and writing to the same file!",
        // ));
    }
    if stdout == stderr {
        println!("stdout == stderr");
    }

    // Use `from_path` to get a `Handle` for a file.
    // Note that the underlying `File` is opened in read-only mode on all
    // platforms:
    // let path_to_read = Path::new("temp/new.txt");
    // let _handle = Handle::from_path(path_to_read)?;

    // You may also check whether two file paths correspond to the same file
    // (due to hard or symbolic links):
    #[cfg(unix)]
    assert!(same_file::is_same_file("/bin/sh", "/usr/bin/sh")?);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
