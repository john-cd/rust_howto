//! Runs the `pest` parser example.
//!
//! TODO Reviewed ??-??-2026.

/// This example demonstrates `pest` parsing a flat arithmetic expression
mod pest;

/// Runs the pest example.
fn main() -> anyhow::Result<()> {
    pest::run()
}
