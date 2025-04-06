#![cfg(target_family = "unix")]
// ANCHOR: example
//! This example demonstrates how to send input to an external command's
//! standard input (stdin) and process its standard output (stdout) and standard
//! error (stderr).
//!
//! It uses the `rev` command, which reverses each line of its input.
//!
//! The example sends the string "1234 56789" to `rev` and then checks if the
//! output is successful. If successful, it parses the output, splits it into
//! words, converts them to lowercase, and stores them in a HashSet to get
//! unique words.

use std::collections::HashSet;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;

fn main() -> Result<()> {
    let mut child = Command::new("rev")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or(anyhow!("Child process' stdin has not been captured!"))?
        .write_all(b"1234 56789")?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let words = raw_output
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect::<HashSet<_>>();
        println!("Found {} unique words:", words.len());
        println!("{:#?}", words);
    } else {
        let err = String::from_utf8(output.stderr)?;
        bail!("External command failed:\n {}", err);
    }
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
