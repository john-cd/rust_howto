//! Runs the build information example.

mod build_info;

/// Runs the build information example.
fn main() -> anyhow::Result<()> {
    build_info::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example end to end and asserts that it succeeds.
    #[test]
    fn test_main_runs() {
        assert!(main().is_ok());
    }
}
