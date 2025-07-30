//! - Recognize an absolute HTTP / HTTPS URL.
//! - Recognize specific URLs (e.g., specific domains and path patterns).

mod url_kind;
mod url_valid_chars;

use url_kind::*;
use url_valid_chars::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::combinator::seq;
use winnow::error::ErrMode;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;

// use crate::ast::*;

/// Recognize an absolute HTTP / HTTPS URL.
pub fn parse_naked_url<'s>(input: &mut &'s str) -> ModalResult<UrlKind<'s>> {
    // If the child parser was successful, return the consumed input as produced value.
    seq!(
        alt(("http://", "https://")), // Protocols.
        parse_url_chars,
    )
    .take()
    .map_err(ErrMode::Backtrack)
    .try_map(url_kind) // .map_err(|e| ContextError::from_external_error(input, e)))
    .context(Label("naked URL"))
    .context(Expected(Description(
        "http:// or https:// followed by valid URL characters",
    )))
    .parse_next(input)
}

// ///
// pub fn parse_naked_url_element<'s>(input: &mut &'s str) -> ModalResult<Element<'s>> {
//     // If the child parser was successful, return the consumed input as produced value.
//     seq!(
//         alt(("http://", "https://")), // Protocols.
//         parse_url_chars,
//     )
//     .take()
//     .map_err(|e| ErrMode::Backtrack(e))
//     .try_map(|url| url_kind(url)) // .map_err(|e| ContextError::from_external_error(input, e)))
//     .context(Label("naked URL"))
//     .context(Expected(Description(
//         "http:// or https:// followed by valid URL characters",
//     )))
//     .parse_next(input)
// }
