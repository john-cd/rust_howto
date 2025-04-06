// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to use the `quick-xml` crate for parsing
// //! and generating XML data in Rust. It covers low-level parsing,
// //! deserialization with `serde`, creating XML with the writer API, and
// //! serialization with `serde`.
// //! See [quick-xml](https://docs.rs/quick-xml/latest/quick_xml).
// use std::io::Cursor;

// use quick_xml::Reader;
// use quick_xml::Writer;
// use quick_xml::de::from_str;
// use quick_xml::events::BytesStart;
// use quick_xml::events::Event;
// use serde::Deserialize;
// use serde::Serialize;

// // Define structs for serialization/deserialization
// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// struct Person {
//     name: String,
//     age: u8,
//     #[serde(rename = "hobbies")]
//     hobby_list: Hobbies,
// }

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// struct Hobbies {
//     #[serde(rename = "hobby")]
//     entries: Vec<String>,
// }

// fn main() -> anyhow::Result<()> {
//     // Example 1: Parse XML using the Reader API.
//     println!("Example 1: Low-level parsing");
//     let xml = r#"
//         <person>
//             <name>John Doe</name>
//             <age>32</age>
//             <hobbies>
//                 <hobby>Reading</hobby>
//                 <hobby>Hiking</hobby>
//                 <hobby>Programming</hobby>
//             </hobbies>
//         </person>
//     "#;

//     let mut reader = Reader::from_str(xml);
//     reader.trim_text(true);

//     let mut buf = Vec::new();
//     let mut depth = 0;
//     let mut txt = Vec::new();

//     // Manual parsing loop.
//     loop {
//         match reader.read_event_into(&mut buf) {
//             Ok(Event::Start(ref e)) => {
//                 println!(
//                     "{:indent$}Start: {}",
//                     "",
//                     e.name().as_ref(),
//                     indent = depth * 2
//                 );
//                 depth += 1;
//             }
//             Ok(Event::End(ref e)) => {
//                 depth -= 1;
//                 println!(
//                     "{:indent$}End: {}",
//                     "",
//                     e.name().as_ref(),
//                     indent = depth * 2
//                 );
//             }
//             Ok(Event::Text(e)) => {
//                 txt = e.unescape()?.into_owned();
//                 println!(
//                     "{:indent$}Text: {}",
//                     "",
//                     String::from_utf8_lossy(&txt),
//                     indent = depth * 2
//                 );
//             }
//             Ok(Event::Eof) => break,
//             Err(e) => panic!(
//                 "Error at position {}: {:?}",
//                 reader.buffer_position(),
//                 e
//             ),
//             _ => (),
//         }
//         buf.clear();
//     }

//     // Example 2: Using `serde` for deserialization.
//     println!("\nExample 2: Deserializing XML");
//     let person: Person = from_str(xml)?;
//     println!("Deserialized: {:#?}", person);

//     // Example 3: Creating XML with `Writer`.
//     println!("\nExample 3: Creating XML");
//     let mut writer = Writer::new(Cursor::new(Vec::new()));

//     writer.write_event(Event::Start(BytesStart::new("person")))?;

//     writer
//         .create_element("name")
//         .write_text_content("Jane Smith")?;
//     writer.create_element("age").write_text_content("28")?;

//     writer.write_event(Event::Start(BytesStart::new("hobbies")))?;
//     writer
//         .create_element("hobby")
//         .write_text_content("Swimming")?;
//     writer
//         .create_element("hobby")
//         .write_text_content("Cooking")?;
//     writer
//         .create_element("hobby")
//         .write_text_content("Painting")?;
//     writer.write_event(Event::End(BytesStart::new("hobbies").to_end()))?;

//     writer.write_event(Event::End(BytesStart::new("person").to_end()))?;

//     let result = writer.into_inner().into_inner();
//     println!("{}", String::from_utf8(result)?);

//     // Example 4: Serializing with `serde`.
//     println!("\nExample 4: Serializing to XML");
//     let jane = Person {
//         name: "Jane Smith".to_string(),
//         age: 28,
//         hobby_list: Hobbies {
//             entries: vec![
//                 "Swimming".to_string(),
//                 "Cooking".to_string(),
//                 "Painting".to_string(),
//             ],
//         },
//     };

//     let serialized = quick_xml::se::to_string(&jane)?;
//     println!("{}", serialized);

//     Ok(())
// }

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// // [finish NOW](https://github.com/john-cd/rust_howto/issues/1236)
