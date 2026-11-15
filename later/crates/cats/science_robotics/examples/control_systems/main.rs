//! Runs the control systems example.
//!
//! TODO Reviewed ??-??-2026.

/// Drive a simulated first-order plant to a setpoint with a PI controller.
mod pid;
/// Runs the example.
/// Runs the control systems example.

fn main() {
    pid::run();
}

/// test their own results).
#[cfg(test)]
mod tests {
    use super::*;

    /// test their own results).
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
