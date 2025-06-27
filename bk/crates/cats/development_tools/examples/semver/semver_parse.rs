#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to parse a semantic version string.
//!
//! `semver` is a parser for Cargo's flavor of Semantic Versioning.

use anyhow::Result;
use semver::Version;

fn main() -> Result<()> {
    let parsed_version = Version::parse("0.2.6")?;
    // The parsed version is equal to the following:
    assert_eq!(
        parsed_version,
        Version {
            major: 0,
            minor: 2,
            patch: 6,
            pre: semver::Prerelease::EMPTY,
            build: semver::BuildMetadata::EMPTY,
        }
    );
    println!("{parsed_version:?}");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
