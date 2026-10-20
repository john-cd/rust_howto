#![allow(dead_code)]
// ANCHOR: example
//! Parse, deserialize, and write XML with `quick-xml`.

use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::de::from_str;
use quick_xml::events::BytesEnd;
use quick_xml::events::BytesStart;
use quick_xml::events::BytesText;
use quick_xml::events::Event;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename = "person")]
struct Person {
    name: String,
    age: u8,
}

fn main() -> anyhow::Result<()> {
    let xml = r#"<person><name>John Doe</name><age>32</age></person>"#;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    loop {
        match reader.read_event()? {
            Event::Start(event) => {
                println!(
                    "start: {}",
                    String::from_utf8_lossy(event.name().as_ref())
                );
            }
            Event::Text(event) => {
                println!("text: {}", event.decode()?.into_owned());
            }
            Event::End(event) => {
                println!(
                    "end: {}",
                    String::from_utf8_lossy(event.name().as_ref())
                );
            }
            Event::Eof => break,
            _ => {}
        }
    }

    let person: Person = from_str(xml)?;
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
    println!("{}", String::from_utf8(output)?);
    Ok(())
}
// ANCHOR_END: example

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
    // [finish](https://github.com/john-cd/rust_howto/issues/1236)
}
