// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Test a compiled command-line executable with assert_cmd and predicates.

use assert_cmd::Command;
use predicates::prelude::*;

/// Checks that `--help` lists the `--name` option.
///
/// # Errors
///
/// Returns an error if the test executable cannot be located.
fn check_help_lists_the_name_option() -> anyhow::Result<()> {
    Command::cargo_bin("cli-demo")?
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--name"));
    Ok(())
}

/// Checks that `--name Ada` prints the requested greeting.
///
/// # Errors
///
/// Returns an error if the test executable cannot be located.
fn check_success_prints_the_requested_greeting() -> anyhow::Result<()> {
    Command::cargo_bin("cli-demo")?
        .args(["--name", "Ada"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello, Ada!"));
    Ok(())
}

/// Checks that `--fail` exits unsuccessfully and prints its diagnostic.
///
/// # Errors
///
/// Returns an error if the test executable cannot be located.
fn check_failure_reports_an_error() -> anyhow::Result<()> {
    Command::cargo_bin("cli-demo")?
        .arg("--fail")
        .assert()
        .failure()
        .stderr(predicate::str::contains("requested failure"));
    Ok(())
}
// ANCHOR_END: example

/// Runs the command-line integration checks in sequence.
///
/// # Errors
///
/// Returns an error if the example operation fails.
pub fn run() -> anyhow::Result<()> {
    check_help_lists_the_name_option()?;
    check_success_prints_the_requested_greeting()?;
    check_failure_reports_an_error()?;
    Ok(())
}

/// Covers the CLI help, success, and failure cases.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that help output lists the name option.
    #[test]
    fn help_lists_the_name_option() {
        check_help_lists_the_name_option().expect("the `cli-demo` binary is available");
    }

    /// Verifies that a successful invocation prints the greeting.
    #[test]
    fn success_prints_the_requested_greeting() {
        check_success_prints_the_requested_greeting().expect("the `cli-demo` binary is available");
    }

    /// Verifies that a failing invocation reports its diagnostic.
    #[test]
    fn failure_reports_an_error() {
        check_failure_reports_an_error().expect("the `cli-demo` binary is available");
    }
}
