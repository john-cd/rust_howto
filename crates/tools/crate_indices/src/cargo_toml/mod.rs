//! Read Cargo.toml and extract dependencies

use anyhow::Result;
// use cargo_toml::Manifest;

/// Return a list of dependencies for the book's code examples (from
/// Cargo.toml)
pub fn get_dependencies() -> Result<Vec<String>> {
    // TODO P0 fix
    // let cargo_toml_bytes = include_bytes!("../../../../../crates/ex/.../Cargo.toml");
    // let manifest = Manifest::from_slice(cargo_toml_bytes)?;
    // // Cargo.toml refers to the workspace Cargo.toml,
    // // therefore some fields are not available until
    // // `cargo package` rewrites it with full details.
    // // let some_fields_unavailable = manifest.needs_workspace_inheritance();

    // let dependencies: Vec<String> = manifest
    //     .dependencies
    //     .keys()
    //     .map(|k| String::from(k.trim()))
    //     .collect();
    let dependencies: Vec<String> = vec![];
    Ok(dependencies)
}
