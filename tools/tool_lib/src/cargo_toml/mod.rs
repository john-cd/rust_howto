//! Read Cargo.toml files and extract dependencies

use anyhow::Result;
use cargo_toml::Manifest;
use std::path::Path;
use walkdir::WalkDir;
use tracing::debug;

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
            // Cargo.toml refers to the workspace Cargo.toml,
            // therefore some fields are not available until
            // `cargo package` rewrites it with full details.
            // let some_fields_unavailable =
            // manifest.needs_workspace_inheritance();

            dependencies.extend(manifest.dependencies.keys().map(|k| String::from(k.trim())));

            dependencies.extend(
                manifest
                    .build_dependencies
                    .keys()
                    .map(|k| String::from(k.trim())),
            );
            let targets = manifest.target.values();
            for target in targets {
                dependencies.extend(target.dependencies.keys().map(|k| String::from(k.trim())));
            }
        }
    }
    dependencies.sort();
    dependencies.dedup();
    Ok(dependencies)
}
