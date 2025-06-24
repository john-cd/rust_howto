#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates the use of the `pest` parser generator framework
//! for parsing a simple arithmetic expression like 3 + 5 * 2 - 8 / 4.
//!
//! In `Cargo.toml`, add:
//! ```toml
//! [dependencies]
//! pest = "2.7.15"
//! pest_derive = "2.7.15"
//!
//! [build-dependencies]
//! pest_generator = "2.7.15"
//! ```
//! The `grammar.pest` file defines the grammar rules, and the ``.rs` file
//! uses the generated parser to parse the input and print the parsed pairs.
//!
//! `pest` uses parsing expression grammars (or PEG) as its input, which are
//! similar in spirit to regular expressions, but which offer the enhanced
//! expressivity needed to parse complex languages.
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../examples/pest/grammar.pest"] // Path relative to `src`
struct ArithmeticParser;

// // `pest` creates the function `ArithmeticParser::parse` and an enum called
// // `Rule` with multiple variants:
// #[allow(non_camel_case_types)]
// enum Rule {
//     expression,
//     term,
//     factor,
//     number,
// }

fn main() {
    let input = "3 + 5 * 2 - 8 / 4";
    match ArithmeticParser::parse(Rule::expression, input) {
        Ok(pairs) => {
            for pair in pairs {
                println!("{:?}", pair);
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [finish](https://github.com/john-cd/rust_howto/issues/826)
