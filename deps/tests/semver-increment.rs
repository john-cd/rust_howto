use std::error::Error;

use semver::Version;

#[test]
fn test() -> Result<(), Box<dyn Error>> {
    let parsed_version = Version::parse("0.2.6")?;

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

    // TODO increment_*() no longer exist; remove example?

    // parsed_version.increment_patch();
    // assert_eq!(parsed_version.to_string(), "0.2.7");
    // println!("New patch release: v{}", parsed_version);

    // parsed_version.increment_minor();
    // assert_eq!(parsed_version.to_string(), "0.3.0");
    // println!("New minor release: v{}", parsed_version);

    // parsed_version.increment_major();
    // assert_eq!(parsed_version.to_string(), "1.0.0");
    // println!("New major release: v{}", parsed_version);

    Ok(())
}
