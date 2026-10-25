#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Test a command-line program end to end with `assert_cmd`.

use assert_cmd::Command;
use predicates::prelude::*;

/// Runs `rustc --version` and checks the outcome.
///
/// For a binary of your own package, use
/// `Command::cargo_bin("my-tool")?` instead of `Command::new`; Cargo builds
/// the binary before running integration tests (in `tests/*.rs`).
pub fn run() -> anyhow::Result<()> {
    // Success: the exit code is 0 and `stdout` matches.
    Command::new("rustc")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::starts_with("rustc "));

    // Failure: a non-zero exit code and a message on `stderr`.
    Command::new("rustc")
        .arg("--this-flag-does-not-exist")
        .assert()
        .failure()
        .stderr(predicate::str::contains("this-flag-does-not-exist"));

    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rustc_reports_its_version() -> anyhow::Result<()> {
        run()
    }
}
