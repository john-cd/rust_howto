//! A placeholder crate published to make `rust_howto` searchable on crates.io.
//!
//! TODO Reviewed ??-??-2026.

use std::error::Error;

use cargo_toml::Manifest;

/// Prints fields of the `Cargo.toml` manifest.
fn main() -> Result<(), Box<dyn Error>> {
    println!("This crate is a placeholder. It does nothing useful.\n");

    // Let's print parts of the Cargo.toml manifest.

    let cargo_toml_bytes = include_bytes!("../Cargo.toml");
    let manifest = Manifest::from_slice(cargo_toml_bytes)?;
    // Cargo.toml refers to the workspace ../Cargo.toml,
    // therefore some fields are not available until
    // `cargo package` rewrites it with full details.
    let some_fields_unavailable = manifest.needs_workspace_inheritance();

    if let Some(package) = manifest.package {
        println!("Name: {}\n", package.name);
        if some_fields_unavailable {
            return Ok(());
        }
        println!(
            "Description: {}\n",
            package.description.unwrap_or_default().get()?
        );
        println!("Authors: {:?}\n", package.authors.get()?);
        println!(
            "Homepage: {}\n",
            package.homepage.unwrap_or_default().get()?
        );
        println!(
            "Repository: {}\n",
            package.repository.unwrap_or_default().get()?
        );
        println!(
            "Documentation: {}\n",
            package.documentation.unwrap_or_default().get()?
        );
        println!("Keywords: {:?}\n", package.keywords.get()?);
        println!("Categories: {:?}\n", package.categories.get()?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use cargo_toml::Edition;
    use cargo_toml::Inheritable;
    use cargo_toml::Manifest;
    use cargo_toml::Package;
    use cargo_toml::Publish;
    use cargo_toml::SemVer;
    use cargo_toml::Value;

    use super::*;

    /// Verifies core identification metadata in embedded `Cargo.toml`.
    #[test]
    fn test_embedded_cargo_toml_core_metadata() {
        let bytes = include_bytes!("../Cargo.toml");
        let manifest = Manifest::from_slice(bytes).expect("embedded Cargo.toml should parse");

        let package = manifest
            .package
            .as_ref()
            .expect("embedded Cargo.toml should contain a package section");

        assert_eq!(package.name, "rust_howto");
        assert_eq!(
            package
                .version
                .get()
                .expect("version should be defined directly"),
            &SemVer::new(0, 1, 2)
        );
        assert_eq!(
            package
                .edition
                .get()
                .expect("edition should be defined directly"),
            &Edition::E2024
        );
        assert_eq!(
            package.description.as_ref().map(|description| {
                description
                    .get()
                    .expect("description should be defined directly")
                    .as_str()
            }),
            Some(
                "A documentation summary and a compendium of snippets and recipes for the Rust language and ecosystem. Everything you need for day-to-day Rust coding, all in one place."
            )
        );
        assert_eq!(
            package
                .authors
                .get()
                .expect("authors should be defined directly"),
            &["John CD".to_owned()]
        );
    }

    /// Verifies links, categories, and license in embedded `Cargo.toml`.
    #[test]
    fn test_embedded_cargo_toml_links_and_classification() {
        let bytes = include_bytes!("../Cargo.toml");
        let manifest = Manifest::from_slice(bytes).expect("embedded Cargo.toml should parse");

        let package = manifest
            .package
            .as_ref()
            .expect("embedded Cargo.toml should contain a package section");

        assert_eq!(
            package.license.as_ref().map(|license| {
                license
                    .get()
                    .expect("license should be defined directly")
                    .as_str()
            }),
            Some("MIT OR Apache-2.0")
        );
        assert_eq!(
            package.documentation.as_ref().map(|documentation| {
                documentation
                    .get()
                    .expect("documentation should be defined directly")
                    .as_str()
            }),
            Some("https://docs.rs/rust_howto/")
        );
        assert_eq!(
            package.homepage.as_ref().map(|homepage| {
                homepage
                    .get()
                    .expect("homepage should be defined directly")
                    .as_str()
            }),
            Some("https://john-cd.com/rust_howto/")
        );
        assert_eq!(
            package.repository.as_ref().map(|repository| {
                repository
                    .get()
                    .expect("repository should be defined directly")
                    .as_str()
            }),
            Some("https://github.com/john-cd/rust_howto/")
        );
        assert!(
            package
                .keywords
                .get()
                .expect("keywords should be defined directly")
                .contains(&"rust".to_owned())
        );
        assert!(
            package
                .categories
                .get()
                .expect("categories should be defined directly")
                .contains(&"development-tools".to_owned())
        );
        assert!(!manifest.needs_workspace_inheritance());
    }

    /// Verifies that malformed TOML content yields a parsing error.
    #[test]
    fn test_malformed_cargo_toml_parsing_fails() {
        let invalid_toml = b"this is not valid toml = [";
        let result = Manifest::from_slice(invalid_toml);
        let _err = result.expect_err("malformed TOML must return an error");
    }

    /// Verifies that `needs_workspace_inheritance` detects inherited fields.
    #[test]
    fn test_workspace_inheritance_detection() {
        let toml_with_inheritance = br#"
[package]
name = "inherited_crate"
version.workspace = true
edition = "2024"
authors.workspace = true
description.workspace = true
license.workspace = true
publish = true
readme = "README.md"
"#;
        let manifest =
            Manifest::from_slice(toml_with_inheritance).expect("valid TOML syntax should parse");
        assert!(manifest.needs_workspace_inheritance());

        let package = manifest
            .package
            .as_ref()
            .expect("parsed manifest should contain a package section");

        assert_eq!(package.name, "inherited_crate");
        let _version_err = package
            .version
            .get()
            .expect_err("inherited version must return error before workspace completion");
        let _authors_err = package
            .authors
            .get()
            .expect_err("inherited authors must return error before workspace completion");
        assert_eq!(
            package
                .edition
                .get()
                .expect("concrete edition must be accessible"),
            &Edition::E2024
        );
    }

    /// Verifies fallback behavior when workspace inheritance is active.
    #[test]
    fn test_workspace_inheritance_fallback_behavior() {
        let toml_with_inheritance = br#"
[package]
name = "inherited_crate"
version.workspace = true
edition = "2024"
authors.workspace = true
description.workspace = true
license.workspace = true
publish = true
readme = "README.md"
"#;
        let manifest =
            Manifest::from_slice(toml_with_inheritance).expect("valid TOML syntax should parse");
        let some_fields_unavailable = manifest.needs_workspace_inheritance();
        assert!(some_fields_unavailable);

        let package = manifest
            .package
            .as_ref()
            .expect("manifest should have package defined");

        assert_eq!(package.name, "inherited_crate");

        // Emulate the fallback early-return branch used in main().
        let executed_early_return = if some_fields_unavailable {
            true
        } else {
            let _desc = package
                .description
                .as_ref()
                .and_then(|description| description.get().ok());
            false
        };

        assert!(executed_early_return);
    }

    /// Verifies serialization of a manifest with concrete and inherited properties.
    #[test]
    fn test_manifest_serialization() {
        let mut package: Package<Value> = Package::new("roundtrip_demo", SemVer::new(1, 0, 0));
        package.description = Some(Inheritable::Set("Roundtrip test package.".to_owned()));
        package.edition = Inheritable::Set(Edition::E2024);
        package.publish = Inheritable::Set(Publish::Flag(true));

        #[expect(
            deprecated,
            reason = "Manifest::replace is a deprecated Cargo field in cargo_toml"
        )]
        let manifest: Manifest<Value> = Manifest {
            package: Some(package),
            workspace: None,
            dependencies: cargo_toml::DepsSet::new(),
            dev_dependencies: cargo_toml::DepsSet::new(),
            build_dependencies: cargo_toml::DepsSet::new(),
            target: cargo_toml::TargetDepsSet::new(),
            features: cargo_toml::FeatureSet::new(),
            replace: cargo_toml::DepsSet::new(),
            patch: cargo_toml::PatchSet::new(),
            lib: None,
            profile: cargo_toml::Profiles::default(),
            badges: cargo_toml::Badges::default(),
            bin: Vec::new(),
            bench: Vec::new(),
            test: Vec::new(),
            example: Vec::new(),
            lints: Inheritable::Inherited,
        };

        let serialized_toml =
            toml::to_string(&manifest).expect("manifest should serialize to TOML");
        assert!(serialized_toml.contains("name = \"roundtrip_demo\""));
        assert!(serialized_toml.contains("version = \"1.0.0\""));
        assert!(serialized_toml.contains("edition = \"2024\""));
        assert!(serialized_toml.contains("workspace = true"));

        let deserialized: Manifest = Manifest::from_slice(serialized_toml.as_bytes())
            .expect("serialized manifest should parse back into Manifest");
        let parsed_package = deserialized
            .package
            .as_ref()
            .expect("deserialized manifest should contain a package");
        assert_eq!(parsed_package.name, "roundtrip_demo");
        assert_eq!(
            parsed_package
                .version
                .get()
                .expect("version should be defined"),
            &SemVer::new(1, 0, 0)
        );
        assert_eq!(
            parsed_package.description.as_ref().map(|description| {
                description
                    .get()
                    .expect("description should be defined")
                    .as_str()
            }),
            Some("Roundtrip test package.")
        );
        assert!(deserialized.needs_workspace_inheritance());
    }

    /// Verifies that `main()` runs successfully without error.
    #[test]
    fn test_main_execution() {
        let entry = main;
        entry().expect("main should execute successfully");
    }
}
