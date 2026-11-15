//! Runs the HTTP types example.
//!
//! TODO Reviewed ??-??-2026.

/// This example demonstrates the basic usage of the `http` crate for creating
mod http;
/// Runs the HTTP types and interfaces example.
///
/// # Errors
/// Returns an error if header parsing or message construction fails.
fn main() -> anyhow::Result<()> {
    http::run()
}

/// Verifies that the HTTP types example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        main().expect("the example should run");
    }
}
