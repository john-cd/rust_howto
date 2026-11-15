//! Runs the low-level system call example.
//!
//! TODO Reviewed ??-??-2026.

/// This example demonstrates how to use `libc` to call into the C standard
mod libc;
/// Runs the example.
/// Runs the low level system calls example.

fn main() {
    libc::run();
}

/// Verifies that the low-level system call example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
