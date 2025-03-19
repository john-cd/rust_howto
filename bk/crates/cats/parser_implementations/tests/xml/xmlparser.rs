// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use xmlparser::{Token, Tokenizer};

// fn main() {
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
//             Ok(Token::Declaration { version, encoding, standalone, span: _ }) => {
//                 println!("XML Declaration: version={} encoding={:?} standalone={:?}",
//                     version, encoding, standalone);
//             }
//             Ok(Token::Comment { text, span: _ }) => {
//                 println!("{}Comment: {}", " ".repeat(depth * 2), text);
//             }
//             Ok(Token::ElementStart { prefix, local, span: _ }) => {
//                 print!("{}Element Start: ", " ".repeat(depth * 2));
//                 if let Some(prefix) = prefix {
//                     print!("{}:", prefix);
//                 }
//                 println!("{}", local);
//                 depth += 1;
//             }
//             Ok(Token::Attribute { prefix, local, value, span: _ }) => {
//                 print!("{} Attribute: ", " ".repeat(depth * 2));
//                 if let Some(prefix) = prefix {
//                     print!("{}:", prefix);
//                 }
//                 println!("{}=\"{}\"", local, value);
//             }
//             Ok(Token::ElementEnd { end, span: _ }) => {
//                 match end {
//                     xmlparser::ElementEnd::Close(prefix, local) => {
//                         depth -= 1;
//                         print!("{}Element End: ", " ".repeat(depth * 2));
//                         if let Some(prefix) = prefix {
//                             print!("{}:", prefix);
//                         }
//                         println!("{}", local);
//                     }
//                     xmlparser::ElementEnd::Empty => {
//                         depth -= 1;
//                         println!("{} (Empty)", " ".repeat(depth * 2));
//                     }
//                     xmlparser::ElementEnd::Open => {}
//                 }
//             }
//             Ok(Token::Text { text, span: _ }) => {
//                 let trimmed = text.trim();
//                 if !trimmed.is_empty() {
//                     println!("{}Text: {}", " ".repeat(depth * 2), trimmed);
//                 }
//             }
//             Err(e) => {
//                 println!("Error: {:?}", e);
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
// // [add example](https://github.com/john-cd/rust_howto/issues/1102)
