// ANCHOR: example
//! This example demonstrates how to use the `ini` crate to parse and manipulate
//! INI files.
//!
//! INI is an informal standard for configuration files.
//! INI files are simple text files with a basic structure composed of
//! "sections" and "properties".

use ini::Ini;

/// Reads an INI configuration from a string, manipulates it, and writes it to a
/// file.
fn main() -> anyhow::Result<()> {
    let config_str = r#"
[section1]
key1 = value1
key2 = value2

[section2]
key3 = value3
key4 = value4
"#;
    // Read an INI configuration.
    let ini = Ini::load_from_str(config_str)?;
    // Or use: `Ini::load_from_file("conf.ini")`.

    // Get the first value for the given key in the given section.
    let value1 = ini.get_from(Some("section1"), "key1");
    let value3 = ini.get_from(Some("section2"), "key3");

    println!("Value for key1: {:?}", value1);
    println!("Value for key3: {:?}", value3);

    // Or get a section, then retrieve the key.
    let section = ini.section(Some("section2")).unwrap();
    let _value4 = section.get("key4").unwrap();

    // Or iterate through key value pairs.
    for (sec, prop) in &ini {
        println!("Section: {:?}", sec);
        for (key, value) in prop.iter() {
            println!("{:?}:{:?}", key, value);
        }
    }

    // Write an .ini configuration file.
    let mut conf = Ini::new();
    conf.with_section(Some("Section3"))
        .set("key5", "value5")
        .set("key6", "value6");
    conf.write_to_file("temp/conf.ini").unwrap();

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
