#![allow(dead_code)]
#![cfg(not(windows))]
// ANCHOR: example
use std::process::Command;

use anyhow::Context;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use semver::Version;
use semver::VersionReq;

/// This example demonstrates how to check if a command line tool is installed
/// and if it meets a minimum version requirement.
///
/// It uses the `git` command as an example, but it can be adapted to any
/// command line tool.
///
/// It uses the `semver` crate to parse and compare versions.
fn main() -> Result<()> {
    // Define the minimum required version.
    let version_constraint = "> 1.12.0";
    let version_test = VersionReq::parse(version_constraint)?;
    // Execute the command and capture its output.
    let output = Command::new("git").arg("--version").output()?;

    if !output.status.success() {
        bail!("The command returned a failing error code.");
    }

    let stdout = String::from_utf8(output.stdout)?;
    let version = stdout
        .split(" ")
        .last()
        .ok_or_else(|| anyhow!("Invalid command output."))?
        .trim(); // Remove any extraneous newlines.
    // Parse the version string into a `semver::Version`.
    let parsed_version =
        Version::parse(version).context(format!("version: {version}"))?;

    // Check if the parsed version meets the required version constraint.
    if !version_test.matches(&parsed_version) {
        bail!(
            "Command version lower than minimum supported version (found {}, need {})",
            parsed_version,
            version_constraint
        );
    }
    println!("{parsed_version:?}");
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
