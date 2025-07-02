// //! Parsing Utilities
// use nom::Parser;
// use nom::character::complete::multispace0;
// use nom::error::ParseError;
// use nom::sequence::delimited;
// use nom::character::char;use nom::IResult;
// use nom::bytes::complete::is_not;

// /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
// /// trailing whitespace, returning the output of `inner`.
// pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
// where
//     F: Parser<&'a str, Output = O, Error = E>,
// {
//     delimited(multispace0, inner, multispace0)
// }

// /// Parses a simple quoted string (between " and ").
// pub fn parse_quoted_string(input: &str) -> IResult<&str, &str> {
//     delimited(char('"'), is_not(r#"""#), char('"')).parse(input)
// }
