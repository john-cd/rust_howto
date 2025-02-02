// ANCHOR: example

// Simple command line argument parser

struct Args {
    name: String,
    age: u32,
}

fn get_args() -> anyhow::Result<Args> {
    // Allows writing Short/Long/Value without an Arg prefix
    // and adds convenience methods to OsString:
    use lexopt::prelude::*;

    let mut parser = lexopt::Parser::from_iter(&["myapp", "-a", "30", "John"]);
    // In a real world app, create a parser using `std::env::args_os`
    // let mut parser = lexopt::Parser::from_env();
    let mut name = String::new();
    let mut age = 0;

    // Get the name of the command, as in the zeroth argument of the process
    let bin_name: String = parser.bin_name().unwrap_or("unknown").into();

    // Loop the next option or positional argument
    while let Some(arg) = parser.next()? {
        match arg {
            // `Short` and `Long` indicate an option, here -a or --age
            Short('a') | Long("age") => {
                // `value()` returns the value that belongs to the option as a
                // standard `OsString`.
                age = parser.value()?.parse()?;
            }
            // Positional argument
            Value(val) => {
                // `string()` converts the `OsString` into a `String` if it is
                // valid Unicode.
                name = val.string()?;
            }
            Long("help") => {
                println!("Usage: {bin_name} [-a|--age=NUM] [NAME]");
                std::process::exit(0);
            }
            _ => {
                return Err(anyhow::anyhow!("Unexpected flag or option"));
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

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// ANCHOR_END: example
