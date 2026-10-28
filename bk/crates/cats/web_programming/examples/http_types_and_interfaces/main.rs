//! Runs the HTTP types example.
//!
//! TODO Reviewed ??-??-2026.

mod http;

/// Runs the HTTP types and interfaces example.
///
/// # Errors
///
/// Returns an error if header parsing or message construction fails.
fn main() -> anyhow::Result<()> {
    http::run()
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
