//! Read Cargo.toml and extract dependencies

use anyhow::Result;
use cargo_toml::Manifest;
use walkdir::WalkDir;

/// Return a list of dependencies for the book's code examples
/// (from Cargo.toml files)
pub fn get_dependencies() -> Result<Vec<String>> {
    let mut dependencies: Vec<String> = Vec::new();

    for entry in WalkDir::new("/code/crates/ex").min_depth(1).into_iter() {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml" {
            let path = entry.path();
            // println!("> Reading {}", path.display());
            let manifest = Manifest::from_path(path)?;
            // Cargo.toml refers to the workspace Cargo.toml,
            // therefore some fields are not available until
            // `cargo package` rewrites it with full details.
            // let some_fields_unavailable =
            // manifest.needs_workspace_inheritance();

            dependencies.extend(
                manifest.dependencies.keys().map(|k| String::from(k.trim())),
            );

            dependencies.extend(
                manifest
                    .build_dependencies
                    .keys()
                    .map(|k| String::from(k.trim())),
            );
            let targets = manifest.target.values();
            for target in targets {
                dependencies.extend(
                    target.dependencies.keys().map(|k| String::from(k.trim())),
                );
            }
        }
    }
    dependencies.sort();
    dependencies.dedup();
    Ok(dependencies)
}
