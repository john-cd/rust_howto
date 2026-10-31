// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Lists the packages and targets of a Cargo workspace with `cargo_metadata`.
//!
//! `cargo metadata` prints, as JSON, what Cargo knows about a project: its
//! packages, dependencies, features and targets. Build tools and IDEs use it
//! instead of parsing `Cargo.toml` themselves.

use std::path::Path;

use cargo_metadata::MetadataCommand;

fn main() -> anyhow::Result<()> {
    // Point at this crate's own manifest. `no_deps` skips resolving
    // dependencies, which is faster and works offline.
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let metadata = MetadataCommand::new()
        .manifest_path(manifest)
        .no_deps()
        .exec()?;

    for package in metadata.workspace_packages() {
        println!("{} {}", package.name, package.version);
        for target in &package.targets {
            println!("  {:?}: {}", target.kind, target.name);
        }
    }

    assert!(
        metadata
            .workspace_packages()
            .iter()
            .any(|p| p.name.as_str() == "development_tools")
    );
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the cargo metadata example through its entry point.
    #[test]
    fn test_cargo_metadata_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
