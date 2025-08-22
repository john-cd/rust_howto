#![allow(dead_code)]
// ANCHOR: example
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use tempfile::Builder;
use tempfile::NamedTempFile;
use tempfile::TempDir;
use tempfile::spooled_tempfile;
use tempfile::tempdir;
use tempfile::tempfile;

fn main() -> std::io::Result<()> {
    // 1. Create a temporary file (inside of `env::temp_dir()`).
    // Use `tempfile()` when you need a real `File` but don't need to refer to
    // it by path.
    let mut temp_file: File = tempfile()?;
    writeln!(temp_file, "This is temporary.")?;

    // Seek to start:
    temp_file.seek(SeekFrom::Start(0))?;

    // Read it:
    let mut buf = String::new();
    temp_file.read_to_string(&mut buf).unwrap();
    assert_eq!(buf, "This is temporary.\n");

    // The temporary file will be automatically removed by the OS when the last
    // handle to it is closed / goes out of scope. The above doesn't rely on
    // Rust destructors being run, so will (almost) never fail to clean up
    // the temporary file.

    // 2. Use `spooled_tempfile()` when you need an in-memory buffer that will
    //    ultimately be backed by a temporary file if it gets too large. This
    //    variant is also secure/reliable in the presence of a pathological
    //    temporary file cleaner.
    let mut file = spooled_tempfile(15);
    writeln!(file, "short")?;
    assert!(!file.is_rolled());
    // As a result of the following write call, the size of the data will exceed
    // `max_size` (15), so it will be written to a temporary file on disk,
    // and the in-memory buffer will be dropped:
    writeln!(file, "much longer than the max size")?;
    assert!(file.is_rolled());

    // 3. Create a temporary directory (inside of `env::temp_dir()`).
    // Use `tempdir()` when you need a temporary directory that will be
    // recursively deleted on drop.
    let temp_dir: TempDir = tempdir()?;
    let temp_file_path = temp_dir.path().join("temporary_file.txt");
    let mut temp_file = File::create(temp_file_path)?;
    writeln!(temp_file, "This is also temporary.")?;

    // `TempDir` rely on Rust destructors for cleanup. Destructors may fail to
    // run if the process exits through an unhandled signal interrupt, or if the
    // instance is declared statically, among other possible reasons.
    // a. Make sure that handles to files inside the directory are dropped
    // before the `TempDir` goes out of scope.
    drop(temp_file);
    // b. By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `tmp_dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    temp_dir.close()?;

    // 4. Use `NamedTempFile::new()` when you need a named temporary file that
    //    can be refered to its path. `NamedTempFile` also rely on Rust
    //    destructors for cleanup, which may fail. Read the documentation and
    //    prefer `tempfile()` in most cases.
    let temp_file0 = NamedTempFile::new()?;
    println!("Temporary file created at: {:?}", temp_file0.path());

    // Use `Builder` to create a new temporary file (or directory) with custom
    // options.
    let mut temp_file = Builder::new()
        .prefix("my-temporary-file")
        .suffix(".txt")
        .rand_bytes(6)
        .tempfile()?;

    writeln!(temp_file, "Hello, named temporary file!")?;

    // Print the path of the named temporary file.
    println!("Temporary file created at: {:?}", temp_file.path());

    // Read the content back from the named temporary file.
    // `NamedTempFile::reopen()` guarantees that the re-opened file is the same
    // file, even in the presence of pathological temporary file cleaners.
    // This function is also useful when you need multiple independent handles
    // to the same file (useful for consumer/producer patterns).
    let mut file = temp_file.reopen()?;
    let mut content = String::new();
    Read::read_to_string(&mut file, &mut content)?;

    drop(temp_file);

    // Print the content of the named temporary file:
    println!("Content of temporary file: {content}");

    drop(file);

    Ok(())
}
// Examples adapted from <https://docs.rs/tempfile>
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
