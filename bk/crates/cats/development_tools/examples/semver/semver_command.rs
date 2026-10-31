// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Check whether an external command reports a version that meets a
//! requirement.
//!
//! Semantic versioning writes versions as MAJOR.MINOR.PATCH. A patch release
//! fixes bugs, a minor adds features compatibly, and a major may break things.
//! Version requirements such as `^1.2` say which releases a dependency accepts.

use std::ffi::OsStr;
use std::process::Command;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use semver::Version;
use semver::VersionReq;

/// Runs a command's `--version` option and parses its final output token.
///
/// Returns `Ok(None)` when the command is not installed.
fn command_version(command: &OsStr) -> Result<Option<Version>> {
    let output = match Command::new(command).arg("--version").output() {
        Ok(output) => output,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        }
        Err(error) => {
            return Err(error).with_context(|| {
                format!("failed to run {command:?} --version")
            });
        }
    };

    if !output.status.success() {
        bail!("{command:?} --version exited with {}", output.status);
    }

    let stdout = String::from_utf8(output.stdout)
        .context("version command output was not valid UTF-8")?;
    let version = stdout
        .split_whitespace()
        .last()
        .context("version command produced no version token")?;

    parse_version_token(version)
        .with_context(|| format!("could not parse command version {version:?}"))
        .map(Some)
}

/// Normalizes platform suffixes used by Git for Windows before SemVer parsing.
fn parse_version_token(version: &str) -> Result<Version> {
    let normalized = match version.split_once(".windows.") {
        Some((release, platform_version)) => {
            format!("{release}+windows.{platform_version}")
        }
        None => version.to_owned(),
    };

    Version::parse(&normalized)
        .context("version token was not valid semantic versioning")
}

/// Checks whether the installed Git version satisfies the example requirement.
fn main() -> Result<()> {
    let version_constraint = "> 1.12.0";
    let version_requirement = VersionReq::parse(version_constraint)?;
    let Some(version) = command_version(OsStr::new("git"))? else {
        println!("Git is not installed; install it to run this version check.");
        return Ok(());
    };

    if !version_requirement.matches(&version) {
        bail!(
            "Git version is lower than the minimum (found {version}, need {version_constraint})"
        );
    }
    println!("{version}");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a missing executable is reported without an error.
    #[test]
    fn missing_command_returns_none() -> anyhow::Result<()> {
        let command = std::env::temp_dir().join(format!(
            "rust-howto-missing-command-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_nanos()
        ));

        assert!(command_version(command.as_os_str())?.is_none());
        Ok(())
    }

    /// Verifies Git for Windows build suffixes do not affect version matching.
    #[test]
    fn windows_git_version_matches_release_requirement() -> anyhow::Result<()> {
        let version = parse_version_token("2.53.0.windows.1")?;
        let requirement = VersionReq::parse(">= 2.53.0")?;

        assert!(requirement.matches(&version));
        Ok(())
    }

    /// Verifies that the example exits successfully with or without Git.
    /// Runs the semver command example through its entry point.
    #[test]
    fn test_semver_command_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
