//! Runs the selected Apache Arrow data processing recipe.
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;
use clap::Parser;

mod arrow;

/// Command-line selector for the Arrow recipes.
#[derive(Parser)]
struct Cli {
    /// Runs all Arrow recipes when no subcommand is supplied.
    #[command(subcommand)]
    recipe: Option<arrow::Recipe>,
}

/// Dispatches the selected recipe and reports any failure.
fn main() -> Result<()> {
    arrow::run(Cli::parse().recipe.unwrap_or(arrow::Recipe::All))
}
