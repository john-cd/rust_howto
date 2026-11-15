// TODO Reviewed ??-??-2026.

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
    dependencies::run()?;
    published_recipes::run()?;
    category_page::run()?;
    alphabetical_page::run()
}
