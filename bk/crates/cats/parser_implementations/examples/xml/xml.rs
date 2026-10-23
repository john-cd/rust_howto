#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to read and write XML data using the
//! `xml-rs` crate.

use std::fmt::Write as _;

use anyhow::Context as _;
use xml::reader::EventReader;
use xml::reader::XmlEvent;
use xml::writer::EmitterConfig;
use xml::writer::XmlEvent as WriteEvent;

/// Reads XML events from a string, then writes an XML document.
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
    for event in read_xml(xml_input)? {
        println!("{event}");
    }

    // Example 2: Writing XML.
    println!("\nExample 2: Writing XML");
    println!("{}", write_catalog()?);

    Ok(())
}

/// Parses XML and returns a readable description of its events.
fn read_xml(xml_input: &str) -> anyhow::Result<Vec<String>> {
    let mut events = Vec::new();

    for event in EventReader::from_str(xml_input) {
        match event.context("failed to parse XML event")? {
            XmlEvent::StartElement {
                name, attributes, ..
            } => {
                let mut event = format!("Start: {name}");
                if !attributes.is_empty() {
                    event.push_str(" with attributes:");
                    for attr in &attributes {
                        write!(event, " {}=\"{}\"", attr.name, attr.value)
                            .context("failed to format XML attribute")?;
                    }
                }
                events.push(event);
            }
            XmlEvent::EndElement { name } => {
                events.push(format!("End: {name}"));
            }
            XmlEvent::Characters(text) if !text.trim().is_empty() => {
                events.push(format!("  Text: {text}"));
            }
            _ => {}
        }
    }

    Ok(events)
}

/// Writes a catalog with two products and returns the serialized XML.
fn write_catalog() -> anyhow::Result<String> {
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

    String::from_utf8(writer.into_inner())
        .context("XML writer produced invalid UTF-8")
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Preserves element names, attributes, and text while parsing valid XML.
    #[test]
    fn read_xml_reports_elements_attributes_and_text() -> anyhow::Result<()> {
        let events = read_xml(
            r#"<library><book id="7"><title>Rust</title></book></library>"#,
        )?;

        assert!(events.iter().any(|event| event == "Start: library"));
        assert!(
            events
                .iter()
                .any(|event| event == "Start: book with attributes: id=\"7\"")
        );
        assert!(events.iter().any(|event| event == "  Text: Rust"));

        Ok(())
    }

    /// Returns a parse error instead of reporting malformed XML as success.
    #[test]
    fn read_xml_rejects_malformed_documents() {
        assert!(read_xml("<library><book></library>").is_err());
    }

    /// Writes catalog products that can be read back by the XML event parser.
    #[test]
    fn write_catalog_contains_both_products() -> anyhow::Result<()> {
        let catalog = write_catalog()?;
        let events = read_xml(&catalog)?;

        assert!(
            events
                .iter()
                .any(|event| event
                    == "Start: product with attributes: id=\"123\"")
        );
        assert!(events.iter().any(|event| event == "  Text: Rust Book"));
        assert!(
            events
                .iter()
                .any(|event| event
                    == "Start: product with attributes: id=\"456\"")
        );
        assert!(events.iter().any(|event| event == "  Text: XML Guide"));

        Ok(())
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/1100)
