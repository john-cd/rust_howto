//! \ Runs\ the\ Toasty\ query\ validation\ example\.
//!
//! TODO Reviewed ??-??-2026.

/// Reuses the database model and query-validation demonstration from the book.
#[path = "../../toasty.rs"]
mod toasty;

/// Runs the example.
fn main() -> anyhow::Result<()> {
    toasty::run()
}
