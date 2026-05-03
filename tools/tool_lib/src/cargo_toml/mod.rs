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
    for entry in WalkDir::new(root).into_iter() {
        let entry = entry?;
        if entry.file_name() == "Cargo.toml" {
            let path = entry.path();
            debug!("Reading {}", path.display());
            let manifest = Manifest::from_path(path)?;
            // Note: `Cargo.toml` may refer to the workspace `Cargo.toml`,
            // therefore some fields may not be available until
            // `cargo package` rewrites it with full details.
            // See: manifest.needs_workspace_inheritance();
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use cargo_toml::Dependency;
    use cargo_toml::DependencyDetail;
    use cargo_toml::DepsSet;
    use cargo_toml::InheritedDependencyDetail;
    use tempfile::tempdir;

    use super::*;

    // --- Tests for `extract_crate_names` ---

    #[test]
    fn test_extract_empty() {
        let deps = DepsSet::new();
        let names = extract_crate_names(&deps);
        assert!(names.is_empty());
    }

    #[test]
    fn test_extract_simple() {
        let mut deps = DepsSet::new();
        deps.insert(
            "simple_dep".to_string(),
            Dependency::Simple("1.0".to_string()),
        );
        deps.insert(
            "another_dep".to_string(),
            Dependency::Simple("*".to_string()),
        );

        let mut names = extract_crate_names(&deps);
        names.sort(); // Sort for consistent assertion
        assert_eq!(names, vec!["another_dep", "simple_dep"]);
    }

    #[test]
    fn test_extract_inherited() {
        let mut deps = DepsSet::new();
        deps.insert(
            "inherited_dep".to_string(),
            Dependency::Inherited(InheritedDependencyDetail {
                workspace: true,
                ..Default::default()
            }),
        );

        let names = extract_crate_names(&deps);
        assert_eq!(names, vec!["inherited_dep"]);
    }

    #[test]
    fn test_extract_detailed_no_package() {
        let mut deps = DepsSet::new();
        deps.insert(
            "detailed_dep".to_string(),
            Dependency::Detailed(Box::new(DependencyDetail {
                version: Some("1.0".to_string()),
                package: None, // No explicit package name.
                ..Default::default()
            })),
        );

        let names = extract_crate_names(&deps);
        assert_eq!(names, vec!["detailed_dep"]);
    }

    #[test]
    fn test_extract_detailed_with_package() {
        let mut deps = DepsSet::new();
        deps.insert(
            "alias_dep".to_string(), // This is the alias.
            Dependency::Detailed(Box::new(DependencyDetail {
                version: Some("1.0".to_string()),
                package: Some("real_crate_name".to_string()), // Explicit package name.
                ..Default::default()
            })),
        );

        let names = extract_crate_names(&deps);
        // Should extract the package name.
        assert_eq!(names, vec!["real_crate_name"]);
    }

    #[test]
    fn test_extract_mixed() {
        let mut deps = DepsSet::new();
        deps.insert("simple".to_string(), Dependency::Simple("1.0".to_string()));
        deps.insert(
            "inherited".to_string(),
            Dependency::Inherited(InheritedDependencyDetail {
                workspace: true,
                ..Default::default()
            }),
        );
        deps.insert(
            "detailed".to_string(),
            Dependency::Detailed(Box::new(DependencyDetail {
                version: Some("1.0".to_string()),
                ..Default::default()
            })),
        );
        deps.insert(
            "aliased".to_string(),
            Dependency::Detailed(Box::new(DependencyDetail {
                package: Some("real_name".to_string()),
                ..Default::default()
            })),
        );

        let mut names = extract_crate_names(&deps);
        names.sort();
        assert_eq!(names, vec!["detailed", "inherited", "real_name", "simple"]);
    }

    // --- Tests for `get_dependencies` ---

    // Helper to create a `Cargo.toml` file.
    fn create_cargo_toml(dir: &Path, filename: &str, content: &str) -> Result<()> {
        fs::write(dir.join(filename), content)?;
        Ok(())
    }

    #[test]
    fn test_get_deps_empty_dir() -> Result<()> {
        let dir = tempdir()?;
        let deps = get_dependencies(dir.path())?;
        assert!(deps.is_empty());
        Ok(())
    }

    #[test]
    fn test_get_deps_no_cargo_toml() -> Result<()> {
        let dir = tempdir()?;
        fs::write(dir.path().join("some_file.txt"), "hello")?;
        fs::create_dir(dir.path().join("subdir"))?;
        fs::write(dir.path().join("subdir/another.rs"), "fn main() {}")?;

        let deps = get_dependencies(dir.path())?;
        assert!(deps.is_empty());
        Ok(())
    }

    #[test]
    fn test_get_deps_root_cargo_toml() -> Result<()> {
        let dir = tempdir()?;
        create_cargo_toml(
            dir.path(),
            "Cargo.toml",
            r#"
            [workspace]

            [package]
            name = "hello_world"
            version = "0.1.0"

            [dependencies]
            root_dep = "1.0"
        "#,
        )?;
        fs::create_dir(dir.path().join("subdir"))?;
        create_cargo_toml(
            &dir.path().join("subdir"),
            "Cargo.toml",
            r#"
            [dependencies]
            sub_dep = "2.0"
        "#,
        )?;

        let deps = get_dependencies(dir.path())?;
        assert_eq!(deps, vec!["root_dep", "sub_dep"]);
        Ok(())
    }

    #[test]
    fn test_get_deps_single_file_simple() -> Result<()> {
        let dir = tempdir()?;
        let proj_dir = dir.path().join("project1");
        fs::create_dir(&proj_dir)?;
        create_cargo_toml(
            &proj_dir,
            "Cargo.toml",
            r#"
            [dependencies]
            dep_a = "1.0"
            dep_b = "2.0"
        "#,
        )?;

        let deps = get_dependencies(dir.path())?;
        assert_eq!(deps, vec!["dep_a", "dep_b"]);
        Ok(())
    }

    #[test]
    fn test_get_deps_single_file_mixed_types() -> Result<()> {
        let dir = tempdir()?;
        let proj_dir = dir.path().join("project_mix");
        fs::create_dir(&proj_dir)?;
        create_cargo_toml(
            &proj_dir,
            "Cargo.toml",
            r#"
            [dependencies]
            simple_dep = "1.0"
            detailed_dep = { version = "0.5" }
            aliased_dep = { package = "real_crate", version = "1.1" }

            [build-dependencies]
            build_dep = "0.3"

            [target.'cfg(unix)'.dependencies]
            unix_dep = "0.9"
        "#,
        )?;

        let deps = get_dependencies(dir.path())?;
        assert_eq!(
            deps,
            vec![
                "build_dep",
                "detailed_dep",
                "real_crate", // Note: aliased_dep becomes real_crate.
                "simple_dep",
                "unix_dep"
            ]
        );
        Ok(())
    }

    #[test]
    fn test_get_deps_multiple_files_dedup_sort() -> Result<()> {
        let dir = tempdir()?;

        // Project 1
        let proj1_dir = dir.path().join("project1");
        fs::create_dir(&proj1_dir)?;
        create_cargo_toml(
            &proj1_dir,
            "Cargo.toml",
            r#"
            [dependencies]
            common_dep = "1.0"
            proj1_dep = "0.1"
            another_common = { version = "2.0" }
        "#,
        )?;

        // Project 2 (nested)
        let proj2_dir = dir.path().join("project1/project2");
        fs::create_dir(&proj2_dir)?;
        create_cargo_toml(
            &proj2_dir,
            "Cargo.toml",
            r#"
            [dependencies]
            common_dep = "1.1" # Different version, but same name
            proj2_dep = "0.2"
            another_common = { package = "another_common_real", version = "2.1" } # Aliased differently
        "#,
        )?;

        // Project 3
        let proj3_dir = dir.path().join("project3");
        fs::create_dir(&proj3_dir)?;
        create_cargo_toml(
            &proj3_dir,
            "Cargo.toml",
            r#"
            [build-dependencies]
            build_tool = "3.0"
            common_dep = "1.2" # Yet another version
        "#,
        )?;

        let deps = get_dependencies(dir.path())?;
        assert_eq!(
            deps,
            vec![
                "another_common",      // From project1
                "another_common_real", // From project2 (package name)
                "build_tool",
                "common_dep", // Deduplicated
                "proj1_dep",
                "proj2_dep"
            ]
        );
        Ok(())
    }

    #[test]
    fn test_get_deps_invalid_toml_propagates_error() -> Result<()> {
        let dir = tempdir()?;
        let proj_dir = dir.path().join("bad_project");
        fs::create_dir(&proj_dir)?;
        // Intentionally invalid TOML.
        create_cargo_toml(
            &proj_dir,
            "Cargo.toml",
            r#"
            [dependencies]
            dep_a = "1.0
            dep_b = "2.0" # Missing closing quote on previous line
        "#,
        )?;

        let result = get_dependencies(dir.path());
        assert!(result.is_err());
        // assert!(result.unwrap_err().to_string().contains("..."));
        Ok(())
    }
}
