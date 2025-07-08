// TODO finish reorg

// use winnow::combinator::fail;
// use winnow::error::StrContext::*;
// use winnow::error::StrContextValue::*;

// /// Parses any characters until the next recognized element or end of input.
// /// This acts as a "fallback" for plain text.
// fn parse_plain_text<'a>(input: &mut &'a str) -> Result< Element<'a>> {
//     // Recognize text until the start of a known pattern or end of input.
//     // This is a bit tricky as we want to consume as much as possible,
//     // but stop before the *next* possible recognized element.
//     // For simplicity, this will just take until a newline or a common delimiter.
//     // A more robust solution would involve a `take_while` that looks ahead.

//     let content = many1(none_of("<[hf \n\r"))).take().parse_next(input)?;

//     // If the content is empty (e.g., we start immediately with a special tag),
//     // then it's not a plain text element.
//     if content.is_empty() {
//         return Err(winnow::Err::Error(winnow::error::Error::new(
//             input,
//             ErrorKind::Many1,
//         )));
//     }

//     Ok((remaining, Element::Text(content)))
// }

// /// Parses a sequence of text elements.
// /// It tries to parse specific elements first, then falls back to plain text.
// pub fn parse_text_document<'a>(input: &mut &'a str) -> Result< Vec<Element<'a>>> {
//     many0(alt((
//         parse_hidden_html_div,
//         parse_fenced_code_block,
//         parse_code_span,
//         parse_autolink,
//         parse_inline_link,
//         parse_standalone_url,
//         parse_plain_text,
//         // Handle whitespace explicitly if it's not part of an element
//         map(multispace0, Element::Text),
// fail     .context(Label("TODO"))
//    .context(Expected(Description("")))
//     ))).parse_next(input)
// }



// #[test]
// fn test_parser() {
//     let text_input = r#"
// Hello, this is some text.
// Visit https://www.rust-lang.org for Rust info.
// Here's a [link to Google](https://www.google.com).
// <div>This is an HTML div block.</div>
// A bit more text.
// <div>Nested divs are not fully supported yet, but this is a div content.</div>
// And a final piece of text.

// '''
// This is a block of text
// enclosed in triple quotes.
// It can span multiple lines
// and retain its formatting.
// '''
// "#;

//     println!("Parsing Text Input:\n---\n{}\n---\n", text_input.trim());

//     match parse_text_document(text_input.trim()) {
//         Ok((remaining, elements)) => {
//             if remaining.is_empty() {
//                 println!("Successfully parsed entire document:");
//             } else {
//                 println!("Parsed document with remaining input (may indicate unhandled parts):");
//                 println!("Remaining: \"{}\"", remaining);
//             }
//             for element in elements {
//                 println!("{:?}", element);
//             }
//         }
//         Err(e) => {
//             eprintln!("Error parsing text: {:?}", e);
//         }
//     }
