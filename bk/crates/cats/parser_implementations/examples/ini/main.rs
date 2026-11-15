//! Runs the INI parser example.
//!
//! TODO Reviewed ??-??-2026.

/// Turns ini input into structured syntax data while preserving source locations and reporting
/// malformed input. Analysis and validation code use these routines before applying domain-specific
/// rules.
mod configparser_ini;
/// This example demonstrates how to use `rust-ini` to parse and manipulate INI
mod rust_ini;

/// Runs the ini example.
fn main() -> anyhow::Result<()> {
    configparser_ini::run()?;
    rust_ini::run()
}
