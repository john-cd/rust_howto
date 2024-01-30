/// Functions that create parsers, and
/// extract reference definitions and links
/// from said parser
mod extract_links;
mod extract_refdefs;

pub(crate) use extract_links::*;
pub(crate) use extract_refdefs::*;
use pulldown_cmark::BrokenLink;
use pulldown_cmark::CowStr;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use tracing::warn;

// Public Functions

// RETURN A PARSER WITH APPROPRIATE OPTIONS

/// Return a parser with suitable options
pub(crate) fn get_parser<'callback>(
    markdown_input: &str,
) -> Parser<'_, 'callback> {
    Parser::new_ext(markdown_input, get_options())
}

// Private Functions

/// Return suitable Parser options
fn get_options() -> Options {
    // Set up options and parser.
    // Strikethroughs, etc... are not part of the CommonMark standard
    // and we therefore must enable them explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TABLES);
    options
}

/// Callback function for broken references
///
/// In case the parser encounters any potential links that have a
/// broken reference (e.g \[foo\] when there is no \[foo\]:  entry at the
/// bottom) the provided callback will be called with the
/// reference name, and the returned pair will be used as the link
/// name and title if it is not None.
fn _callback<'input>(
    broken_link: BrokenLink<'input>,
    markdown_input: &'input str,
) -> Option<(CowStr<'input>, CowStr<'input>)> {
    warn!(
        "Issue with the markdown: reference: {}, `{}`, type: {:?}",
        broken_link.reference,
        &markdown_input[broken_link.span],
        broken_link.link_type,
    );
    Some(("https://TODO".into(), "".into()))
}
