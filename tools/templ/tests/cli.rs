// TODO Reviewed ??-??-2026

//! Runs the `templ` binary for subcommands that need no network access.

use std::process::Command;
use std::process::Output;

/// Runs the binary with `args`.
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_templ"))
        .args(args)
        .output()
        .expect("the binary should start")
}

/// The `rbe` subcommand prints Markdown badges for each concept.
#[test]
fn test_rbe_prints_badges_for_each_concept() {
    let output = run(&["rbe", "attributes", "traits"]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(
            "[![Rust by example - attributes][book~rust-by-example~attributes~badge]][book~rust-by-example~attributes]"
        ),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains(
            "[book~rust-by-example~traits]: https://doc.rust-lang.org/rust-by-example/traits.html"
        ),
        "stdout: {stdout}"
    );
}

/// Running without a subcommand reports an error.
#[test]
fn test_no_command_reports_error() {
    let output = run(&[]);
    assert!(!output.status.success());
    assert!(!output.stderr.is_empty());
}

/// A subcommand without its required argument is rejected with a usage message.
#[test]
fn test_missing_argument_reports_usage() {
    let output = run(&["rbe"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CONCEPT"), "stderr: {stderr}");
}

/// An unknown subcommand is rejected.
#[test]
fn test_unknown_subcommand_fails() {
    assert!(!run(&["bogus"]).status.success());
}

/// `--version` prints the package version.
#[test]
fn test_version_flag() {
    let output = run(&["--version"]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(env!("CARGO_PKG_VERSION")));
}
