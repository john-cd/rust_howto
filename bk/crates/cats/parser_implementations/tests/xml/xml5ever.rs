// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! The following demonstrates how to use the `xml5ever` crate for parsing
// //! XML documents. It provides two main examples:
// //!
// //! 1.  **TokenSink Example**: Shows how to use `TokenSink` to process XML
// //! tokens as they are parsed.
// //! 2.  **TreeSink Example**: Demonstrates how to
// //! build a custom tree structure from an XML document using `TreeSink`. //!
// //!
// //! Add to your `Cargo.toml` file:
// //! ```
// //! [dependencies]
// //! xml5ever = "0.21.0"
// //! markup5ever = "0.15.0" # Or latest
// //! ```
// use std::default::Default;

// use markup5ever::interface::QualName;
// use markup5ever::namespace_url;
// use markup5ever::tendril::TendrilSink;
// use xml5ever::driver::ParseSetting;
// use xml5ever::driver::Parser;
// use xml5ever::driver;
// use xml5ever::tokenizer::TagKind;
// use xml5ever::tokenizer::Token;
// use xml5ever::tokenizer::TokenSink;
// use xml5ever::tokenizer::XmlTokenizer;
// use xml5ever::tokenizer::XmlTokenizerOpts;
// use xml5ever::tree_builder::Attribute;
// use xml5ever::tree_builder::ElementFlags;
// use xml5ever::tree_builder::NodeOrText;
// use xml5ever::tree_builder::TreeSink;

// /// A `TokenSink` implementation that prints XML tokens to the console.

// struct DOMPrinter {
//     indent: usize,
// }

// impl DOMPrinter {
//     fn new() -> Self {
//         DOMPrinter { indent: 0 }
//     }

//     fn print_indent(&self) {
//         for _ in 0..self.indent {
//             print!("  ");
//         }
//     }
// }

// /// Implements the `TokenSink` trait to process XML tokens.
// impl TokenSink for DOMPrinter {
//     fn process_token(&mut self, token: Token) {
//         match token {
//             Token::TagToken(tag) => match tag.kind {
//                 TagKind::StartTag => {
//                     self.print_indent();
//                     print!("<{}", tag.name.local);

//                     for attr in tag.attrs {
//                         print!(" {}=\"{}\"", attr.name.local, attr.value);
//                     }

//                     if tag.self_closing {
//                         println!(" />");
//                     } else {
//                         println!(">");
//                         self.indent += 1;
//                     }
//                 }
//                 TagKind::EndTag => {
//                     self.indent -= 1;
//                     self.print_indent();
//                     println!("</{}>", tag.name.local);
//                 }
//                 _ => {}
//             },
//             Token::CharacterTokens(text) => {
//                 let text = text.trim();
//                 if !text.is_empty() {
//                     self.print_indent();
//                     println!("\"{}\"", text);
//                 }
//             }
//             Token::DoctypeToken(doctype) => {
//                 println!("<!DOCTYPE {}>", doctype.name);
//             }
//             Token::CommentToken(text) => {
//                 self.print_indent();
//                 println!("<!-- {} -->", text);
//             }
//             Token::PIToken(pi) => {
//                 self.print_indent();
//                 println!("<?{} {}>", pi.target, pi.data);
//             }
//             _ => {}
//         }
//     }
// }

// /// Represents a node in the custom tree structure.
// #[derive(Debug)]
// struct Node {
//     name: Option<QualName>,
//     attrs: Vec<Attribute>,
//     children: Vec<Node>,
//     text: Option<String>,
// }

// /// A `TreeSink` implementation that builds a custom tree structure from XML.
// struct TreeBuilder {
//     root: Option<Node>,
//     stack: Vec<Node>,
// }

// impl TreeBuilder {
//     fn new() -> Self {
//         TreeBuilder {
//             root: None,
//             stack: Vec::new(),
//         }
//     }

//     /// Retrieves the root node of the constructed tree.
//     fn get_tree(mut self) -> Option<Node> {
//         if self.stack.len() > 0 {
//             self.root = Some(self.stack.remove(0));
//         }
//         self.root
//     }
// }

// /// Implements the `TreeSink` trait to build a custom tree structure.
// impl TreeSink for TreeBuilder {
//     type Handle = usize;
//     type Output = Self;

//     fn finish(self) -> Self {
//         self
//     }

//     fn parse_error(&mut self, _: std::str::Chars) {}

//     fn get_document(&mut self) -> usize {
//         0
//     }

//     fn elem_name<'a>(&'a self, &id: &Self::Handle) -> QualName {
//         self.stack[id].name.clone().unwrap()
//     }

//     fn create_element(
//         &mut self,
//         name: QualName,
//         attrs: Vec<Attribute>,
//         _: ElementFlags,
//     ) -> usize {
//         let node = Node {
//             name: Some(name),
//             attrs,
//             children: Vec::new(),
//             text: None,
//         };
//         self.stack.push(node);
//         self.stack.len() - 1
//     }

//     fn create_comment(&mut self, _text: markup::tendril::StrTendril) -> usize
// {         0 // Simplified
//     }

//     fn create_pi(
//         &mut self,
//         _target: markup::tendril::StrTendril,
//         _data: markup::tendril::StrTendril,
//     ) -> usize {
//         0 // Simplified
//     }

//     fn append(
//         &mut self,
//         parent: &Self::Handle,
//         child: NodeOrText<Self::Handle>,
//     ) {
//         match child {
//             NodeOrText::AppendNode(id) => {
//                 let child_node = self.stack.remove(id);
//                 self.stack[*parent].children.push(child_node);
//             }
//             NodeOrText::AppendText(text) => {
//                 let node = Node {
//                     name: None,
//                     attrs: Vec::new(),
//                     children: Vec::new(),
//                     text: Some(text.to_string()),
//                 };
//                 self.stack[*parent].children.push(node);
//             }
//         }
//     }

//     fn append_before_sibling(
//         &mut self,
//         _sibling: &Self::Handle,
//         _new_node: NodeOrText<Self::Handle>,
//     ) -> Result<(), ()> {
//         Ok(()) // Simplified
//     }

//     fn append_doctype_to_document(
//         &mut self,
//         _name: markup::tendril::StrTendril,
//         _public_id: markup::tendril::StrTendril,
//         _system_id: markup::tendril::StrTendril,
//     ) {
//         // Simplified
//     }

//     fn add_attrs_if_missing(
//         &mut self,
//         _handle: &Self::Handle,
//         _attrs: Vec<Attribute>,
//     ) {
//         // Simplified.
//     }

//     fn remove_from_parent(&mut self, _target: &Self::Handle) {
//         // Simplified.
//     }

//     fn reparent_children(
//         &mut self,
//         _node: &Self::Handle,
//         _new_parent: &Self::Handle,
//     ) {
//         // Simplified.
//     }

//     fn mark_script_already_started(&mut self, _target: &Self::Handle) {
//         // Simplified.
//     }

//     fn set_quirks_mode(&mut self, _mode: markup5ever::interface::QuirksMode)
// {         // Simplified.
//     }
// }

// /// Recursively prints the tree structure to the console.
// fn print_node(node: &Node, depth: usize) {
//     let indent = "  ".repeat(depth);

//     if let Some(name) = &node.name {
//         print!("{}Element: {}", indent, name.local);

//         if !node.attrs.is_empty() {
//             print!(" [");
//             for (i, attr) in node.attrs.iter().enumerate() {
//                 if i > 0 {
//                     print!(", ");
//                 }
//                 print!("{}=\"{}\"", attr.name.local, attr.value);
//             }
//             print!("]");
//         }
//         println!();
//     } else if let Some(text) = &node.text {
//         let trimmed = text.trim();
//         if !trimmed.is_empty() {
//             println!("{}Text: \"{}\"", indent, trimmed);
//         }
//     }

//     for child in &node.children {
//         print_node(child, depth + 1);
//     }
// }

// fn main() {
//     let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
// <library xmlns="http://example.org/lib">
//   <!-- Digital library collection -->
//   <book id="b1" available="true">
//     <title>Rust Programming</title>
//     <author>John Doe</author>
//     <published year="2023" />
//     <description>A comprehensive guide to Rust</description>
//   </book>
//   <book id="b2" available="false">
//     <title>XML Processing with Rust</title>
//     <author>Jane Smith</author>
//     <published year="2022" />
//     <description>Learn about XML processing in Rust</description>
//   </book>
// </library>"#;

//     println!("Example 1: Using xml5ever TokenSink\n");

//     let sink = DOMPrinter::new();
//     let mut tok = XmlTokenizer::new(sink, XmlTokenizerOpts::default());
//     tok.feed(xml.into());
//     tok.end();

//     println!("\nExample 2: Using xml5ever TreeSink\n");

//     let tb = TreeBuilder::new();
//     let parser = Parser::new(tb);
//     let dom = parser.one(xml);

//     if let Some(tree) = dom.get_tree() {
//         print_node(&tree, 0);
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish NOW](https://github.com/john-cd/rust_howto/issues/1103)
