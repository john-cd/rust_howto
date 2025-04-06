#![cfg(target_family = "unix")]
// ANCHOR: example
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

/// This example demonstrates how to capture both standard output and standard
/// error from an external command and redirect them to the same file.
///
/// It uses `ls` command with a non-existent directory "oops" to generate an
/// error. Both the standard output (listing of the current directory) and the
/// standard error (error message about "oops") will be written to
/// "temp/out.txt".
fn main() -> Result<(), std::io::Error> {
    let path = Path::new("temp/out.txt");
    let outputs = File::create(path)?;
    // Creates a new File instance that shares the same underlying file handle
    // as the existing File instance. Reads, writes, and seeks will affect both
    // File instances simultaneously.
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args([".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    // Preparation
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    let path = Path::new("temp/out.txt");
    if path.exists() {
        fs::remove_file(path)?;
    }
    main()?;
    Ok(())
}
// [review; print; NOW](https://github.com/john-cd/rust_howto/issues/167)
