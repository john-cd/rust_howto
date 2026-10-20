#![allow(dead_code)]
// ANCHOR: example
use rutie::Object;
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

/// Mark the RustModule function as a Ruby module.
// Note: In a real rutie project, you'd use macros to export this.
// This is a simplified representation for documentation.

pub extern "C" fn hello(
    _argc: i32,
    _argv: *const rutie::types::Value,
    _slf: rutie::types::Value,
) -> rutie::types::Value {
    RString::new_utf8("Hello from Rust!").value()
}

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

    #[test]
    #[ignore = "requires Ruby VM initialization"]
    fn test_main() {
        main();
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/1036)
