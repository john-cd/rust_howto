#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! # `html5ever` example
// //!
// //! `html5ever` is an HTML parser developed as part of the Servo project.
// //! It can parse and serialize HTML according to the WHATWG specs (aka
// //! "HTML5"). `html5ever`` uses callbacks to manipulate the DOM, therefore it
// //! does not provide any DOM tree representation itself.
// //! It exclusively uses UTF-8 to represent strings.
// use std::default::Default;

// // Options for the parser
// use html5ever::driver::ParseOpts;
// use html5ever::parse_document;
// use html5ever::tendril::TendrilSink;
// use html5ever::tree_builder::TreeBuilder;
// use html5ever::tree_builder::TreeBuilderOpts;
// use html5ever::tree_builder::TreeSink;
// // A simple thread-safe reference-counted DOM.
// // See <https://en.wikipedia.org/wiki/Document_Object_Model>
// use markup5ever_arcdom::ArcDom;
// use markup5ever_arcdom::Handle;

// fn main() {
//     let html = "<html><head><title>Example</title></head><body><p>Hello,
// world!</p></body></html>";

//     // Parse the HTML document.
//     let parser: html5ever::driver::Parser<_> =
//         parse_document(ArcDom::default(), ParseOpts::default());
//     let dom = parser.read_from(&mut html.as_bytes()).unwrap();

//     let document = dom.document;
//     walk(document);
// }

// fn walk(handle: Handle) {
//     let node = handle;
//     match node.data {
//         markup5ever_arcdom::NodeData::Document => {
//             for child in node.children.borrow().iter() {
//                 walk(child.clone());
//             }
//         }
//         markup5ever_arcdom::NodeData::Element { ref name, .. } => {
//             println!("Element: {:?}", name.local);
//             for child in node.children.borrow().iter() {
//                 walk(child.clone());
//             }
//         }
//         markup5ever_arcdom::NodeData::Text { ref contents } => {
//             println!("Text: {:?}", contents.borrow());
//         }
//         _ => {}
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish NOW](https://github.com/john-cd/rust_howto/issues/1090)
// // Look at
// // <https://github.com/servo/html5ever/blob/main/rcdom/examples/html2html.rs>
// // <https://github.com/servo/html5ever/blob/main/rcdom/examples/print-rcdom.rs>
