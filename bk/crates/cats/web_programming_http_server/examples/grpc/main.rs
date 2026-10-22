//! Runs the gRPC server example.
//!
//! TODO Reviewed ??-??-2026

mod tonic;
mod tonic_client;

/// Runs the example from the `tonic` module.
fn main() -> anyhow::Result<()> {
    tonic::run()?;
    Ok(())
}
