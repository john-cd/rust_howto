#![allow(dead_code)]
// ANCHOR: example
//! `nom` is a parser combinator library, which makes it easy to build parsers
//! from small, reusable components. Below is a simple example demonstrating how
//! to use `nom` to parse a comma-separated list of numbers.

use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
// Parser for handling lists:
use nom::multi::separated_list0;

fn parse_number(input: &str) -> IResult<&str, u32> {
    // `digit1` is a parser that recognizes one or more ASCII numerical
    // characters (0-9). It returns a string slice containing the matched
    // digits.
    //
    // `map_res` is a combinator that applies a function returning a `Result`
    // over the result of a parser. In this case, we use `str::parse` to
    // convert the string of digits into a `u32` integer. If `str::parse`
    // fails (e.g., the string is not a valid number), `map_res` will return an
    // error.
    map_res(digit1, str::parse).parse(input)
}

/// `parse_comma_separated_numbers` parses a comma-separated list of numbers.
/// It returns a vector of `u32` integers.
fn parse_comma_separated_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    // `tag` recognizes a pattern / string.
    // `separated_list0` alternates between two parsers to produce a list of
    // elements.
    separated_list0(tag(","), parse_number).parse(input)
}

fn main() {
    let input = "1,2,3,4,5";
    match parse_comma_separated_numbers(input) {
        Ok((remaining, result)) => {
            println!("Parsed numbers: {result:?}");
            println!("Remaining input: {remaining:?}");
        }
        Err(err) => println!("Error: {err:?}"),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
