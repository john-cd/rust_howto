// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Parsing command-line arguments with `pico-args`.

use pico_args::Arguments;

/// Parses the optional `--name` and `--age` values, the `--verbose` flag and a
/// free argument, then prints them.
fn main() -> anyhow::Result<()> {
    // Returns the arguments that this program was started with.
    // The executable path will be removed.
    let mut args = Arguments::from_env();

    // Parse named arguments:
    // `--name` and `--age` are optional key-value pairs, parsed using the
    // `FromStr` trait. This is a shorthand for
    // `opt_value_from_fn("--name", FromStr::from_str)`.
    let name: Option<String> = args.opt_value_from_str("--name")?;
    // The `FromStr` trait is used to parse the value:
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

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the pico args example through its entry point.
    #[test]
    fn test_pico_args_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    #[test]
    fn options_flags_and_free_arguments_are_extracted() {
        let mut args = Arguments::from_vec(
            ["--name", "Ann", "--age", "7", "--verbose", "f.txt"]
                .map(Into::into)
                .to_vec(),
        );
        assert_eq!(
            args.opt_value_from_str::<_, String>("--name")
                .unwrap()
                .as_deref(),
            Some("Ann")
        );
        assert_eq!(
            args.opt_value_from_str::<_, u32>("--age").unwrap(),
            Some(7)
        );
        assert!(args.contains("--verbose"));
        assert_eq!(
            args.opt_free_from_str::<String>().unwrap().as_deref(),
            Some("f.txt")
        );
        assert!(args.finish().is_empty());
    }

    #[test]
    fn invalid_numbers_and_leftovers_are_reported() {
        let mut args = Arguments::from_vec(
            ["--age", "x", "--extra"].map(Into::into).to_vec(),
        );
        assert!(args.opt_value_from_str::<_, u32>("--age").is_err());
        assert_eq!(
            args.finish().len(),
            3,
            "a failed parse leaves its arguments unconsumed"
        );
    }
}
