//! Example of writing a cargo subcommand.
//!
//! TODO Reviewed ??-??-2026.

mod cargo_subcommand;

/// Runs the `cargo-hello` subcommand with the arguments supplied by Cargo.
///
/// # Errors
///
/// Returns an error if Cargo cannot read metadata for the selected workspace.
fn main() -> anyhow::Result<()> {
    cargo_subcommand::run_from(std::env::args_os())
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
