//! Runs the artificial intelligence examples.
//!
//! TODO Reviewed ??-??-2026.

mod astar;
mod bonsai_bt;

/// Runs the artificial intelligence examples.
fn main() -> anyhow::Result<()> {
    astar::run()?;
    bonsai_bt::run()
}
