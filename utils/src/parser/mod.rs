mod debug;

mod parse;
mod refdefs;
mod rules;
mod test;
mod write;

use std::io::Write;
use std::path::Path;

use anyhow::Result;
use pulldown_cmark::BrokenLink;
use pulldown_cmark::CowStr;
use pulldown_cmark::LinkType;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;
use tracing::debug;
use tracing::warn;

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

// Public Functions

/// Parse a Markdown string and write all raw events to e.g. a file
/// for debugging purposes
///
/// markdown_input: &str equivalent
/// dest_file_path: path to the file to create and write into
pub fn debug_parse_to<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    dest_file_path: P,
) -> Result<()> {
    debug!("\nParsing markdown ---------------\n");
    let f = std::fs::File::create(dest_file_path)?;
    let parser = Parser::new_ext(markdown_input.as_ref(), get_options());
    debug::debug_parse_to(parser, f)?;
    Ok(())
}

// REFERENCE DEFINTIONS

/// Parse a Markdown string and write reference definitions found
/// therein to a file, given a path
///
/// markdown_input: &str equivalent
/// dest_file_path: path to the file to create and write into
pub fn write_ref_defs_to<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    dest_file_path: P,
) -> Result<()> {
    let f = std::fs::File::create(dest_file_path)?;
    let parser = Parser::new_ext(markdown_input.as_ref(), get_options());
    refdefs::write_ref_defs(&parser, f)?;
    Ok(())
}

/// Get existing reference definitions from a Markdown string,
/// identify URLs that are GitHub repos, create badge URLs for these
/// links, and write to a file, given a path.
///
/// markdown_input: &str equivalent
/// dest_file_path: path to the file to create and write into
pub fn generate_badges<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    dest_file_path: P,
) -> Result<()> {
    let mut f = std::io::BufWriter::new(std::fs::File::create(dest_file_path)?);
    let parser = Parser::new_ext(markdown_input.as_ref(), get_options());
    refdefs::write_github_repo_badge_refdefs(&parser, &mut f)?;
    f.flush().unwrap();
    Ok(())
}

// LINKS

// TODO need to remove internal links; deduplicate code

/// Parse a Markdown string and write all inline links and autolinks
/// (i.e., not written as reference-style links) found therein to a
/// file
///
/// markdown_input: &str equivalent
/// dest_file_path: path to the file to create and write into
pub fn write_inline_links<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    dest_file_path: P,
) -> Result<()> {
    let parser = Parser::new_ext(markdown_input.as_ref(), get_options());

    let mut f = std::fs::File::create(dest_file_path)?;
    let links: Vec<super::link::Link> = parse::extract_links(parser);
    let links: Vec<_> = links
        .iter()
        .filter(|l| {
            [LinkType::Inline, LinkType::Autolink]
                .iter()
                .any(|&x| l.get_link_type().unwrap() == x)
        })
        .collect();
    if !links.is_empty() {
        for l in links {
            writeln!(
                &mut f,
                "{:?}\n{}\n{}\n",
                l,
                l.to_reference_link(),
                l.to_reference_definition()
            )?;
        }
    }
    Ok(())
}

// Write all links to a file
/// markdown_input: &str equivalent
/// dest_file_path: path to the file to create and write into
pub fn write_links<S: AsRef<str>, P: AsRef<Path>>(
    markdown_input: S,
    dest_file_path: P,
) -> Result<()> {
    let parser = Parser::new_ext(markdown_input.as_ref(), get_options());
    let mut f = std::fs::File::create(dest_file_path)?;

    let links: Vec<super::link::Link> = parse::extract_links(parser);
    if !links.is_empty() {
        for l in links {
            writeln!(
                &mut f,
                "{:?}\n{}\n{}\n{}\n{}\n{}\n",
                l,
                l.to_inline_link(),
                l.to_reference_link(),
                l.to_reference_definition(),
                l.to_link_with_badge(),
                l.to_badge_reference_definition()
            )?;
        }
    }
    Ok(())
}

// TODO

// let markdown_input_length = markdown_input.as_ref().len();
// write_markdown_to(parser, markdown_input_length, f)?;

// TODO

//// Set up the parser. We can treat is as any other iterator.
//// For each event, we print its details, such as the tag or string.
// let parser = Parser::new_with_broken_link_callback(
//     markdown_input.as_ref(),
//     get_options(),
//     Some(&mut |broken_link: BrokenLink| { callback(broken_link,
// markdown_input.as_ref()) }), )
