use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::opt;
use winnow::combinator::seq;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;

use super::super::ast::Element;
use super::super::parse_parts::parse_link_destination;
use super::super::parse_parts::parse_link_label;
use super::super::parse_parts::parse_link_text;
use super::super::parse_parts::parse_link_title;

/// Parses a Markdown-style inline link: `[text](url)`.
pub fn parse_inline_link<'s>(input: &mut &'s str) -> ModalResult<Element<'s>> {
    seq!(
        parse_link_text,
        _: '(',
        parse_link_destination,
        opt(parse_link_title),
        _: ')',
    )
    .map(|(text, url, title)| Element::InlineLink { text, url, title })
    .context(Label("inline link"))
    .context(Expected(Description("[text](url)")))
    .parse_next(input)
}

/// Parses a reference-style link: `[text][label]`.
pub fn parse_reference_style_link<'s>(input: &mut &'s str) -> ModalResult<Element<'s>> {
    seq!(parse_link_text, parse_link_label,)
        .map(|(text, label)| Element::ReferenceStyleLink { text, label })
        .context(Label("reference style link"))
        .context(Expected(Description("[text][label]")))
        .parse_next(input)
}
