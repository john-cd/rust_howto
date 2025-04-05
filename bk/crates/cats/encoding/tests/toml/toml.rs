// ANCHOR: example
//! The following demonstrates TOML decoding.
//!
//! TOML is a minimal configuration file format.
//! It is designed to map unambiguously to a hash table.

// Representation of a TOML value e.g. String,
// Integer, Float, Array, Table...
use toml::Value;
use toml::de::Error;

fn main() -> Result<(), Error> {
    // Sample TOML.
    // Note the use of a Rust raw string,
    // so that there is no need to escape the inner double quotes.
    let toml_content = r#"
          # This is a TOML comment

          key = "value"
          "quoted key" = "possible but rare"

          # A table a.k.a. hash tables or dictionaries:
          [table]
          key = 123

          [strings]
          str1 = "basic string with \"escaping\" \n"
          str2 = 'literal string. No escaping is performed'
          str3 = """multi-line \
          basic string"""
          str4 = '''multi-line literal strings allow a single quote ' inside'''

          [numbers]
          int1 = 42
          hex1 = 0xdeadbeef # Or octal or binary
          float1 = 6.626e-34

          [dates]
          dt1 = 2025-02-09T00:00:00-09:00
          # Dates, times, and datetimes with and without offsets are possible

          [arrays]
          basic = ["a", "b"]
          nested_arrays_of_ints = [ [ 1, 2 ], [3, 4, 5] ]
          numbers = [ 0.1, 0.2, 0.5, 1, 2, 5 ]  # Mixed-type arrays are allowed

          # Dotted key notation
          table2.key = 3.14
          # Equivalent to
          #[table2]
          #key = 3.14

          [a.b]
          c = "1.0"
          # Equivalent to a.b.c = "1.0"

          inline_table = { x = 1, y = 2 }

          [[array-of-tables]]
          name = "John"

          [[array-of-tables]]  # Empty table within the array

          [[arrays-of-tables]]
          name = "Blake"
          "#;

    let toml: Value = toml::from_str(toml_content)?;

    assert_eq!(toml["table"]["key"].as_integer(), Some(123));
    println!("{}", toml["strings"]["str1"].as_str().unwrap());
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
