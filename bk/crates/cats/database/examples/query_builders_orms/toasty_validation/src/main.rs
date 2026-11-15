//! \ Runs\ the\ Toasty\ query\ validation\ example\.
//!
//! TODO Reviewed ??-??-2026.

#[path = "../../toasty.rs"]
mod toasty;
/// Runs the example.
fn main() -> anyhow::Result<()> {
    toasty::run()
}
