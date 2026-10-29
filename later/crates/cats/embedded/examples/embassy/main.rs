//! Starts the host-side Embassy task example.
// TODO Reviewed ??-??-2026.

mod embassy;

// ANCHOR: example
/// Runs the bounded host example.
fn main() -> anyhow::Result<()> {
    embassy::run()
}
// ANCHOR_END: example

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
