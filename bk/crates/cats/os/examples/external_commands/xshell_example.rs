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
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
