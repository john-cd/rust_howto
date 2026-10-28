// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calling Rust from Ruby with `magnus`.

use magnus::Error;
/// Returns a greeting that can be called from Ruby or tested without Ruby.
fn hello_rust(name: String) -> String {
    format!("Hello, {name}!")
}

/// Registers and calls the Rust greeting through the embedded Ruby VM.
fn embedded_greeting() -> Result<String, Error> {
    // SAFETY: this function initializes the process's sole Ruby VM, uses it on
    // this thread, and keeps the cleanup guard alive through all Ruby calls.
    let ruby = unsafe { magnus::embed::init() };

    ruby.define_global_function(
        "hello_rust",
        magnus::function!(hello_rust, 1),
    )?;

    ruby.eval::<String>("hello_rust('Ada')")
}

/// Prints the greeting returned through the embedded Ruby VM.
fn main() -> Result<(), Error> {
    println!("{}", embedded_greeting()?);
    Ok(())
}
// ANCHOR_END: example

/// Runs the Magnus embedding example.
pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the Rust greeting without starting a Ruby VM.
    #[test]
    fn rust_greeting_formats_the_name() {
        assert_eq!(hello_rust("Ada".to_owned()), "Hello, Ada!");
    }

    /// Checks the value returned when embedded Ruby calls the registered Rust
    /// function.
    #[test]
    #[ignore = "needs Ruby headers and libruby"]
    fn embedded_ruby_returns_the_rust_greeting() {
        let greeting =
            embedded_greeting().expect("embedded Ruby should call hello_rust");

        assert_eq!(greeting, "Hello, Ada!");
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/1035)
