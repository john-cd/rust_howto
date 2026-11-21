//! Command-line entry point for the recipe table updater.
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context as _;
use anyhow::Result;
use clap::Parser;

/// Updates recipe tables under an mdBook root.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// mdBook root containing `src/` and optionally `drafts/`.
    book_root: PathBuf,
}

/// Parses options, updates recipe tables, and prints changed fragments.
fn run() -> Result<()> {
    let args = Args::parse();
    if !args.book_root.is_dir() {
        anyhow::bail!("book root is not a directory: {}", args.book_root.display());
    }
    let root = args
        .book_root
        .canonicalize()
        .with_context(|| format!("could not resolve book root {}", args.book_root.display()))?;
    let changed = recipe_tables::update_book(&root)?;
    for path in &changed {
        println!("Updated {}", path.display());
    }
    println!("Updated {} recipe table fragment(s).", changed.len());
    Ok(())
}

/// Starts the command and reports operational failures.
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("fix_recipe_tables: {error:#}");
            ExitCode::FAILURE
        }
    }
}
