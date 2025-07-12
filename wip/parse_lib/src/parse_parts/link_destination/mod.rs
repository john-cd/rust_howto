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

mod angle_brackets;
mod nonempty_sequence;

pub(crate) use angle_brackets::*;
use nonempty_sequence::*;
use winnow::combinator::alt;
use winnow::combinator::fail;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::prelude::*;

/// Parses a link destination.
///
/// <https://spec.commonmark.org/0.31.2/#link-destination>
pub(crate) fn parse_link_destination<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    alt((
        parse_angle_brackets,
        parse_non_empty_sequence,
        fail
        .context(Label("link destination"))
        .context(Expected(Description(r"a sequence of zero or more characters between an opening < and a closing >; or a nonempty sequence of characters that does not start with <")))
    ))
    .parse_next(input)
}
