//! Runs the OpenID Connect authorization-code example.
//!
//! TODO Reviewed ??-??-2026.

mod oidc;

/// Runs the OpenID Connect authorization-code example.
fn main() -> anyhow::Result<()> {
    oidc::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The entry point runs offline.
    #[test]
    fn test_main_entry_point() {
        main().expect("the offline fixture should succeed");
    }
}
