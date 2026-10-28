// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates how to work with pre-release versions using the `semver` crate.

use anyhow::Result;
use semver::Version;

/// Parses versions with and without a pre-release identifier.
fn main() -> Result<()> {
    let version_1 = Version::parse("1.0.0-alpha")?;
    // Prints `Version { major: 1, minor: 0, patch: 0, pre: [Identifier(Alpha)],
    // build: [] }`.
    println!("{version_1:?}");
    let version_2 = Version::parse("1.0.0")?;
    println!("{version_2:?}");

    assert!(!version_1.pre.is_empty());
    assert!(version_2.pre.is_empty());

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the semver prerelease example through its entry point.
    #[test]
    fn test_semver_prerelease_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
