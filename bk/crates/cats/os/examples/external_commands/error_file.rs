#![allow(dead_code)]
// ANCHOR: example
#![cfg(target_family = "unix")]
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
    // Creates a new `File` instance that shares the same underlying file handle
    // as the existing File instance. Reads, writes, and seeks will affect both
    // `File` instances simultaneously.
    let errors = outputs.try_clone()?;

    Command::new("ls")
        .args([".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait_with_output()?;

    // Print contents of the file.
    std::fs::read_to_string(path)?
        .lines()
        .for_each(|line| println!("{}", line));

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    // Preparation.
    if !std::fs::exists("temp")? {
        std::fs::create_dir("temp")?;
    }
    let path = Path::new("temp/out.txt");
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    main()?;
    Ok(())
}
