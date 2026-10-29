//! Runs the ahead-of-time macro helper example.

mod watt;

/// Runs the `watt` token-transformation example.
fn main() -> anyhow::Result<()> {
    watt::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
