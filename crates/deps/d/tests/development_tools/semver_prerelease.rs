// ANCHOR: example
use anyhow::Result;
use semver::Version;

fn main() -> Result<()> {
    let version_1 = Version::parse("1.0.0-alpha")?;
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
