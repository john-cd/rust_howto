// //! Parsing Utilities
// use winnow::Parser;
// use winnow::character::multispace0;
// use winnow::error::ParseError;
// use winnow::combinator::delimited;
// use winnow::character::char;
// use winnow::Result;
// use winnow::bytes::complete::is_not;

// /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
// /// trailing whitespace, returning the output of `inner`.
// pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
// where
//     F: Parser<&'a str, Output = O, Error = E>,
// {
//     delimited(multispace0, inner, multispace0)
// }

// /// Parses a simple quoted string (between " and ").
// pub fn parse_quoted_string<'s>(input: &mut &'s str) -> Result< &'s str> {
//     delimited(""", is_not(r#"""#), """).parse_next(input)
// }
