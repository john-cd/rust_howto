#![allow(dead_code)]
// ANCHOR: example
//! The `Default` trait provides a way to create a default value for a type.
//! It initializes a struct without manually specifying every field.

// `derive` implements it automatically:
#[derive(Default, Debug)]
struct Config {
    timeout: u32,          // defaults to `0`.
    retries: u8,           // defaults to `0`.
    endpoint: String,      // defaults to `"".to_string()`.
    logging_enabled: bool, // defaults to `false`.
}

// If you want custom defaults, you can implement the trait manually:
#[derive(Debug)]
struct Config2 {
    timeout: u32,
    retries: u8,
    endpoint: String,
    logging_enabled: bool,
}

impl Default for Config2 {
    fn default() -> Self {
        Config2 {
            timeout: 5000,
            retries: 3,
            endpoint: "https://api.example.com".to_string(),
            logging_enabled: true,
        }
    }
}

fn main() {
    let config = Config::default();
    println!("{config:?}");

    let config2 = Config2::default();
    println!("{config2:?}");

    // You can also use struct update syntax to override specific fields:
    let _custom_config = Config {
        timeout: 10000,
        ..Config::default()
    };
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
