// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Checks that every workspace member opts in to the shared
//! `[workspace.lints]` table, and can fix a manifest with `toml_edit`.
//!
//! A Cargo workspace groups several crates. Shared `[workspace.lints]` set
//! Clippy and compiler lint levels once for every member, so rules stay
//! consistent. This example checks that every crate opts in.

use std::path::Path;

use cargo_metadata::MetadataCommand;
use toml_edit::DocumentMut;
use toml_edit::value;

/// Returns `true` if the manifest contains `[lints] workspace = true`.
fn has_workspace_lints(manifest: &str) -> anyhow::Result<bool> {
    let doc: DocumentMut = manifest.parse()?;
    Ok(doc
        .get("lints")
        .and_then(|lints| lints.get("workspace"))
        .and_then(toml_edit::Item::as_bool)
        .unwrap_or(false))
}

/// Adds `[lints] workspace = true`, keeping the rest of the file intact.
fn add_workspace_lints(manifest: &str) -> anyhow::Result<String> {
    let mut doc: DocumentMut = manifest.parse()?;
    doc["lints"]["workspace"] = value(true);
    Ok(doc.to_string())
}

/// Checks every workspace package and fails with its missing opt-ins.
fn main() -> anyhow::Result<()> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let metadata = MetadataCommand::new()
        .manifest_path(manifest)
        .no_deps()
        .exec()?;

    // Collect every member that does not opt in; a real `xtask` would then
    // exit with a non-zero status.
    let mut missing = Vec::new();
    for package in metadata.workspace_packages() {
        let text = std::fs::read_to_string(&package.manifest_path)?;
        if !has_workspace_lints(&text)? {
            // Show the manifest as it would look once fixed.
            println!("{}", add_workspace_lints(&text)?);
            missing.push(package.name.to_string());
        }
    }
    println!("members without `[lints] workspace = true`: {missing:?}");
    if !missing.is_empty() {
        anyhow::bail!(
            "workspace packages missing `[lints] workspace = true`: {missing:?}"
        );
    }
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

    /// Confirms the current example crate opts into the workspace lint table.
    /// Runs the workspace lints example through its entry point.
    #[test]
    fn test_workspace_lints_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    /// Checks that a candidate fix preserves comments and enables workspace
    /// lints.
    #[test]
    fn detects_and_fixes_a_missing_table() -> anyhow::Result<()> {
        let before = "[package]\nname = \"demo\" # keep this comment\n";
        assert!(!has_workspace_lints(before)?);

        let after = add_workspace_lints(before)?;
        assert!(has_workspace_lints(&after)?);
        assert!(after.contains("# keep this comment"));
        Ok(())
    }
}
