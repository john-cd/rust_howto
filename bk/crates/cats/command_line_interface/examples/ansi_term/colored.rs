#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates text styling using the `colored` crate.

use colored::Colorize;

fn main() {
    // Print styled text using foreground color and bold formatting.
    println!("{}", "Success".green().bold());

    // Print styled text with a yellow foreground on a blue background.
    println!("{}", "Warning".yellow().on_blue());

    // Print red, underlined text for emphasis.
    println!("{}", "Error".red().underline());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
// TODO add to a chapter on command line interfaces with colored
