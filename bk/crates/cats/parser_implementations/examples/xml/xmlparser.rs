#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to use the `xmlparser` crate to parse an
// //! XML string. It showcases handling XML declarations, comments, elements,
// //! attributes, and text content.
// use xmlparser::Token;
// use xmlparser::Tokenizer;

// fn main() {
//     // The example XML string represents a simple library with books.
//     let xml = r#"<?xml version="1.0"?>
// <library>
//     <!-- Our book collection -->
//     <book id="b1" available="true">
//         <title>Rust Programming</title>
//         <author>John Doe</author>
//     </book>
//     <book id="b2" available="false">
//         <title>XML Processing</title>
//         <author>Jane Smith</author>
//     </book>
// </library>"#;

//     let mut depth = 0;

//     for token in Tokenizer::from(xml) {
//         match token {
//             Ok(Token::Declaration {
//                 version,
//                 encoding,
//                 standalone,
//                 span: _,
//             }) => {
//                 println!(
//                     "XML Declaration: version={} encoding={:?}
// standalone={:?}",                     version, encoding, standalone
//                 );
//             }
//             Ok(Token::Comment { text, span: _ }) => {
//                 println!("{" ".repeat(depth * 2}Comment: {}"), text);
//             }
//             Ok(Token::ElementStart {
//                 prefix,
//                 local,
//                 span: _,
//             }) => {
//                 print!("{" ".repeat(depth * 2}Element Start: "));
//                 if let Some(prefix) = prefix {
//                     print!("{prefix}:");
//                 }
//                 println!("{local}");
//                 depth += 1;
//             }
//             Ok(Token::Attribute {
//                 prefix,
//                 local,
//                 value,
//                 span: _,
//             }) => {
//                 print!("{" ".repeat(depth * 2} Attribute: "));
//                 if let Some(prefix) = prefix {
//                     print!("{prefix}:");
//                 }
//                 println!("{local, value}=\"{}\"");
//             }
//             Ok(Token::ElementEnd { end, span: _ }) => match end {
//                 xmlparser::ElementEnd::Close(prefix, local) => {
//                     depth -= 1;
//                     print!("{" ".repeat(depth * 2}Element End: "));
//                     if let Some(prefix) = prefix {
//                         print!("{prefix}:");
//                     }
//                     println!("{local}");
//                 }
//                 xmlparser::ElementEnd::Empty => {
//                     depth -= 1;
//                     println!("{" ".repeat(depth * 2} (Empty)"));
//                 }
//                 xmlparser::ElementEnd::Open => {}
//             },
//             Ok(Token::Text { text, span: _ }) => {
//                 let trimmed = text.trim();
//                 if !trimmed.is_empty() {
//                     println!("{" ".repeat(depth * 2}Text: {}"), trimmed);
//                 }
//             }
//             Err(e) => {
//                 println!("Error: {e:?}");
//                 break;
//             }
//             _ => {}
//         }
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/1102)
