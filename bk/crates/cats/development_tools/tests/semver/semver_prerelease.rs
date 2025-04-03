// ANCHOR: example
//! Demonstrates how to work with pre-release versions using the `semver` crate.

use anyhow::Result;
use semver::Version;

fn main() -> Result<()> {
    let version_1 = Version::parse("1.0.0-alpha")?;
    // Prints: Version { major: 1, minor: 0, patch: 0, pre: [Identifier(Alpha)], build: [] }
    println!("{:?}", version_1);
    let version_2 = Version::parse("1.0.0")?;
    println!("{:?}", version_2);

    assert!(!version_1.pre.is_empty());
    assert!(version_2.pre.is_empty());

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
