// ANCHOR: example
use anyhow::Result;
use semver::BuildMetadata;
use semver::Prerelease;
use semver::Version;

fn main() -> Result<()> {
    let version_str = "1.0.49-125+g72ee7853";
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

    let serialized_version = parsed_version.to_string();
    assert_eq!(&serialized_version, version_str);

    Ok(())
}

// ANCHOR_END: example
#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
