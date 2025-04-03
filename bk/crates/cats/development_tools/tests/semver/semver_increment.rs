// ANCHOR: example
//! This example demonstrates how to parse a semantic version string and
//! how to increment the major, minor, and patch components.

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
    // increment_*() no longer exist;
    // https://docs.rs/semver/latest/semver/struct.Version.html
    //
    // let mut patch_version = parsed_version.clone();
    // patch_version.increment_patch();
    // assert_eq!(patch_version.to_string(), "0.2.7");
    // println!("New patch release: v{}", patch_version);

    // let mut minor_version = parsed_version.clone();
    // minor_version.increment_minor();
    // assert_eq!(minor_version.to_string(), "0.3.0");
    // println!("New minor release: v{}", minor_version);

    // let mut major_version = parsed_version.clone();
    // major_version.increment_major();
    // assert_eq!(major_version.to_string(), "1.0.0");
    // println!("New major release: v{}", major_version);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> Result<()> {
    main()?;
    Ok(())
}
// [finish; increment_*() no longer exist; remove example maybe NOW](https://github.com/john-cd/rust_howto/issues/903)
