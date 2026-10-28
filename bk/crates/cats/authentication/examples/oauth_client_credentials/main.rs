//! Runs the OAuth 2.0 client-credentials example.
//!
//! TODO Reviewed ??-??-2026.

mod client_credentials;

/// Runs the OAuth 2.0 client-credentials example.
fn main() -> anyhow::Result<()> {
    client_credentials::run()
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
