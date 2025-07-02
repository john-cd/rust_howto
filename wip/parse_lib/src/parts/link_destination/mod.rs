//! Parse link destinations.
//!
//! A link destination consists of either
//! - a sequence of zero or more characters between an opening < and a closing >
//!   that contains no line endings or unescaped < or > characters,
//!   (implemented in angle_brackets.rs), or
//! - a nonempty sequence of characters that does not start with <,
//!   does not include ASCII control characters or space character, and
//!   includes parentheses only if
//!   (a) they are backslash-escaped or
//!   (b) they are part of a balanced pair of unescaped parentheses.
//!
//! (Implementations may impose limits on parentheses nesting to avoid performance issues,
//! but at least three levels of nesting should be supported.)
//!
//! <https://spec.commonmark.org/0.31.2/#link-destination>

use nom::IResult;
// use nom::Parser;
// use nom::branch::alt;
// use nom::bytes::complete::take_while1;
// use nom::bytes::is_not;
// use nom::character::complete::char;
// use nom::character::complete::none_of;
// use nom::combinator::cut;
// use nom::combinator::recognize;
// use nom::multi::many0;
// use nom::sequence::delimited;
// use nom::sequence::preceded;

mod angle_brackets;
mod nonempty_sequence;

use angle_brackets::*;
use nom::Parser;
use nom::branch::alt;
use nonempty_sequence::*;

/// Parses a link destination.
///
/// <https://spec.commonmark.org/0.31.2/#link-destination>
pub fn parse_link_destination(input: &str) -> IResult<&str, &str> {
    alt((parse_angle_brackets, parse_non_empty_sequence)).parse(input)
}
