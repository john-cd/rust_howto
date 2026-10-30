//! Standalone entry point for the Xilem counter example.

#[path = "xilem.rs"]
mod xilem;

/// Runs the native Xilem counter window.
fn main() -> Result<(), winit::error::EventLoopError> {
    run()
}

/// Runs the native Xilem counter window.
pub fn run() -> Result<(), winit::error::EventLoopError> {
    xilem::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the counter window through the standalone entry point.
    #[test]
    #[ignore = "requires an interactive desktop session"]
    fn test_main_entry_point() {
        assert!(main().is_ok());
    }
}
