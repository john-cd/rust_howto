// ANCHOR: example
//! Captures and prints a stack trace using the standard library.

use std::backtrace::Backtrace;

/// Captures and prints a backtrace when enabled by the environment.
fn main() -> anyhow::Result<()> {
    let backtrace = Backtrace::capture();
    println!("Backtrace status: {:?}", backtrace.status());
    println!("{backtrace}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the backtrace example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::backtrace::Backtrace;

    /// Verifies that a forced backtrace can be formatted as text.
    #[test]
    fn forced_backtrace_has_a_display_value() {
        let backtrace = Backtrace::force_capture();

        assert!(!backtrace.to_string().is_empty());
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
