// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates local crate-index generation with checked fixture files.

/// Generate an alphabetical crate section with published recipe links.
mod alphabetical_page;
/// Generate a category page from local TOML metadata and recipe links.
mod category_page;
/// List dependencies declared in local Cargo manifests.
mod dependencies;
/// Shared local files for the crate-index examples.
mod fixtures;
/// Collect published recipe links from the local book index.
mod published_recipes;

/// Runs each local crate-index example in sequence.
fn main() -> anyhow::Result<()> {
    example_body()
}
/// Runs each offline capability from the shared book example runner.
///
/// # Errors
/// Returns an error if a step of the example fails.
pub fn example_body() -> anyhow::Result<()> {
    dependencies::run()?;
    published_recipes::run()?;
    category_page::run()?;
    alphabetical_page::run()
}
// ANCHOR_END: example

/// Runs the example through its program entry point.
///
/// # Errors
/// Returns an error if the example operation fails.
pub fn run() -> anyhow::Result<()> {
    example_body()
}

/// Verifies the crate-index examples run with their temporary fixtures.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the crate-index examples run with their temporary fixtures.
    #[test]
    fn test_crate_index_examples_run_offline() -> anyhow::Result<()> {
        run()?;
        assert!(main().is_ok());
        Ok(())
    }
}
