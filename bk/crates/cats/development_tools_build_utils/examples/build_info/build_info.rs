//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use std::collections::BTreeSet;

use build_info::CrateInfo;
use build_info::VersionControl;
use build_info::semver::Version;

build_info::build_info!(fn build_info);

/// Prints build metadata embedded by the build script.
fn main() -> anyhow::Result<()> {
    let info = build_info();
    let git_info = info
        .version_control
        .as_ref()
        .and_then(VersionControl::git)
        .ok_or_else(|| anyhow::anyhow!("Git metadata is unavailable"))?;
    let build_timestamp = info.timestamp.to_string();
    let compiler_version = info.compiler.version.to_string();

    anyhow::ensure!(!build_timestamp.is_empty(), "build timestamp is empty");
    anyhow::ensure!(!compiler_version.is_empty(), "compiler version is empty");
    anyhow::ensure!(
        !git_info.commit_short_id.is_empty(),
        "Git commit hash is empty"
    );

    println!("Built at: {build_timestamp}");
    println!("Git commit: {}", git_info.commit_short_id);
    println!("Compiler: {compiler_version}");
    println!("Dependencies:");
    for (name, version) in get_dependencies(&info.crate_info) {
        println!("  {name} {version}");
    }

    Ok(())
}

/// Returns the crate and its collected dependencies without duplicate versions.
fn get_dependencies(crate_info: &CrateInfo) -> BTreeSet<(String, Version)> {
    let mut dependencies = BTreeSet::new();
    collect_dependencies(crate_info, &mut dependencies);
    dependencies
}

/// Adds a crate and its collected dependencies to the ordered set.
fn collect_dependencies(
    crate_info: &CrateInfo,
    dependencies: &mut BTreeSet<(String, Version)>,
) {
    dependencies.insert((crate_info.name.clone(), crate_info.version.clone()));
    for dependency in &crate_info.dependencies {
        collect_dependencies(dependency, dependencies);
    }
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the binary has Git, build timestamp, and compiler metadata.
    #[test]
    fn build_info_contains_git_time_and_compiler() -> anyhow::Result<()> {
        main()
    }
}
