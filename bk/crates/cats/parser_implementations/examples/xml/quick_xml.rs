#![allow(dead_code)]
// ANCHOR: example
//! Parse, deserialize, and write XML with `quick-xml`.
//!
//! TODO Reviewed ??-??-2026

use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::de::from_str;
use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use quick_xml::events::BytesText;
use quick_xml::events::Event;
use serde::Deserialize;

/// A person, deserialized from XML.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "person")]
struct Person {
    /// The name of the person.
    name: String,
    /// The age of the person.
    age: u8,
}

/// Reads XML events, deserializes a struct from XML, and writes XML.
fn main() -> anyhow::Result<()> {
    let xml = "<person><name>John Doe</name><age>32</age></person>";

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    loop {
        match reader.read_event()? {
            Event::Start(event) => {
                println!("start: {}", event.name().as_ref());
            }
            Event::Text(event) => {
                println!(
                    "text: {}",
                    quick_xml::escape::unescape(event.as_ref())?
                );
            }
            Event::End(event) => {
                println!("end: {}", event.name().as_ref());
            }
            Event::Eof => break,
            _ => {}
        }
    }

    let person: Person = from_str(xml)?;
    assert_eq!(person.name, "John Doe");
    assert_eq!(person.age, 32);
    println!("Deserialized: {} ({})", person.name, person.age);

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.write_event(Event::Start(BytesStart::new("person")))?;
    writer.write_event(Event::Start(BytesStart::new("name")))?;
    writer.write_event(Event::Text(BytesText::new("Jane Smith")))?;
    writer.write_event(Event::End(BytesEnd::new("name")))?;
    writer.write_event(Event::Start(BytesStart::new("age")))?;
    writer.write_event(Event::Text(BytesText::new("28")))?;
    writer.write_event(Event::End(BytesEnd::new("age")))?;
    writer.write_event(Event::End(BytesEnd::new("person")))?;

    let output = writer.into_inner().into_inner();
    let output = String::from_utf8(output)?;
    assert!(output.contains("<name>Jane Smith</name>"));
    assert!(output.contains("<age>28</age>"));
    println!("{output}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }

    /// XML child elements populate the corresponding person fields.
    #[test]
    fn deserializes_person_fields() -> anyhow::Result<()> {
        let person: Person =
            from_str("<person><name>John Doe</name><age>32</age></person>")?;

        assert_eq!(
            person,
            Person {
                name: "John Doe".into(),
                age: 32
            }
        );
        Ok(())
    }
}
