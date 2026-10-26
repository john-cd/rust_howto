#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to deserialize TOML data into Rust structs.

use std::collections::HashMap;

use serde::Deserialize;
use toml::de::Error;

/// The configuration.
#[derive(Deserialize, Debug)]
struct Config {
    /// The package settings.
    package: Package,
    /// The dependencies, as a map from names to versions.
    dependencies: HashMap<String, String>,
}

/// The settings of a package.
#[derive(Deserialize, Debug)]
struct Package {
    /// The name of the package.
    name: String,
    /// The version of the package.
    version: String,
    /// The authors of the package.
    authors: Vec<String>,
}

/// Deserializes TOML into a struct and checks its content.
fn main() -> Result<(), Error> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

    let package_info: Config = toml::from_str(toml_content)?;
    println!("{package_info:?}");

    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.dependencies["serde"], "1.0");

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
