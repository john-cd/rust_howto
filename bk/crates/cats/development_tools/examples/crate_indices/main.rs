#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates local crate-index generation with checked fixture files.

mod alphabetical_page;
mod category_page;
mod dependencies;
mod fixtures;
mod published_recipes;

/// Runs each local crate-index example in sequence.
fn main() -> anyhow::Result<()> {
    run()
}

/// Runs each offline capability from the shared book example runner.
pub fn run() -> anyhow::Result<()> {
    dependencies::run()?;
    published_recipes::run()?;
    category_page::run()?;
    alphabetical_page::run()
}
// ANCHOR_END: example
