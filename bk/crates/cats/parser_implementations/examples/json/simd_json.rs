// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of using the `simd-json` crate for parsing and manipulating JSON
//! data.
//!
//! `simd-json` is a Rust port of the extremely fast simdjson c++ library with
//! `serde` compatibility. For best performance, use `mimalloc` or `jemalloc`
//! instead of the system allocator used by default.
//!
//! Example of using `mimalloc`:
//! ```rust
//! use mimalloc::MiMalloc;
//!
//! #[global_allocator]
//! static GLOBAL: MiMalloc = MiMalloc;
//! ```

use simd_json::OwnedValue;
use simd_json::derived::ValueObjectAccess as _;
use simd_json::derived::ValueObjectAccessAsArray as _;
use simd_json::derived::ValueObjectAccessAsScalar as _;
use simd_json::prelude::*;

/// Parses a JSON string with `simd-json` and reads its values.
fn main() {
    let mut json_string = br#"
    {
        "name": "Alice",
        "age": 30,
        "city": "New York",
        "numbers": [1, 2, 3, 4, 5],
        "nested": {
            "value": true
        }
    }
    "#
    .to_vec();

    // Parse the JSON string into a `BorrowedValue`.
    // `BorrowedValue` is designed for read-only access and avoids unnecessary
    // copying, making it very fast.
    match simd_json::to_borrowed_value(&mut json_string) {
        Ok(json) => {
            println!("Parsed JSON: {json:?}");

            // Access values using `ValueAccess` traits.
            if let Some(name) = json.get_str("name") {
                println!("Name: {name}");
            }

            if let Some(age) = json.get_u64("age") {
                println!("Age: {age}");
            }

            if let Some(city) = json.get_str("city") {
                println!("City: {city}");
            }

            if let Some(numbers) = json.get_array("numbers") {
                println!("Numbers: {numbers:?}");
            }

            if let Some(nested) = json.get("nested")
                && let Some(nested_value) = nested.get_bool("value")
            {
                println!("Nested Value: {nested_value}");
            }
        }
        Err(e) => {
            eprintln!("Error parsing JSON: {e}");
        }
    }

    // Example using `OwnedValue` (for modifications).
    // This is slower then the `BorrowedValue` as a tradeoff for getting rid of
    // lifetimes.
    let mut owned_json: OwnedValue =
        simd_json::to_owned_value(&mut json_string)
            .expect("the simd json example invariant should hold");

    if let Some(name) = owned_json.get_mut("name") {
        *name = OwnedValue::from("Bob");
    }

    println!("Modified Owned JSON: {owned_json}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the simd json example through its entry point.
    #[test]
    fn test_simd_json_example_entry_point() {
        main();
    }

    /// Typed accessors read the parsed values.
    #[test]
    fn reads_values() {
        let mut data =
            br#"{"name": "Alice", "age": 30, "nested": {"value": true}}"#
                .to_vec();
        let json = simd_json::to_borrowed_value(&mut data).expect("valid JSON");
        assert_eq!(json.get_str("name"), Some("Alice"));
        assert_eq!(json.get_u64("age"), Some(30));
        assert_eq!(
            json.get("nested").and_then(|n| n.get_bool("value")),
            Some(true)
        );
        assert_eq!(json.get_str("missing"), None);
    }

    /// The owned value can be modified.
    #[test]
    fn owned_value_can_be_modified() -> Result<(), Box<dyn std::error::Error>> {
        let mut data = br#"{"name": "Alice"}"#.to_vec();
        let mut owned = simd_json::to_owned_value(&mut data)?;
        *owned.get_mut("name").ok_or("missing value")? =
            OwnedValue::from("Bob");
        assert_eq!(owned.get_str("name"), Some("Bob"));
        Ok(())
    }

    /// Invalid JSON is an error.
    #[test]
    fn invalid_json_is_rejected() {
        let mut data = b"{\"name\": ".to_vec();
        assert!(simd_json::to_borrowed_value(&mut data).is_err());
    }
}
