#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to read and write XML data using the
//! `xml-rs` crate.
use xml::reader::EventReader;
use xml::reader::XmlEvent;
use xml::writer::EmitterConfig;
use xml::writer::XmlEvent as WriteEvent;

fn main() -> anyhow::Result<()> {
    // Example 1: Reading XML.
    let xml_input = r#"<?xml version="1.0"?>
        <library>
            <book id="1">
                <title>Rust Programming</title>
                <author>John Doe</author>
                <year>2023</year>
            </book>
            <book id="2">
                <title>XML Processing</title>
                <author>Jane Smith</author>
                <year>2022</year>
            </book>
        </library>
    "#;

    println!("Example 1: Reading XML");
    let parser = EventReader::from_str(xml_input);

    for event in parser {
        match event {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                print!("Start: {name}");
                if !attributes.is_empty() {
                    print!(" with attributes:");
                    for attr in &attributes {
                        print!(" {}=\"{}\"", attr.name, attr.value);
                    }
                }
                println!();
            }
            Ok(XmlEvent::EndElement { name }) => {
                println!("End: {name}");
            }
            Ok(XmlEvent::Characters(text)) if !text.trim().is_empty() => {
                println!("  Text: {text}");
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }

    // Example 2: Writing XML.
    println!("\nExample 2: Writing XML");
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(Vec::new());

    // Root element:
    writer.write(WriteEvent::start_element("catalog"))?;

    // First product (builder-style with attribute):
    writer.write(WriteEvent::start_element("product").attr("id", "123"))?;
    writer.write(WriteEvent::start_element("name"))?;
    writer.write(WriteEvent::characters("Rust Book"))?;
    writer.write(WriteEvent::end_element())?;
    writer.write(WriteEvent::start_element("price").attr("currency", "USD"))?;
    writer.write(WriteEvent::characters("29.99"))?;
    writer.write(WriteEvent::end_element())?;
    writer.write(WriteEvent::end_element())?;

    // Second product:
    writer.write(WriteEvent::start_element("product").attr("id", "456"))?;
    writer.write(WriteEvent::start_element("name"))?;
    writer.write(WriteEvent::characters("XML Guide"))?;
    writer.write(WriteEvent::end_element())?;
    writer.write(WriteEvent::start_element("price").attr("currency", "EUR"))?;
    writer.write(WriteEvent::characters("24.99"))?;
    writer.write(WriteEvent::end_element())?;
    writer.write(WriteEvent::end_element())?;

    // Close root element:
    writer.write(WriteEvent::end_element())?;

    let result = writer.into_inner();
    println!("{}", String::from_utf8(result)?);

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
        main()?;
        Ok(())
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/1100)
