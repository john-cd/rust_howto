#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `lexopt` crate to parse command
//! line arguments.

struct Args {
    name: String,
    age: u32,
    // Add more fields here as needed.
}

fn get_args() -> anyhow::Result<Args> {
    // Allows writing Short/Long/Value without an `Arg` prefix
    // and adds convenience methods to `OsString`:
    use lexopt::prelude::*;

    // Create a parser from a fixed set of arguments for testing.
    let mut parser = lexopt::Parser::from_iter(&["myapp", "-a", "30", "John"]);
    // In a real-world app, create a parser using `std::env::args_os`:
    // `let mut parser = lexopt::Parser::from_env();`.

    let mut name = String::new();
    let mut age = 0;

    // Get the name of the command, as in the zeroth argument of the process:
    let bin_name: String = parser.bin_name().unwrap_or("unknown").into();

    // Loop the next option or positional argument:
    while let Some(arg) = parser.next()? {
        match arg {
            // `Short` and `Long` indicate an option, here `-a` or `--age`.
            Short('a') | Long("age") => {
                // `value()` returns the value that belongs to the option as a
                // standard `OsString`.
                age = parser.value()?.parse()?;
            }
            // Positional argument:
            Value(val) => {
                // `string()` converts the `OsString` into a `String` if it is
                // valid Unicode.
                name = val.string()?;
            }
            // Handle the `--help` option:
            Long("help") => {
                println!("Usage: {bin_name} [-a|--age=NUM] [NAME]");
                std::process::exit(0);
            }
            // Handle unexpected flags or options:
            _ => {
                return Err(anyhow::anyhow!("Unexpected flag or option"));
                // Consider adding more specific error messages here.
            }
        }
    }
    Ok(Args { name, age })
}

fn main() -> anyhow::Result<()> {
    let args = get_args()?;
    println!("Name: {}", args.name);
    println!("Age: {}", args.age);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
