// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Embedding build details, such as the version, Git commit and build time, in
//! the binary helps with support: a bug report can state which build produced
//! it. Git metadata is included when the build runs in a Git checkout.

use std::collections::BTreeSet;

use build_info::CrateInfo;
use build_info::VersionControl;
use build_info::semver::Version;

build_info::build_info!(fn build_info);

/// Prints build metadata embedded by the build script.
fn main() -> anyhow::Result<()> {
    let info = build_info();
    let git_commit = info
        .version_control
        .as_ref()
        .and_then(VersionControl::git)
        .map(|git| git.commit_short_id.as_str())
        .unwrap_or("unavailable");
    let build_timestamp = info.timestamp.to_string();
    let compiler_version = info.compiler.version.to_string();

    anyhow::ensure!(!build_timestamp.is_empty(), "build timestamp is empty");
    anyhow::ensure!(!compiler_version.is_empty(), "compiler version is empty");
    println!("Built at: {build_timestamp}");
    println!("Git commit: {git_commit}");
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
fn collect_dependencies(crate_info: &CrateInfo, dependencies: &mut BTreeSet<(String, Version)>) {
    dependencies.insert((crate_info.name.clone(), crate_info.version.clone()));
    for dependency in &crate_info.dependencies {
        collect_dependencies(dependency, dependencies);
    }
}
// ANCHOR_END: example

/// Embedding build details, such as the version, Git commit and build time, in the binary helps
/// with support: a bug report can state exactly which build produced it.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Verifies build timestamp and compiler metadata are available.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the binary reports build timestamp and compiler metadata.
    #[test]
    fn build_info_has_timestamp_and_compiler() -> anyhow::Result<()> {
        main()
    }

    /// Checks that the dependencies include the crate itself and are sorted and unique.
    #[test]
    fn dependencies_include_the_crate_itself_and_are_sorted_and_unique() {
        let info = build_info();
        let deps = get_dependencies(&info.crate_info);
        assert!(
            deps.iter().any(|(name, v)| {
                *name == info.crate_info.name && *v == info.crate_info.version
            })
        );
        assert!(deps.len() > 1, "the crate has dependencies");
        let names: Vec<_> = deps.iter().collect();
        assert!(names.windows(2).all(|w| w[0] < w[1]));
    }

    /// Verifies that the collecting a single crate adds exactly one entry.
    #[test]
    fn collecting_a_single_crate_adds_exactly_one_entry() {
        let mut crate_info = build_info().crate_info.clone();
        crate_info.dependencies.clear();
        let deps = get_dependencies(&crate_info);
        assert_eq!(deps.len(), 1);
    }
}
