//! Read Cargo.toml files and extract dependencies

use std::path::Path;

use anyhow::Result;
use cargo_toml::Dependency;
use cargo_toml::DepsSet;
use cargo_toml::Manifest;
use tracing::debug;
use walkdir::WalkDir;

/// Return a sorted, deduplicated list of dependencies for the book's code examples.
///
/// Inspect all `Cargo.toml` files in the provided directory or in its children,
/// collect the dependencies, build dependencies, etc.
pub fn get_dependencies<P: AsRef<Path>>(root: P) -> Result<Vec<String>> {
    let mut dependencies: Vec<String> = Vec::new();

    let root = root.as_ref();
    debug!("Searching for Cargo.toml manifests in {}", root.display());
    for entry in WalkDir::new(root).min_depth(1).into_iter() {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml" {
            let path = entry.path();
            debug!("Reading {}", path.display());
            let manifest = Manifest::from_path(path)?;
            // Note: Cargo.toml refers to the workspace Cargo.toml,
            // therefore some fields are not available until
            // `cargo package` rewrites it with full details.
            // See  manifest.needs_workspace_inheritance();
            // In this case, we don't care.

            dependencies.extend(extract_crate_names(&manifest.dependencies));

            dependencies.extend(extract_crate_names(&manifest.build_dependencies));

            let targets = manifest.target.values();
            for target in targets {
                dependencies.extend(extract_crate_names(&target.dependencies));
            }
        }
    }
    dependencies.sort();
    dependencies.dedup();
    Ok(dependencies)
}

/// Extract the crate names from the Dependency Set.
fn extract_crate_names(depset: &DepsSet) -> Vec<String> {
    depset
        .iter()
        .map(|(k, v)| {
            match v {
                // crate_name = "1.2.3"
                Dependency::Simple(_) => k,
                // crate_name.workspace = true
                Dependency::Inherited(_) => k,
                // alias = { package = "crate_name", ... }
                Dependency::Detailed(detail) => {
                    if let Some(ref pkg) = detail.package {
                        pkg
                    } else {
                        k
                    }
                }
            }
        })
        .map(|k| String::from(k.trim()))
        .collect()
}
