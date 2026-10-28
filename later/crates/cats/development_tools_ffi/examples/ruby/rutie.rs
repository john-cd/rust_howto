// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calling Rust from Ruby with `rutie`.

use rutie::Object as _;
use rutie::RString;
use rutie::VM;

/// Example
///
///  1. Compile the Rust code:
/// ```bash
/// cargo build --release
/// ```
///
/// 2. Run the Ruby script using a Ruby interpreter:
/// ```bash
/// ruby script.rb
/// ```

/// Mark the `RustModule` function as a Ruby module.
// Note: In a real rutie project, you'd use macros to export this.
// This is a simplified representation for documentation.

/// Prints a greeting to the console.
pub extern "C" fn hello(
    _argc: i32,
    _argv: *const rutie::types::Value,
    _slf: rutie::types::Value,
) -> rutie::types::Value {
    RString::new_utf8("Hello from Rust!").value()
}

/// Initializes the Ruby VM.
fn main() {
    VM::init();
    // In a real application, you would define modules and classes here.
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rutie example through its entry point.
    #[test]
    #[ignore = "needs Ruby; run this test alone in its own process"]
    fn test_rutie_example_entry_point() {
        main();
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/1036)
