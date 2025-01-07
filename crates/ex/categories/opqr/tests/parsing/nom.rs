// ANCHOR: example
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::separated_list0;

// `nom` is a parser combinator library, which makes it easy to build parsers
// from small, reusable components. Below is a simple example demonstrating how
// to use `nom` to parse a comma-separated list of numbers.

fn parse_number(input: &str) -> IResult<&str, u32> {
    // `digit1` recognizes one or more ASCII numerical characters: 0-9
    // `map_res` applies a function returning a `Result` over the result of a
    // parser. In this case, we parse the string of digits into an integer.
    map_res(digit1, str::parse)(input)
}

fn parse_comma_separated_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    // `tag` recognizes a pattern / string
    // `separated_list0` alternates between two parsers to produce a list of
    // elements
    separated_list0(tag(","), parse_number)(input)
}

fn main() {
    let input = "1,2,3,4,5";
    match parse_comma_separated_numbers(input) {
        Ok((remaining, result)) => {
            println!("Parsed numbers: {:?}", result);
            println!("Remaining input: {:?}", remaining);
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
