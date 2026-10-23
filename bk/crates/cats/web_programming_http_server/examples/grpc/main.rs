//! Runs the gRPC server example.
//!
//! TODO Reviewed ??-??-2026.

mod tonic;
mod tonic_client;

/// Runs the grpc example.
fn main() -> anyhow::Result<()> {
    tonic::run()?;
    Ok(())
}
