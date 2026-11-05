// TODO Reviewed ??-??-2026.

//! CLI integration tests for `templ`.

use std::process::Command;

/// Checks that the `rbe` command formats Rust By Example badges correctly.
#[test]
fn test_cli_rbe_badge_formatting() {
    let output = Command::new(env!("CARGO_BIN_EXE_templ"))
        .args(["rbe", "attributes", "traits"])
        .output()
        .expect("binary should execute");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("[![Rust by example - attributes][book~rust-by-example~attributes~badge]][book~rust-by-example~attributes]"),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("[book~rust-by-example~attributes~badge]: https://img.shields.io/badge/Rust_By_Example-attributes-violet?logo=mdbook"),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("[book~rust-by-example~attributes]: https://doc.rust-lang.org/rust-by-example/attributes.html"),
        "stdout: {stdout}"
    );

    assert!(
        stdout.contains("[![Rust by example - traits][book~rust-by-example~traits~badge]][book~rust-by-example~traits]"),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("[book~rust-by-example~traits]: https://doc.rust-lang.org/rust-by-example/traits.html"),
        "stdout: {stdout}"
    );
}

/// Checks that `category_badge` formats category badges correctly.
#[test]
fn test_cli_category_badge_formatting() {
    let output = Command::new(env!("CARGO_BIN_EXE_templ"))
        .args(["category_badge", "web_programming"])
        .output()
        .expect("binary should execute");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        stdout.contains("[![cat~web-programming][cat~web-programming~badge]][cat~web-programming]{{hi:Web programming}}"),
        "stdout: {stdout}"
    );
}

/// Checks that calling `templ` with no arguments reports an error.
#[test]
fn test_cli_no_command_fails() {
    let output = Command::new(env!("CARGO_BIN_EXE_templ"))
        .output()
        .expect("binary should execute");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("You did not enter a command!"),
        "stderr: {stderr}"
    );
}

/// Checks that an unknown category reports an informative error.
#[test]
fn test_cli_unknown_category_fails() {
    let output = Command::new(env!("CARGO_BIN_EXE_templ"))
        .args(["category_badge", "nonexistent_category_slug_xyz"])
        .output()
        .expect("binary should execute");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Could not find the desired category!"),
        "stderr: {stderr}"
    );
}

/// Checks that missing required arguments reports an error.
#[test]
fn test_cli_missing_required_args_fails() {
    let output = Command::new(env!("CARGO_BIN_EXE_templ"))
        .arg("rbe")
        .output()
        .expect("binary should execute");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("required arguments were not provided") || stderr.contains("<CONCEPT>"),
        "stderr: {stderr}"
    );
}
