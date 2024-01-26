/// Functions that read from a Markdown parser and write the whole
/// content to various outputs / formats
use std::borrow::Borrow;
use std::io::Write;

use anyhow::Result;
use pulldown_cmark::html;
use pulldown_cmark::Event;

// HTML

/// Read from a Markdown parser and write HTML to standard output
///
/// parser: Markdown parser
#[allow(dead_code)]
pub(crate) fn write_html_to_stdout<'a, I>(parser: I)
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

/// Read from a Markdown parser and write HTML to bytes
///
/// parser: Markdown parser
#[allow(dead_code)]
pub(crate) fn write_html_to_bytes<'a, I>(parser: I) -> Result<Vec<u8>>
where
    I: Iterator<Item = Event<'a>>,
{
    let mut bytes = Vec::new();
    // A Cursor wraps an in-memory buffer
    html::write_html(std::io::Cursor::new(&mut bytes), parser)?;
    Ok(bytes)
}

/// Read from a Markdown parser and write HTML to string
///
/// parser: Markdown parser
#[allow(dead_code)]
pub(crate) fn write_html_to_string<'a, I>(parser: I) -> String
where
    I: Iterator<Item = Event<'a>>,
{
    // Write to a new String buffer
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

// MARKDOWN

/// Read from a Markdown parser and write Markdown to a writer (e.g.
/// File).
///
/// parser: Markdown parser
/// w: Writer e.g. File
#[allow(dead_code)]
pub(crate) fn write_markdown_to<'a, I, E, W>(
    parser: I,
    markdown_input_length: usize,
    w: &mut W,
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
