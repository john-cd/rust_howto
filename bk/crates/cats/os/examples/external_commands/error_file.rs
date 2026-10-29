// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to capture both standard output and standard
//! error from an external command and redirect them to the same file.

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
fn main() -> anyhow::Result<()> {
    let path = Path::new("temp/out.txt");
    let outputs = File::create(path)?;
    // Creates a new `File` instance that shares the same underlying file handle
    // as the existing File instance. Reads, writes, and seeks will affect both
    // `File` instances simultaneously.
    let errors = outputs.try_clone()?;

    let status = Command::new("ls")
        .args([".", "oops"])
        .stdout(Stdio::from(outputs))
        .stderr(Stdio::from(errors))
        .spawn()?
        .wait()?;

    if status.success() {
        anyhow::bail!(
            "`ls` unexpectedly succeeded for the missing path `oops`."
        );
    }
    println!("`ls` exited with {status}, as expected for the missing path.");

    // Print contents of the file.
    std::fs::read_to_string(path)?
        .lines()
        .for_each(|line| println!("{line}"));

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the error file example's main path succeeds.
    /// Runs the error file example through its entry point.
    #[test]
    fn test_error_file_example_entry_point() -> anyhow::Result<()> {
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

    /// Both standard output and standard error end up in the same file.
    #[test]
    fn output_and_error_share_the_file() -> anyhow::Result<()> {
        let path = std::env::temp_dir().join("rust_howto_error_file.txt");
        let outputs = File::create(&path)?;
        let errors = outputs.try_clone()?;
        let status = Command::new("ls")
            .args([".", "oops"])
            .stdout(Stdio::from(outputs))
            .stderr(Stdio::from(errors))
            .status()?;
        assert!(!status.success());
        let contents = std::fs::read_to_string(&path)?;
        assert!(contents.contains("oops"), "{contents}");
        std::fs::remove_file(&path)?;
        Ok(())
    }
}
