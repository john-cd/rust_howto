// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Run a child command with `duct` and capture its output.
//!
//! A child process is another program started by yours. `duct` builds commands
//! and pipelines with a short syntax and captures their output. Always treat
//! the output as text from an outside program that can fail.

use std::path::Path;

use duct::cmd;

/// Reads the short Git status in the given directory.
///
/// # Errors
///
/// Returns an error if the Git child process fails.
fn git_status(directory: &Path) -> std::io::Result<String> {
    cmd!("git", "status", "--short", "--ignore-submodules=all")
        .dir(directory)
        .read()
}

/// Runs `git status --short --ignore-submodules=all` and prints its output.
fn main() -> anyhow::Result<()> {
    let output = git_status(&std::env::current_dir()?)?;
    println!("{output}");
    Ok(())
}
// ANCHOR_END: example

/// Runs `git status --short --ignore-submodules=all` and prints its output.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Verifies that `duct` can run Git and capture its output.
#[cfg(test)]
mod tests {
    use super::*;

    /// Reports an untracked file from a temporary Git repository.
    #[test]
    fn reports_git_status_in_a_temporary_checkout() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        cmd!("git", "init", "--quiet").dir(dir.path()).run()?;
        std::fs::write(dir.path().join("duct-status-example.txt"), b"example")?;
        let status = git_status(dir.path())?;
        assert_eq!(status, "?? duct-status-example.txt");
        Ok(())
    }

    /// `duct` captures the standard output of a command.
    #[test]
    fn duct_captures_output() -> anyhow::Result<()> {
        let output = cmd!("git", "--version").read()?;
        assert!(output.starts_with("git version"), "{output}");
        Ok(())
    }

    /// A failing command is reported as an error.
    #[test]
    fn duct_reports_failure() {
        assert!(
            cmd!("git", "no-such-subcommand")
                .stderr_null()
                .read()
                .is_err()
        );
    }
}
