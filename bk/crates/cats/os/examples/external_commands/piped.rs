// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to pipe the output of one external command to
//! the input of another.

#![allow(dead_code)]
#![allow(clippy::single_match)]
#![cfg(target_family = "unix")]

use std::process::Command;
use std::process::Stdio;

/// This example demonstrates how to pipe the output of one external command to
/// the input of another.
///
/// It uses the `du`, `sort`, and `head` commands to find the top 10 largest
/// files and directories in the current directory.
///
/// The `du` command is used to get the disk usage of each file and directory.
/// The `sort` command is used to sort the output of `du` in reverse
/// human-readable order. The Rust code prints the first 10 sorted lines after
/// checking both child processes' exit statuses.
fn main() -> anyhow::Result<()> {
    let directory = std::env::current_dir()?;
    let mut du_output_child = Command::new("du")
        .arg("-ah")
        .arg(&directory)
        .stdout(Stdio::piped())
        .spawn()?;

    let du_output = du_output_child
        .stdout
        .take()
        .ok_or_else(|| std::io::Error::other("`du` stdout was not captured"))?;
    let mut sort_output_child = Command::new("sort")
        .arg("-hr")
        .stdin(du_output)
        .stdout(Stdio::piped())
        .spawn()?;
    let sort_output = sort_output_child.wait_with_output()?;
    let du_status = du_output_child.wait()?;

    if !du_status.success() {
        anyhow::bail!("`du` exited with status {du_status}.");
    }
    if !sort_output.status.success() {
        anyhow::bail!("`sort` exited with status {}.", sort_output.status);
    }

    let sorted_usage = String::from_utf8(sort_output.stdout)?;
    let largest = sorted_usage.lines().take(10).collect::<Vec<_>>().join("\n");
    println!(
        "Top 10 biggest files and directories in '{}':\n{}",
        directory.display(),
        largest
    );

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the piped example through its entry point.
    #[test]
    fn test_piped_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
