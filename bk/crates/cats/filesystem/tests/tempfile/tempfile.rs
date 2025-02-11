// ANCHOR: example
use std::io::Write;

use tempfile::Builder;
use tempfile::tempdir;
use tempfile::tempfile;

// Temporary files and directories.

fn main() -> std::io::Result<()> {
    // 1. Create a temporary file (inside of `env::temp_dir()`).
    // It will be automatically removed by the OS
    // when the last handle to it is closed.
    let mut temp_file = tempfile()?;
    writeln!(temp_file, "This is temporary.")?;

    // 2. Create a temporary directory (inside of `env::temp_dir()`).
    let temp_dir = tempdir()?;
    let temp_file_path = temp_dir.path().join("temporary_file");
    let mut temp_file = std::fs::File::create(temp_file_path)?;
    writeln!(temp_file, "This is also temporary.")?;

    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `tmp_dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    drop(temp_file);
    temp_dir.close()?;

    // 3. Create a named temporary file
    // Note: NamedTempFile rely on Rust destructors for cleanup.
    // They may fail to run if the process exits through an unhandled signal
    // interrupt.
    let mut temp_file = Builder::new()
        .prefix("my-temporary-file")
        .suffix(".txt")
        .rand_bytes(6)
        .tempfile()?;
    // or, for example: let mut temp_file = tempfile::NamedTempFile::new()?;

    writeln!(temp_file, "Hello, temporary file!")?;

    // Print the path of the temporary file
    println!("Temporary file created at: {:?}", temp_file.path());

    // Read the content back from the temporary file
    let mut file = temp_file.reopen()?;
    let mut content = String::new();
    std::io::Read::read_to_string(&mut file, &mut content)?;

    // Print the content
    println!("Content of temporary file: {}", content);

    Ok(())
}
// Adapted from https://docs.rs/tempfile
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
