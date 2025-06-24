#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to parse JSON5 data using the `json5` crate.
//!
//! [JSON5](https://json5.org) is a superset of JSON that allows for more human-friendly syntax,
//! such as comments, unquoted keys, and trailing commas.
//! It aims to be easier to write and maintain by hand (e.g. for config files).
//! It is not intended to be used for machine-to-machine communication.
use serde_json::Value;

fn main() {
    let json5_str = r#"{
        // Comments are allowed
        unquoted: 'and you can use single quotes',
        trailingComma: ['in arrays', 'like', 'this', ],
        hexadecimal: 0xDEADbeef,
        infinity: Infinity,
        nan: NaN,
        nested: {
            pi: 3.141592653,
        },
        'quoted key': true, // Quoted keys are allowed
        "backwardsCompatible": "with JSON",
    }"#;

    // Parse the JSON5 string into a `serde_json::Value`.
    match json5::from_str::<Value>(json5_str) {
        Ok(value) => {
            println!("Parsed JSON5: {:#?}", value);

            if let Some(nested) = value.get("nested") {
                if let Some(pi) = nested.get("pi") {
                    println!("Pi: {}", pi);
                }
            }

            if let Some(hex) = value.get("hexadecimal") {
                println!("Hex: {}", hex);
            }

            if let Some(inf) = value.get("infinity") {
                println!("Infinity: {}", inf);
            }

            if let Some(nan) = value.get("nan") {
                println!("NaN: {}", nan);
            }

            if let Some(unquoted) = value.get("unquoted") {
                println!("Unquoted: {}", unquoted);
            }

            if let Some(quoted_key) = value.get("quoted key") {
                println!("Quoted Key: {}", quoted_key);
            }
        }
        Err(e) => {
            eprintln!("Error parsing JSON5: {}", e);
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
