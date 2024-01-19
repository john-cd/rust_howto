use anyhow::Result;
use semver::Version;

fn main() -> Result<()> {
    let _version_1 = Version::parse("1.0.0-alpha")?;
    let _version_2 = Version::parse("1.0.0")?;

    // assert!(version_1.is_prerelease());
    // assert!(!version_2.is_prerelease());

    Ok(())
}
