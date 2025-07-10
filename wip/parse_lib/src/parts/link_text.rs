//! Parses link text.
//!
//! Simplified from <https://spec.commonmark.org/0.31.2/#link-text>:
//!
//! A link text consists of a sequence of zero or more inline elements enclosed by square brackets ([ and ]). The following rules apply:
//! - Links may not contain other links, at any level of nesting. If multiple otherwise valid link definitions appear nested inside each other, the inner-most definition is used.
//! - Brackets are allowed in the link text only if (a) they are backslash-escaped or (b) they appear as a matched pair of brackets, with an open bracket [, a sequence of zero or more inlines, and a close bracket ].
//! - Backtick code spans, autolinks, and raw HTML tags bind more tightly than the brackets in link text. Thus, for example, [foo`]` could not be a link text, since the second ] is part of a code span.
//! - The brackets in link text bind more tightly than markers for emphasis and strong emphasis. Thus, for example, `*[foo*](url)` is a link.

use winnow::Parser;
use winnow::ascii::alphanumeric0;
use winnow::ascii::space0;
use winnow::ascii::take_escaped;
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::fail;
use winnow::combinator::repeat;
use winnow::error::ErrMode;
use winnow::error::ModalResult;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;
use winnow::token::one_of;

/// Parses text that may contain escaped brackets or backslashes: \[ or \] or \\.
fn parse_text_with_escapes<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    let allowed_chars = alt((alphanumeric0, space0));

    take_escaped(allowed_chars, '\\', one_of(['[', ']', '\\']))
        .context(Label("text with Markdown 'link text'"))
        .context(Expected(Description(
            r"alphanumeric characters, spaces, or tabs. May contain \-escaped [ or ] or \",
        )))
        .parse_next(input)
        .map_err(|e| ErrMode::Backtrack(e))
}

/// Parses content within balanced brackets.
/// It can contain text with escaped brackets, or nested balanced parentheses.
fn parse_balanced_brackets_content<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    repeat::<_, _, String, _, _>(
        0..,
        alt((
            parse_text_with_escapes,
            parse_balanced_brackets, // Recursively parse.
            fail.context(Label("balanced brackets content"))
                .context(Expected(Description(
                    "text (with possibly escaped brackets) or nested balanced brackets",
                ))),
        )),
    )
    .take()
    .parse_next(input)
}

// Parse pairs of brackets within Markdown 'link text' (e.g., `[bar]` within `[foo [bar] baz]`)
// This inner content *cannot* contain full links.
fn parse_balanced_brackets<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    delimited(
        "[",
        parse_balanced_brackets_content,
        cut_err("]"), // `cut` ensures a matching `]` must be found.
    )
    .take()
    .context(Label(
        "balanced brackets nested within Markdown 'link text'",
    ))
    .context(Expected(Description(
        "matched pairs of brackets within the link text, e.g., [foo [bar] baz]",
    )))
    .parse_next(input)
}

/// Parses the contents of a Markdown link's text.
fn parse_link_text_content<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    repeat::<_, _, String, _, _>(
        0..,
        alt((
            // TODO parse image
            parse_balanced_brackets,
            parse_text_with_escapes,
            fail.context(Label("link text content"))
                .context(Expected(Description(""))),
        )),
    )
    .take()
    .parse_next(input)
}

/// Parses a Markdown link's text, which is enclosed in square brackets `[]`.
///
/// This parser handles:
/// - Any character within the brackets.
/// - Escaped square brackets (`\[` and `\]`) which should be treated as literal characters.
///
/// # Arguments
/// * `input` - The input string slice to parse.
///
/// # Returns
/// A `Result` containing the parsed link text as a `String` if successful,
/// or a `winnow::error::Err` if parsing fails.
///
/// # Examples
/// ```rust
/// use winnow::Parser;
/// use parse_lib::parts::parse_link_text;
///
/// assert_eq!(parse_link_text.parse("[Hello World]").unwrap(), "Hello World");
/// assert_eq!(parse_link_text.parse("[Link with \\[bracket\\]]").unwrap(), "Link with [bracket]");
/// assert_eq!(parse_link_text.parse("[]").unwrap(), "");
/// ```
pub(crate) fn parse_link_text<'s>(input: &mut &'s str) -> ModalResult<&'s str> {
    // The entire link text is delimited by `[` and `]`.
    delimited('[', parse_link_text_content, cut_err(']'))
        .context(Label("link text"))
        .context(Expected(Description(
            "content of a Markdown link's text, enclosed in square brackets",
        )))
        .parse_next(input)
}

// --- Test Cases ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_link_text_basic() {
        assert_eq!(
            parse_link_text.parse_peek("[Hello World]"),
            Ok(("", "Hello World"))
        );
        assert_eq!(
            parse_link_text
                .parse(r"[Link with escaped \[bracket\] xyz]")
                .unwrap(),
            r"Link with escaped \[bracket\] xyz"
        );
        assert_eq!(parse_link_text.parse("[]").unwrap(), "");
    }

    #[test]
    fn test_parse_link_text_escaped_brackets() {
        assert_eq!(parse_link_text.parse(r"[ \[foo\] ]").unwrap(), r" \[foo\] ");
        assert_eq!(
            parse_link_text.parse(r"[foo \[bar\] baz]").unwrap(),
            r"foo \[bar\] baz"
        );
    }

    #[test]
    fn test_parse_link_text_nested_brackets() {
        assert_eq!(
            parse_link_text.parse("[foo [bar] baz]").unwrap(),
            "foo [bar] baz"
        );
        assert_eq!(
            parse_link_text.parse("[a [b [c] d] e]").unwrap(),
            "a [b [c] d] e"
        );
    }

    #[test]
    fn test_parse_link_text_escaped_backslash() {
        assert_eq!(
            parse_link_text.parse(r"[foo \\ bar]").unwrap(),
            r"foo \\ bar"
        );
    }

    #[test]
    fn test_parse_link_text_unmatched_bracket() {
        assert!(parse_link_text.parse("[foo [bar]").is_err());
        assert!(parse_link_text.parse("[foo bar]]").is_err());
    }

    #[test]
    fn test_parse_link_text_only_escapes() {
        assert_eq!(parse_link_text.parse(r"[\[\]\[\]]").unwrap(), r"\[\]\[\]");
    }

    #[test]
    fn test_parse_link_text_empty_and_whitespace() {
        assert_eq!(parse_link_text.parse("[   ]").unwrap(), "   ");
        assert_eq!(parse_link_text.parse("[\t]").unwrap(), "\t");
    }
}
