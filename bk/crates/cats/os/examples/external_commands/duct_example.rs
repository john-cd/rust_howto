// ANCHOR: example
//! Run a child command with `duct` and capture its output.

use duct::cmd;

/// Runs `git status --short --ignore-submodules=all` and prints its output.
fn main() -> anyhow::Result<()> {
    let output =
        cmd!("git", "status", "--short", "--ignore-submodules=all").read()?;
    println!("{output}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `duct` can run Git and capture its output.
    /// Runs the duct example through its entry point.
    #[test]
    fn test_duct_example_entry_point() -> anyhow::Result<()> {
        main()
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
