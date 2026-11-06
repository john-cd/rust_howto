//! Integration tests for the `clap_builder_xmpl` CLI application.
//!
//! Exercises full CLI binary execution with various flags, subcommands,
//! environment variables, and error cases.
//!
//! TODO Reviewed ??-??-2026.

use std::process::Command;

/// Executes the CLI binary with the given arguments and environment variables.
fn run_cli(args: &[&str], envs: &[(&str, &str)]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_clap_builder_xmpl"));
    command.args(args);
    // Clear potentially conflicting environment variables by default.
    command.env_remove("TOOL_CONFIG_FILE");
    command.env_remove("TOOL_VERBOSE");
    for (key, val) in envs {
        command.env(key, val);
    }
    command
        .output()
        .unwrap_or_else(|error| panic!("failed to execute CLI binary: {error}"))
}

/// Verifies default CLI output when no arguments are provided.
#[test]
fn test_cli_no_arguments() {
    let output = run_cli(&[], &[]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines, ["Debug mode is off", "Command: none"]);
}

/// Verifies verbosity flag behavior (`-v`, `-vv`, `-vvv`, and beyond).
#[test]
fn test_cli_verbosity_levels() {
    let cases = [
        (&["-v"][..], "Debug mode is kind of on"),
        (&["-vv"][..], "Debug mode is on"),
        (&["-vvv"][..], "Don't be crazy"),
        (&["-vvvv"][..], "Don't be crazy"),
    ];

    for (args, expected) in cases {
        let output = run_cli(args, &[]);
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        let first_line = stdout.lines().next().unwrap_or_default();
        assert_eq!(first_line, expected);
    }
}

/// Verifies specifying configuration files via short flag, long flag, and hidden alias.
#[test]
fn test_cli_config_flags() {
    let cases = [
        (&["-c", "app.toml"][..], "Value for config file: app.toml"),
        (
            &["--config", "custom.toml"][..],
            "Value for config file: custom.toml",
        ),
        (
            &["--configuration", "alias.toml"][..],
            "Value for config file: alias.toml",
        ),
    ];

    for (args, expected) in cases {
        let output = run_cli(args, &[]);
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains(expected));
    }
}

/// Verifies passing positional file paths directly as arguments.
#[test]
fn test_cli_positional_filepaths() {
    let output = run_cli(&["data1.csv", "data2.csv"], &[]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(r#"Command: open ["data1.csv", "data2.csv"]"#));
}

/// Verifies the `open` subcommand and its interaction with global config flags.
#[test]
fn test_cli_subcommand_open() {
    let output = run_cli(&["-c", "cfg.toml", "open", "file1.txt", "file2.txt"], &[]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Value for config file: cfg.toml"));
    assert!(stdout.contains(r#"Command: open ["file1.txt", "file2.txt"]"#));
}

/// Verifies the `query` subcommand with multiple terms.
#[test]
fn test_cli_subcommand_query() {
    let output = run_cli(&["query", "SELECT", "*", "FROM", "users"], &[]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(r#"Command: query ["SELECT", "*", "FROM", "users"]"#));
}

/// Verifies the `test` subcommand and abbreviated subcommand matching.
#[test]
fn test_cli_subcommand_test_and_inferred() {
    let output_exact = run_cli(&["test"], &[]);
    assert!(output_exact.status.success());
    let stdout_exact = String::from_utf8_lossy(&output_exact.stdout);
    assert!(stdout_exact.contains("Command: test"));

    // Clap is configured with `infer_subcommands(true)`, so `te` should resolve to `test`.
    let output_inferred = run_cli(&["te"], &[]);
    assert!(output_inferred.status.success());
    let stdout_inferred = String::from_utf8_lossy(&output_inferred.stdout);
    assert!(stdout_inferred.contains("Command: test"));
}

/// Verifies environment variables `TOOL_CONFIG_FILE` and `TOOL_VERBOSE`.
#[test]
fn test_cli_environment_variables() {
    let output = run_cli(
        &[],
        &[
            ("TOOL_CONFIG_FILE", "env_config.toml"),
            ("TOOL_VERBOSE", "2"),
        ],
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Debug mode is on"));
    assert!(stdout.contains("Value for config file: env_config.toml"));
}

/// Verifies invalid arguments return a non-zero exit status and produce stderr diagnostics.
#[test]
fn test_cli_invalid_arguments() {
    // Missing required value for `--config`
    let output_missing_val = run_cli(&["--config"], &[]);
    assert!(!output_missing_val.status.success());
    let stderr = String::from_utf8_lossy(&output_missing_val.stderr);
    assert!(stderr.contains("a value is required"));

    // Unknown flag
    let output_unknown_flag = run_cli(&["--unknown-flag"], &[]);
    assert!(!output_unknown_flag.status.success());
    let stderr_unknown = String::from_utf8_lossy(&output_unknown_flag.stderr);
    assert!(stderr_unknown.contains("unexpected argument"));
}

/// Verifies short and long help, as well as `--version`, display expected information with success
/// status.
#[test]
fn test_cli_help_and_version() {
    let output_short_help = run_cli(&["-h"], &[]);
    assert!(output_short_help.status.success());
    let stdout_short_help = String::from_utf8_lossy(&output_short_help.stdout);
    assert!(stdout_short_help.contains("A demo CLI-based tool"));

    let output_long_help = run_cli(&["--help"], &[]);
    assert!(output_long_help.status.success());
    let stdout_long_help = String::from_utf8_lossy(&output_long_help.stdout);
    assert!(stdout_long_help.contains("A tool that can open one or more files or query data."));
    assert!(stdout_long_help.contains("Usage:"));

    let output_version = run_cli(&["--version"], &[]);
    assert!(output_version.status.success());
    let stdout_version = String::from_utf8_lossy(&output_version.stdout);
    assert!(stdout_version.contains(env!("CARGO_PKG_VERSION")));
}
