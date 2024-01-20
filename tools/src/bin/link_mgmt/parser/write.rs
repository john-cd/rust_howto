use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::io::Write;

use anyhow::Result;
use pulldown_cmark::html;
use pulldown_cmark::Event;
use pulldown_cmark::LinkDef;
use pulldown_cmark::Parser;

#[allow(dead_code)]
pub fn write_html_to_stdout<'a, I>(parser: I)
where
    I: Iterator<Item = Event<'a>>,
{
    // Write to stdout. This could also be anything implementing the
    // `Write` trait e.g., a file or network socket.
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"\nHTML output:\n").unwrap();
    html::write_html(&mut handle, parser).unwrap();
}

#[allow(dead_code)]
pub fn write_html_to_bytes<'a, I>(parser: I) -> Result<Vec<u8>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut bytes = Vec::new();
    // A Cursor wraps an in-memory buffer
    html::write_html(std::io::Cursor::new(&mut bytes), parser)?;
    Ok(bytes)
}

#[allow(dead_code)]
pub fn write_html_to_string<'a, I>(parser: I) -> String
where
    I: Iterator<Item = Event<'a>>,
{
    // Write to a new String buffer
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

#[allow(dead_code)]
pub fn write_markdown_to<'a, I, E, W>(
    parser: I,
    markdown_input_length: usize,
    mut w: W,
) -> Result<()>
where
    I: Iterator<Item = E>,
    E: Borrow<Event<'a>>,
    W: Write,
{
    let mut buf = String::with_capacity(markdown_input_length + 128);
    let options = pulldown_cmark_to_cmark::Options {
        // newlines_after_headline:,
        // newlines_after_paragraph:,
        // newlines_after_codeblock:,
        // newlines_after_table:,
        // newlines_after_rule:,
        // newlines_after_list:,
        // newlines_after_blockquote:,
        // newlines_after_rest:,
        // code_block_token_count:,
        // code_block_token: '',
        // list_token: '',
        // ordered_list_token: '',
        // increment_ordered_list_bullets: true,
        // emphasis_token: '',
        // strong_token: "",
        ..pulldown_cmark_to_cmark::Options::default()
    };
    pulldown_cmark_to_cmark::cmark_with_options(parser, &mut buf, options)?;
    w.write_all(buf.as_bytes())?;
    Ok(())
}

pub fn write_ref_defs<W>(parser: &Parser, mut f: W) -> Result<()>
where
    W: Write,
{
    // BTreeMap is a sorted map
    let sorted_refdefs: BTreeMap<_, _> =
        parser.reference_definitions().iter().collect();

    for (s, LinkDef { dest, title, .. }) in sorted_refdefs {
        if let Some(t) = title {
            writeln!(&mut f, "[{s}]: {dest} \"{t:?}\"")?;
        } else {
            writeln!(&mut f, "[{s}]: {dest}")?;
        }
    }
    Ok(())
}