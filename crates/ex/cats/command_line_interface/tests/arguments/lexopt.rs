// ANCHOR: example
use lexopt::ValueExt;

// Simple command line argument parser

struct Args {
    name: String,
    age: u32,
}

fn get_args() -> anyhow::Result<Args> {
    // Allows writing Short/Long/Value without an Arg prefix
    // and adds convenience methods to OsString:
    use lexopt::prelude::*;

    // Create a parser using `std::env::args_os`
    let mut parser = lexopt::Parser::from_env();
    let mut name = String::new();
    let mut age = 0;

    // Print the name of the command, as in the zeroth argument of the process
    let bin_name = parser.bin_name().unwrap_or("unknown");
    println!("{}", bin_name);

    // Loop the next option or positional argument
    while let Some(arg) = parser.next()? {
        match arg {
            // -n or --name
            Short('n') | Long("name") => {
                // Convert the OsString into a String if it’s valid Unicode.
                name = parser.string()?;
            }
            Long("age") => {
                age = parser.value()?.parse()?;
            }
            Long("help") => {
                println!("Usage: myapp [-n|--name=NAME] AGE");
                std::process::exit(0);
            }
            lexopt::Arg::Value(val) => {
                anyhow::anyhow!("Unexpected argument: {:?}", val);
            }
            _ => {
                anyhow::anyhow!("Unexpected flag or option");
            }
        }
    }
    Ok(Args { name, age })
}

fn main() -> anyhow::Result<()> {
    // Simulate command-line arguments
    unsafe {
        std::env::set_var("CARGO_BIN_EXE_lexopt_example", "lexopt_example");
        std::env::set_var(
            "CARGO_BIN_EXE_lexopt_example",
            "--name John --age 30",
        );
    }
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

// [P1](https://github.com/john-cd/rust_howto/issues/678) finish
