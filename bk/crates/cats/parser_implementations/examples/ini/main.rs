//! Runs the INI parser example.
//!
//! TODO Reviewed ??-??-2026

mod rust_ini;

/// Runs the example from the `rust_ini` module.
fn main() -> anyhow::Result<()> {
    rust_ini::run()
}
