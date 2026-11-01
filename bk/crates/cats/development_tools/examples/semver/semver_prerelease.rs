// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates how to work with pre-release versions using the `semver` crate.
//!
//! A pre-release version such as `1.0.0-beta.1` sorts before the final `1.0.0`
//! and is skipped by default requirements, so users do not get unfinished
//! releases by accident.

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
