// ANCHOR: example
#![cfg(target_family = "unix")]

use std::collections::HashSet;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;

fn main() -> Result<()> {
    let mut child = Command::new("rev")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or(anyhow!("Child process stdin has not been captured!"))?
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
