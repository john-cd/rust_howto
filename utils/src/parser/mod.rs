/// Functions that create a parser,
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

pub(crate) fn get_parser<'input, 'callback>(
    markdown_input: &'input str,
) -> Parser<'input, 'callback>
// where
//     S: AsRef<str> + 'input
{
    Parser::new_ext(markdown_input, get_options())
}

// Private Functions

// Parser options
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

// Callback function for broken references
//
// In case the parser encounters any potential links that have a
// broken reference (e.g [foo] when there is no [foo]:  entry at the
// bottom) the provided callback will be called with the
// reference name, and the returned pair will be used as the link name
// and title if it is not None.
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
