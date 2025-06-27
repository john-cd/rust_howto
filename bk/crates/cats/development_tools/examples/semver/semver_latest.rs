#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to find the maximum matching version from a
//! list of versions based on a given version requirement string, using the
//! `semver` crate.
//!
//! The `find_max_matching_version` function takes a version requirement string
//! and an iterable of version strings. It parses the version requirement and
//! each version string, then filters the versions to find those that match the
//! requirement. Finally, it returns the maximum matching version.

use anyhow::Result;
use semver::Version;
use semver::VersionReq;

/// Finds the maximum matching version from an iterable of version strings based
/// on a version requirement string.
fn find_max_matching_version<'a, I>(
    // The version requirement string (e.g., ">=1.0.0", "<2.0.0").
    version_req_str: &str,
    iterable: I,
) -> Result<Option<Version>>
where
    I: IntoIterator<Item = &'a str>,
{
    // SemVer version requirement describing the intersection of some version
    // comparators, such as >=1.2.3, <1.8.
    let vreq = VersionReq::parse(version_req_str)?;

    Ok(iterable
        .into_iter()
        .filter_map(|s| Version::parse(s).ok())
        .filter(|s| vreq.matches(s))
        .max())
}

fn main() -> Result<()> {
    let max_matching_version =
        find_max_matching_version("<= 1.0.0", vec!["0.9.0", "1.0.0", "1.0.1"])?;
    println!("Maximum matching version: {max_matching_version:?}");
    assert_eq!(max_matching_version, Some(Version::parse("1.0.0")?));

    assert_eq!(
        find_max_matching_version(
            ">1.2.3-alpha.3",
            vec![
                "1.2.3-alpha.3",
                "1.2.3-alpha.4",
                "1.2.3-alpha.10",
                "1.2.3-beta.4",
                "3.4.5-alpha.9",
            ]
        )?,
        Some(Version::parse("1.2.3-beta.4")?)
    );

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
