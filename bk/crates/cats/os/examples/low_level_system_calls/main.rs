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
