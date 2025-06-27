#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to read and write XML data using the
// //! `xml-rs` crate.
// //! ## Features
// //! - Reading XML from a string and parsing its elements and attributes.
// use std::fs::File;
// use std::io::BufReader;

// use xml::attribute::OwnedAttribute;
// use xml::name::OwnedName;
// use xml::reader::EventReader;
// use xml::reader::XmlEvent;
// use xml::writer::EmitterConfig;
// use xml::writer::EventWriter;
// use xml::writer::XmlEvent as WriteEvent;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Example 1: Reading XML.
//     let xml_input = r#"
//         <library>
//             <book id="1">
//                 <title>Rust Programming</title>
//                 <author>John Doe</author>
//                 <year>2023</year>
//             </book>
//             <book id="2">
//                 <title>XML Processing</title>
//                 <author>Jane Smith</author>
//                 <year>2022</year>
//             </book>
//         </library>
//     "#;

//     println!("Example 1: Reading XML");
//     let parser = EventReader::from_str(xml_input);

//     for event in parser {
//         match event {
//             Ok(XmlEvent::StartElement {
//                 name, attributes, ..
//             }) => {
//                 print!("Start: {name}");
//                 if !attributes.is_empty() {
//                     print!(" with attributes: ");
//                     for attr in attributes {
//                         print!("{attr.name, attr.value}=\"{}\" ");
//                     }
//                 }
//                 println!();
//             }
//             Ok(XmlEvent::EndElement { name }) => {
//                 println!("End: {name}");
//             }
//             Ok(XmlEvent::Characters(text)) => {
//                 if !text.trim().is_empty() {
//                     println!("  Text: {text}");
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Error: {e}");
//                 break;
//             }
//             _ => {}
//         }
//     }

//     // Example 2: Writing XML.
//     println!("\nExample 2: Writing XML");
//     let mut writer = EmitterConfig::new()
//         .perform_indent(true)
//         .create_writer(Vec::new());

//     // Start document:
//     writer.write(WriteEvent::StartDocument {
//         version: xml::common::XmlVersion::Version10,
//         encoding: Some("UTF-8"),
//         standalone: None,
//     })?;

//     // Root element:
//     writer.write(WriteEvent::StartElement {
//         name: OwnedName::local("catalog"),
//         attributes: vec![],
//         namespace: xml::namespace::Namespace::empty(),
//     })?;

//     // First product:
//     writer.write(WriteEvent::StartElement {
//         name: OwnedName::local("product"),
//         attributes: vec![OwnedAttribute {
//             name: OwnedName::local("id"),
//             value: "123".to_string(),
//         }],
//         namespace: xml::namespace::Namespace::empty(),
//     })?;

//     // Name element:
//     writer.write(WriteEvent::StartElement {
//         name: OwnedName::local("name"),
//         attributes: vec![],
//         namespace: xml::namespace::Namespace::empty(),
//     })?;
//     writer.write(WriteEvent::Characters("Rust Book"))?;
//     writer.write(WriteEvent::EndElement {
//         name: OwnedName::local("name"),
//     })?;

//     // Price element:
//     writer.write(WriteEvent::StartElement {
//         name: OwnedName::local("price"),
//         attributes: vec![OwnedAttribute {
//             name: OwnedName::local("currency"),
//             value: "USD".to_string(),
//         }],
//         namespace: xml::namespace::Namespace::empty(),
//     })?;
//     writer.write(WriteEvent::Characters("29.99"))?;
//     writer.write(WriteEvent::EndElement {
//         name: OwnedName::local("price"),
//     })?;

//     // Close first product:
//     writer.write(WriteEvent::EndElement {
//         name: OwnedName::local("product"),
//     })?;

//     // Second product (shorter syntax using helper function):
//     writer.write(WriteEvent::start_element("product").attr("id", "456"))?;
//     writer.write(WriteEvent::start_element("name"))?;
//     writer.write(WriteEvent::Characters("XML Guide"))?;
//     writer.write(WriteEvent::end_element())?;

//     writer.write(WriteEvent::start_element("price").attr("currency",
// "EUR"))?;     writer.write(WriteEvent::Characters("24.99"))?;
//     writer.write(WriteEvent::end_element())?;

//     writer.write(WriteEvent::end_element())?;

//     // Close root element:
//     writer.write(WriteEvent::EndElement {
//         name: OwnedName::local("catalog"),
//     })?;

//     // Get the resulting XML.
//     let result = writer.into_inner();
//     println!("{String::from_utf8(result}")?);

//     Ok(())
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish NOW](https://github.com/john-cd/rust_howto/issues/1100)
