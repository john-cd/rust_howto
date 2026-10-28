// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Check whether we are reading & writing to the same file.

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
    if (stdin == stdout) || (stdin == stderr) {
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

    // Compare two path spellings for the same file without platform-specific
    // paths.
    let temp_dir = tempfile::tempdir()?;
    let file = temp_dir.path().join("same.txt");
    let equivalent_path = temp_dir.path().join(".").join("same.txt");
    std::fs::File::create(&file)?;
    assert!(same_file::is_same_file(&file, &equivalent_path)?);

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the same file example through its entry point.
    #[test]
    fn test_same_file_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
