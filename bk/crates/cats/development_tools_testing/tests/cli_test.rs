// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Test a compiled command-line executable with assert_cmd and predicates.

use assert_cmd::Command;
use predicates::prelude::*;

/// Verifies that the executable documents its name option in help output.
///
/// # Errors
///
/// Returns an error if the test executable cannot be located.
#[test]
fn help_lists_the_name_option() -> anyhow::Result<()> {
    Command::cargo_bin("cli-demo")?
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--name"));
    Ok(())
}

/// Verifies that a successful invocation prints the requested greeting.
///
/// # Errors
///
/// Returns an error if the test executable cannot be located.
#[test]
fn success_prints_the_requested_greeting() -> anyhow::Result<()> {
    Command::cargo_bin("cli-demo")?
        .args(["--name", "Ada"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Ada!"));
    Ok(())
}

/// Verifies that failure returns a nonzero status and a diagnostic.
///
/// # Errors
///
/// Returns an error if the test executable cannot be located.
#[test]
fn failure_reports_an_error() -> anyhow::Result<()> {
    Command::cargo_bin("cli-demo")?
        .arg("--fail")
        .assert()
        .failure()
        .stderr(predicate::str::contains("requested failure"));
    Ok(())
}
// ANCHOR_END: example
