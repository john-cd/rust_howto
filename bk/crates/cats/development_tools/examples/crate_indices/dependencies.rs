// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! List dependencies declared in local Cargo manifests.

use anyhow::Result;

use crate::fixtures::create_fixture;

/// Prints the sorted dependency names found in the local fixture workspace.
pub fn run() -> Result<()> {
    let fixture = create_fixture()?;
    let workspace = fixture.path().join("workspace");
    let dependencies = tool_lib::get_dependencies(workspace)?;
    println!("Workspace dependencies: {dependencies:#?}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that dependency discovery reads and sorts local manifests.
    #[test]
    fn dependency_discovery_uses_local_manifests() -> Result<()> {
        let fixture = create_fixture()?;
        let dependencies =
            tool_lib::get_dependencies(fixture.path().join("workspace"))?;
        assert_eq!(
            dependencies,
            vec!["serde".to_owned(), "serde_json".to_owned()]
        );
        Ok(())
    }
}
