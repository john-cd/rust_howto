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

use winnow::Result;
// use winnow::Parser;
// use winnow::branch::alt;
// use winnow::token::take_while1;
// use winnow::bytes::is_not;
// use winnow::bytes::one_of;
// use winnow::character::complete::none_of;
// use winnow::combinator::cut;
// use winnow::combinator::recognize;
// use winnow::multi::many0;
// use winnow::combinator::delimited;
// use winnow::sequence::preceded;

mod angle_brackets;
mod nonempty_sequence;

use angle_brackets::*;
use winnow::Parser;
use winnow::branch::alt;
use nonempty_sequence::*;

/// Parses a link destination.
///
/// <https://spec.commonmark.org/0.31.2/#link-destination>
pub fn parse_link_destination<'s>(input: &mut &'s str) -> Result< &'s str> {
    alt((parse_angle_brackets, parse_non_empty_sequence)).parse_next(input)
}
