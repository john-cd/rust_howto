#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to run an external command, capture its
//! output, and parse it.
use std::process::Command;

use anyhow::Result;
use anyhow::bail;
use regex::Regex;

/// Represents a Git commit with its hash and message.
#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

/// Executes the `git log --oneline` command, parses its output, and prints the
/// first 5 commits.
///
/// # Returns
///
/// Returns `Ok(())` if the command executes successfully and the output is
/// parsed correctly. Returns an error if the command fails or the output cannot
/// be parsed.
fn main() -> Result<()> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;
    if !output.status.success() {
        bail!("Command executed with failing error code.");
    }

    let pattern = Regex::new(
        r"(?x)
        ([0-9a-fA-F]+) # commit hash
        (.*)           # The commit message",
    )?;

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| Commit {
            hash: cap[1].to_string(),
            message: cap[2].trim().to_string(),
        })
        .take(5)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}
// ANCHOR_END: example

// This test should not run during CI.
#[test]
fn ignore_in_ci() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
