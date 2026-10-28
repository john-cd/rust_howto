// ANCHOR: example
//! Run a child command with `duct` and capture its output.

use duct::cmd;

/// Runs `git status --short` and prints its output.
fn main() -> anyhow::Result<()> {
    let output = cmd!("git", "status", "--short").read()?;
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
}
