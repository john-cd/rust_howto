#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates parsing, comparing, and serializing complex semantic version
//! strings.

use anyhow::Result;
use semver::BuildMetadata;
use semver::Prerelease;
use semver::Version;

fn main() -> Result<()> {
    // Define a complex semantic version string.
    let version_str = "1.0.49-125+g72ee7853";
    // Parse the version string into a Version struct.
    let parsed_version = Version::parse(version_str)?;

    assert_eq!(
        parsed_version,
        Version {
            major: 1,
            minor: 0,
            patch: 49,
            pre: Prerelease::new("125")?,
            build: BuildMetadata::new("g72ee7853")?,
        }
    );

    // Serialize the parsed Version struct back into a string.
    let serialized_version = parsed_version.to_string();

    assert_eq!(&serialized_version, version_str);
    println!("{}", serialized_version);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
