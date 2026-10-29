// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calling Ruby from Rust with `rutie`.
//!
//! Install 64-bit Ruby 4.0.x with development headers and libraries, then run
//! the example or its ignored Ruby-backed test from `later/crates`:
//!
//! ```bash
//! cargo run --locked -p development_tools_ffi --features ruby --example ruby -- rutie
//! cargo test --locked -p development_tools_ffi --features ruby --example ruby test_rutie_example_entry_point -- --ignored --test-threads=1
//! ```

use rutie::AnyException;
use rutie::Class;
use rutie::Object;
use rutie::RString;
use rutie::VM;
use rutie::class;
use rutie::methods;

class!(RustGreeter);

methods!(
    RustGreeter,
    _rtself,
    fn greeting() -> RString {
        RString::new_utf8("Hello from Rutie!")
    }
);

/// Initializes Ruby, registers `RustGreeter`, and evaluates its greeting
/// method.
fn main() -> Result<String, AnyException> {
    VM::init();
    Class::new("RustGreeter", None).define(|class| {
        class.def("greeting", greeting);
    });

    let greeting =
        VM::eval("RustGreeter.new.greeting")?.try_convert_to::<RString>()?;
    Ok(greeting.to_string())
}
// ANCHOR_END: example

/// Prints the greeting produced by an embedded Ruby interpreter.
pub fn run() -> Result<(), AnyException> {
    println!("{}", main()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::main;

    /// Verifies that Ruby can construct the class and call its Rust method.
    #[test]
    #[ignore = "needs Ruby"]
    fn test_rutie_example_entry_point() {
        let greeting =
            main().expect("the embedded Ruby greeting should evaluate");
        assert_eq!(greeting, "Hello from Rutie!");
    }
}
