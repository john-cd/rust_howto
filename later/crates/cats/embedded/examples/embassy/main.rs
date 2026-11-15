// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Starts the host-side Embassy task example.

/// Runs a bounded pair of Embassy tasks on the host executor.
mod embassy;

/// Runs the bounded host example.
fn main() -> anyhow::Result<()> {
    embassy::run()
}
// ANCHOR_END: example

/// Runs the example through its program entry point.
///
/// # Errors
/// Returns an error if the example operation fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Starts the host-side Embassy task example.
///
/// # Errors
/// Returns an error if an operation in the example fails.
/// Returns an error if a step of the example fails.

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }
}
