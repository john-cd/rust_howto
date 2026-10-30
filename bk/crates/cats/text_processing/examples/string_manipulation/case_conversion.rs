// ANCHOR: example
//! Converts words to common naming conventions and uppercases a Unicode
//! character.

use heck::ToSnakeCase as _;
use heck::ToTitleCase as _;

/// Prints common case conversions and a Unicode-aware first-character result.
fn main() -> anyhow::Result<()> {
    let phrase = "rust strings stay readable";
    println!("Title case: {}", phrase.to_title_case());
    println!("Snake case: {}", phrase.to_snake_case());
    println!("Capitalized: {}", capitalize_first_char("élan vital"));
    println!("Uppercase expansion: {}", capitalize_first_char("ßeta"));

    Ok(())
}

/// Uppercases the first Unicode scalar value without slicing at a byte offset.
fn capitalize_first_char(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

// ANCHOR_END: example

/// Runs the example for the command-line dispatcher and unit tests.
pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::capitalize_first_char;
    use super::main;

    /// Uppercases a multibyte initial scalar without panicking on UTF-8.
    #[test]
    fn case_conversion_capitalizes_multibyte_initial_character()
    -> anyhow::Result<()> {
        assert_eq!(capitalize_first_char("élan vital"), "Élan vital");
        main()
    }

    /// Preserves empty input and Unicode uppercase expansions.
    #[test]
    fn case_conversion_handles_empty_input_and_uppercase_expansion() {
        assert_eq!(capitalize_first_char(""), "");
        assert_eq!(capitalize_first_char("ßeta"), "SSeta");
    }
}
