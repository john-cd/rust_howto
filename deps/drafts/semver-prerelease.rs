use anyhow::Result;
use semver::Version;

fn main() -> Result<()> {
    let version_1 = Version::parse("1.0.0-alpha")?;
    let version_2 = Version::parse("1.0.0")?;

    assert!(version_1.is_prerelease());
    assert!(!version_2.is_prerelease());

    Ok(())
}
