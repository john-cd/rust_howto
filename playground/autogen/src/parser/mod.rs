// #![allow(dead_code)]
// use pest::Parser;
// use pest_derive::Parser;
// use pest::error::Error;

// #[derive(Parser)]
// #[grammar = "parser/markdown_links.pest"]
// struct MarkdownLinkParser;

// #[derive(Debug, PartialEq)]
// enum MarkdownElement<'a> {
//     Link {
//         text: &'a str,
//         url: &'a str,
//     },
//     ReferenceDefinition {
//         label: &'a str,
//         url: &'a str,
//     },
//     Text(&'a str),
// }

// fn parse_markdown(input: &str) -> Result<Vec<MarkdownElement>, Error<Rule>> {
//     let mut pairs = MarkdownLinkParser::parse(Rule::document, input)?;
//     let mut elements = Vec::new();

//     for pair in pairs.next().unwrap().into_inner() {
//         match pair.as_rule() {
//             Rule::link => {
//                 let mut inner_pairs = pair.into_inner();
//                 let text = inner_pairs.next().unwrap().as_str();
//                 let url = inner_pairs.next().unwrap().as_str();
//                 elements.push(MarkdownElement::Link { text, url });
//             }
//             Rule::reference_definition => {
//                 let mut inner_pairs = pair.into_inner();
//                 let label = inner_pairs.next().unwrap().as_str();
//                 let url = inner_pairs.next().unwrap().as_str();
//                 elements.push(MarkdownElement::ReferenceDefinition { label, url });
//             }
//             Rule::text => {
//                 elements.push(MarkdownElement::Text(pair.as_str()));
//             }
//             Rule::EOI => (),
//             _ => unreachable!(),
//         }
//     }

//     Ok(elements)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_link() {
//         let input = "[example](https://example.com)";
//         let expected = vec![MarkdownElement::Link {
//             text: "example",
//             url: "https://example.com",
//         }];
//         assert_eq!(parse_markdown(input).unwrap(), expected);
//     }

//     #[test]
//     fn test_reference_definition() {
//         let input = "[myref]: https://example.org";
//         let expected = vec![MarkdownElement::ReferenceDefinition {
//             label: "myref",
//             url: "https://example.org",
//         }];
//         assert_eq!(parse_markdown(input).unwrap(), expected);
//     }

//     #[test]
//     fn test_mixed() {
//         let input = "[example](https://example.com) [myref]: https://example.org some text";
//         let expected = vec![
//             MarkdownElement::Link {
//                 text: "example",
//                 url: "https://example.com",
//             },
//             MarkdownElement::ReferenceDefinition {
//                 label: "myref",
//                 url: "https://example.org",
//             },
//             MarkdownElement::Text(" some text"),
//         ];
//         assert_eq!(parse_markdown(input).unwrap(), expected);
//     }

//     #[test]
//     fn test_text_only() {
//         let input = "some text";
//         let expected = vec![MarkdownElement::Text("some text")];
//         assert_eq!(parse_markdown(input).unwrap(), expected);
//     }

//     #[test]
//     fn test_multiple_links() {
//         let input = "[example](https://example.com) and [another](https://another.com)";
//         let expected = vec![
//             MarkdownElement::Link {
//                 text: "example",
//                 url: "https://example.com",
//             },
//             MarkdownElement::Text(" and "),
//             MarkdownElement::Link {
//                 text: "another",
//                 url: "https://another.com",
//             },
//         ];
//         assert_eq!(parse_markdown(input).unwrap(), expected);
//     }
// }
