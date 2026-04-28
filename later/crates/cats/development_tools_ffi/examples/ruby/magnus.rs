#![allow(dead_code)]
// ANCHOR: example
use magnus::Error;
use magnus::RString;
use magnus::class;
use magnus::value::ReprValue;

/// Simple function to be called from Ruby.
fn hello_rust() -> String {
    String::from("Hello from Rust again!")
}

fn main() -> Result<(), Error> {
    // Initialize the Ruby VM.
    // In actual usage, you might use magnus::init()? but it requires a Ruby
    // environment.

    // Create a new Ruby string object from the given Rust string.
    let string = RString::new("Hello from Rust!");

    // Safely call the `puts` method via the Object class without evaluating
    // dynamic strings.
    class::object().funcall::<_, _, magnus::Value>("puts", (string,))?;

    // Define a global Ruby function that calls a Rust function.
    // define_global_function("hello_rust", hello_rust);

    // Evaluate Ruby code that calls the Rust function
    // eval::<magnus::Value>(
    // r#"
    // puts hello_rust()
    // "#,
    // )?;

    Ok(())
}

// ANCHOR_END: example

// [finish](https://github.com/john-cd/rust_howto/issues/1035)

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        // main(); // Requires Ruby VM to be initialized and linked
    }
}
