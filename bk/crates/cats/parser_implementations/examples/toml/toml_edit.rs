#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `toml_edit` crate to
//! parse, modify, and print TOML documents while preserving the
//! original formatting (comments, spaces and relative order or items).
//!
//! Use `toml_edit` instead of `toml` for format-preserving editing or finer
//! control over output.
use toml_edit::DocumentMut;
use toml_edit::Item;
use toml_edit::value;

fn main() -> anyhow::Result<()> {
    // Parse existing TOML, here a `Cargo.toml` file:
    let toml_content = r#"
[package]
name = "my-package"
version = "0.1.0"
authors = ["Me <me@example.com>"]

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }

[[bin]]
name = "app"
path = "src/main.rs"
"#;

    // Parse the document.
    let mut doc: DocumentMut = toml_content.parse::<DocumentMut>()?;

    // Modify values.
    doc["package"]["version"] = value("0.2.0");

    // Add a new dependency.
    doc["dependencies"]["toml_edit"] = value("0.22.24");

    // Add a feature to an existing dependency.
    // `Item` is an enum representing either a value, a table, an array of
    // tables, or none.
    if let Some(Item::Value(tokio)) = doc
        .get_mut("dependencies")
        .and_then(|deps| deps.get_mut("tokio"))
    {
        if let Some(t) = tokio.as_inline_table_mut() {
            let features = t
                .get_mut("features")
                .and_then(|f| f.as_array_mut())
                .expect("`tokio` features should be an array");

            features.push("time");
        }
    }

    // Add a new table.
    let mut dev_deps = toml_edit::table(); // Empty table.
    dev_deps
        .as_table_mut()
        .unwrap()
        .insert("pretty_assertions", value("1.4.1"));
    doc.insert("dev-dependencies", dev_deps);

    // Add to the array of tables.
    if let Some(Item::ArrayOfTables(bins)) = doc.get_mut("bin") {
        let mut new_bin = toml_edit::Table::new();
        new_bin.insert("name", value("cli"));
        new_bin.insert("path", value("src/cli.rs"));
        bins.push(new_bin);
    }

    // Print the modified document - calls `to_string()`.
    println!("{}", doc);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
