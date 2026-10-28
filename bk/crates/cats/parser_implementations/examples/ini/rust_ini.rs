// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use `rust-ini` to parse and manipulate INI
//! files.
//!
//! INI is an informal standard for configuration files.
//! INI files are simple text files with a basic structure composed of
//! "sections" and "properties".

use ini::Ini;

/// Parses INI text, prints selected values, and writes the edited file to
/// `temp/conf.ini`.
fn main() -> anyhow::Result<()> {
    let config_str = "
[section1]
key1 = value1
key2 = value2

[section2]
key3 = value3
key4 = value4
";
    // Read an INI configuration.
    let ini = Ini::load_from_str(config_str)?;
    // Or use: `Ini::load_from_file("conf.ini")`.

    // Get the first value for the given key in the given section.
    let value1 = ini
        .get_from(Some("section1"), "key1")
        .ok_or_else(|| anyhow::anyhow!("missing key1 in section1"))?;
    let value3 = ini
        .get_from(Some("section2"), "key3")
        .ok_or_else(|| anyhow::anyhow!("missing key3 in section2"))?;

    println!("Value for key1: {value1:?}");
    println!("Value for key3: {value3:?}");

    // Or get a section, then retrieve the key.
    let section = ini
        .section(Some("section2"))
        .ok_or_else(|| anyhow::anyhow!("missing section2"))?;
    let _value4 = section
        .get("key4")
        .ok_or_else(|| anyhow::anyhow!("missing key4 in section2"))?;

    // Or iterate through key value pairs.
    for (sec, prop) in &ini {
        println!("Section: {sec:?}");
        for (key, value) in prop {
            println!("{key:?}:{value:?}");
        }
    }

    // Write an .ini configuration file.
    let mut conf = Ini::new();
    conf.with_section(Some("Section3"))
        .set("key5", "value5")
        .set("key6", "value6");
    conf.write_to_file("temp/conf.ini")?;

    Ok(())
}

// ANCHOR_END: example

/// Runs the `rust-ini` example.
pub fn run() -> anyhow::Result<()> {
    main()
}

/// Tests for the `rust-ini` example.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the rust ini example's main path succeeds.
    #[test]
    fn rust_ini_example_parses_and_writes_configuration() -> anyhow::Result<()>
    {
        use std::fs;
        fs::create_dir_all("temp")?;
        main()?;
        Ok(())
    }
}
