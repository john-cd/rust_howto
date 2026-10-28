//! Runs the low-level system call example.
//!
//! TODO Reviewed ??-??-2026.

mod libc;

/// Runs the low level system calls example.
fn main() {
    libc::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
