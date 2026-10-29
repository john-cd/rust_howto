//! Runs the gRPC server example.
//!
//! TODO Reviewed ??-??-2026.

mod echo;
mod grpc_web;
mod tonic;
mod tonic_client;

/// Runs the grpc example.
fn main() -> anyhow::Result<()> {
    tonic::run()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    #[ignore = "serves on [::1]:50051 until stopped"]
    fn main_runs() {
        main().expect("the example should run");
    }
}
