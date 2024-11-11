//! Read Cargo.toml and extract dependencies

use anyhow::Result;
use cargo_toml::Manifest;

/// Return a list of dependencies for the book's code examples (from
/// deps/Cargo.toml)
pub fn get_dependencies() -> Result<Vec<String>> {
    let cargo_toml_bytes = include_bytes!("../../../../deps/Cargo.toml");
    let manifest = Manifest::from_slice(cargo_toml_bytes)?;
    // Cargo.toml refers to the workspace Cargo.toml,
    // therefore some fields are not available until
    // `cargo package` rewrites it with full details.
    // let some_fields_unavailable = manifest.needs_workspace_inheritance();

    let deps: Vec<String> = manifest
        .dependencies
        .keys()
        .map(|k| String::from(k.trim()))
        .collect();
    Ok(deps)
}
