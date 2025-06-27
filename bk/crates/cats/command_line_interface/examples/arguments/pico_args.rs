#![allow(dead_code)]
// ANCHOR: example
use pico_args::Arguments;

fn main() -> anyhow::Result<()> {
    // Returns the arguments that this program was started with.
    // The executable path will be removed.
    let mut args = Arguments::from_env();

    // Parse named arguments:
    // --name and --age are optional key-value pairs, parsed using the
    // `FromStr` trait. This is a shorthand for
    // `opt_value_from_fn("--name", FromStr::from_str)`.
    let name: Option<String> = args.opt_value_from_str("--name")?;
    // The `FromStr` trait is used to parse the value.
    let age: Option<u32> = args.opt_value_from_str("--age")?;

    // Parse a flag:
    let verbose = args.contains("--verbose");

    // Parse a positional ("free-standing") argument:
    let file: Option<String> = args.opt_free_from_str()?;

    // Print the parsed arguments:
    if let Some(name) = name {
        println!("Name: {name}.");
    }

    if let Some(age) = age {
        println!("Age: {age}.");
    }

    if verbose {
        println!("Verbose mode is on.");
    }

    if let Some(file) = file {
        println!("File: {file}.");
    }

    // Check for unused arguments:
    let remaining_args = args.finish();
    if !remaining_args.is_empty() {
        eprintln!("Warning: unused arguments! {remaining_args:?}.");
    }

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
