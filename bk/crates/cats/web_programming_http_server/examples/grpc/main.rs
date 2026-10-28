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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
