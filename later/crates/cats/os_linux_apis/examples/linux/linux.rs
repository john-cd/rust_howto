#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A Linux-specific example that queries the current process ID.

/// Prints the current process ID using a safe Linux API wrapper.
#[cfg(target_os = "linux")]
fn main() {
    let pid = rustix::process::getpid();
    assert!(pid.as_raw_nonzero().get() > 0);
    println!("Linux process ID: {pid:?}");
}

/// Prints a message when this Linux-only example runs on another target.
#[cfg(not(target_os = "linux"))]
fn main() {
    println!("This example is intended for Linux.");
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example runs on Linux and has a harmless fallback
    /// elsewhere.
    #[test]
    fn example_runs_for_the_current_target() {
        main();
    }
}
