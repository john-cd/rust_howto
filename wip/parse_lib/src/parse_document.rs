use std::path::Path;

use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::alt;
use winnow::combinator::fail;
use winnow::combinator::repeat;
use winnow::error::StrContext::*;
use winnow::error::StrContextValue::*;

use crate::Document;
use crate::ast::Element;
// [finish URL parsing](https://github.com/john-cd/rust_howto/issues/1427)
// use crate::urls::parse_naked_url;
// use crate::directives::parse_directive;
use crate::markdown::*;
use crate::parse_directives::parse_directive_element;
use crate::parse_hidden::parse_hidden_html_div;
// use crate::parse_urls::*;
use crate::parse_wikilinks::parse_wikilink;

/// Parse a sequence of text elements.
/// It tries to parse specific elements first, then falls back to plain text.
fn parse_elements<'a>(input: &mut &'a str) -> ModalResult<Vec<Element<'a>>> {
    repeat(
        0..,
        alt((
            parse_hidden_html_div,
            parse_fenced_code_block,
            parse_code_span,
            parse_directive_element,
            parse_atx_heading,
            parse_link_reference_definition,
            parse_wikilink,
            parse_reference_style_image,
            parse_inline_image,
            parse_reference_style_link,
            parse_inline_link,
            parse_autolink,
            // parse_naked_url,
            parse_plain_text,
            fail.context(Label("text element"))
                .context(Expected(Description("one of a hidden HTML div, fenced code block, code span, autolink, inline link, naked URL, plain text, or whitespace"))),
        )),
    )
    .parse_next(input)
}

/// Parse a Markdown document.
pub fn parse_document<'s>(
    name: &'s str,
    path: Option<&'s Path>,
    input: &'s str,
) -> anyhow::Result<Document<'s>> {
    parse_elements
        .map(|elements| Document::new(name, path, elements))
        .parse(input)
        .map_err(move |e| anyhow::anyhow!("{e}")) // ParsingError::from_parse(e))
}

#[cfg(test)]
mod tests {
    use winnow::Parser;

    use super::*;

    #[test]
    fn test_parser() {
        let text_input = r#"
# Title {#anchor}

Hello, this is some text with a `code span`.
Visit https://www.rust-lang.org for Rust info.
Here's a [link to Google](https://www.google.com).
<div class="hidden">This is a hidden HTML div block.</div>
A bit more text.
<div>this is a div content.</div>
And a final piece of text.

```
This is a block of text
enclosed in triple backticks.
It can span multiple lines
and retain its formatting.
```
"#;

        println!("Parsing Text Input:\n---\n{}\n---\n", text_input.trim());

        match parse_elements.parse_peek(text_input.trim()) {
            Ok((remaining, elements)) => {
                if remaining.is_empty() {
                    println!("Successfully parsed entire document:");
                } else {
                    println!(
                        "Parsed document with remaining input (may indicate unhandled parts):"
                    );
                    println!("Remaining: \"{remaining}\"");
                }
                for element in elements {
                    println!("{element:?}");
                }
            }
            Err(e) => {
                eprintln!("Error parsing text: {e:?}");
            }
        }
    }
}
