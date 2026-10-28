//! Runs the control systems example.
//!
//! TODO Reviewed ??-??-2026.

mod pid;

/// Runs the control systems example.
fn main() {
    pid::run();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
