//! A placeholder crate published to make `rust_howto` searchable on crates.io.
//!
//! TODO Reviewed ??-??-2026.

use std::error::Error;
use std::fmt::Write as _;

use cargo_toml::Manifest;

/// Prints fields of the `Cargo.toml` manifest.
fn main() -> Result<(), Box<dyn Error>> {
    println!("This crate is a placeholder. It does nothing useful.\n");

    // Let's print parts of the Cargo.toml manifest.
    let cargo_toml_bytes = include_bytes!("../Cargo.toml");
    let manifest = Manifest::from_slice(cargo_toml_bytes)?;
    print!("{}", describe(&manifest)?);
    Ok(())
}

/// Renders the package fields of `manifest` as text.
///
/// If the manifest still inherits fields from the workspace (as it does before `cargo package`
/// rewrites it with full details), only the package name is reported.
///
/// # Errors
///
/// Returns an error if an inherited field cannot be resolved.
fn describe(manifest: &Manifest) -> Result<String, Box<dyn Error>> {
    let mut out = String::new();
    // Cargo.toml refers to the workspace ../Cargo.toml,
    // therefore some fields are not available until
    // `cargo package` rewrites it with full details.
    let some_fields_unavailable = manifest.needs_workspace_inheritance();

    if let Some(package) = &manifest.package {
        writeln!(out, "Name: {}\n", package.name)?;
        if some_fields_unavailable {
            return Ok(out);
        }
        writeln!(
            out,
            "Description: {}\n",
            package.description.clone().unwrap_or_default().get()?
        )?;
        writeln!(out, "Authors: {:?}\n", package.authors.get()?)?;
        writeln!(
            out,
            "Homepage: {}\n",
            package.homepage.clone().unwrap_or_default().get()?
        )?;
        writeln!(
            out,
            "Repository: {}\n",
            package.repository.clone().unwrap_or_default().get()?
        )?;
        writeln!(
            out,
            "Documentation: {}\n",
            package.documentation.clone().unwrap_or_default().get()?
        )?;
        writeln!(out, "Keywords: {:?}\n", package.keywords.get()?)?;
        writeln!(out, "Categories: {:?}\n", package.categories.get()?)?;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fully resolved manifest, as produced by `cargo package`.
    const RESOLVED: &str = r#"
[package]
name = "demo"
version = "0.1.0"
description = "A demo."
authors = ["Ann <ann@example.com>"]
homepage = "https://example.com"
repository = "https://example.com/repo"
documentation = "https://docs.example.com"
keywords = ["a", "b"]
categories = ["development-tools"]
"#;

    /// A manifest that inherits fields from the workspace.
    const INHERITED: &str = r#"
[package]
name = "demo"
version.workspace = true
description.workspace = true
authors.workspace = true
"#;

    #[test]
    fn describes_resolved_manifest() {
        let manifest = Manifest::from_slice(RESOLVED.as_bytes()).expect("manifest parses");
        let text = describe(&manifest).expect("fields resolve");
        assert!(text.contains("Name: demo"));
        assert!(text.contains("Description: A demo."));
        assert!(text.contains("Authors: [\"Ann <ann@example.com>\"]"));
        assert!(text.contains("Keywords: [\"a\", \"b\"]"));
        assert!(text.contains("Categories: [\"development-tools\"]"));
    }

    #[test]
    fn workspace_inheritance_reports_only_the_name() {
        let manifest = Manifest::from_slice(INHERITED.as_bytes()).expect("manifest parses");
        assert!(manifest.needs_workspace_inheritance());
        let text = describe(&manifest).expect("no field is resolved");
        assert_eq!(text, "Name: demo\n\n");
    }

    #[test]
    fn manifest_without_package_yields_empty_output() {
        let manifest = Manifest::from_slice(b"[workspace]\nmembers = []\n").expect("parses");
        assert_eq!(describe(&manifest).expect("no package"), "");
    }

    #[test]
    fn bundled_manifest_parses() {
        let manifest =
            Manifest::from_slice(include_bytes!("../Cargo.toml")).expect("bundled manifest");
        assert!(manifest.package.is_some());
        describe(&manifest).expect("manifest describes");
    }
}
