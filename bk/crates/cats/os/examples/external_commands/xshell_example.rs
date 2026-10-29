// ANCHOR: example
//! Run a Rust tool with `xshell` and capture its output.

use xshell::Shell;
use xshell::cmd;

/// Runs `rustc --version` and prints the captured output.
fn main() -> anyhow::Result<()> {
    let shell = Shell::new()?;
    let version = cmd!(shell, "rustc --version").read()?;
    println!("{version}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that `xshell` can run a tool and capture its output.
    /// Runs the xshell example through its entry point.
    #[test]
    fn test_xshell_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// The captured output is the compiler version.
    #[test]
    fn captures_the_compiler_version() -> anyhow::Result<()> {
        let shell = Shell::new()?;
        let version = cmd!(shell, "rustc --version").read()?;
        assert!(version.starts_with("rustc "), "{version}");
        Ok(())
    }

    /// A failing command is reported as an error.
    #[test]
    fn reports_a_failing_command() -> anyhow::Result<()> {
        let shell = Shell::new()?;
        assert!(
            cmd!(shell, "rustc --no-such-flag")
                .ignore_stderr()
                .read()
                .is_err()
        );
        Ok(())
    }
}
