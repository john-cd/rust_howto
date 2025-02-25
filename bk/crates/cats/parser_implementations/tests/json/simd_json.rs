// ANCHOR: example

use simd_json::OwnedValue;
use simd_json::derived::ValueObjectAccess;
use simd_json::derived::ValueObjectAccessAsArray;
use simd_json::derived::ValueObjectAccessAsScalar;
use simd_json::prelude::*;

// `simd-json` is a Rust port of the extremely fast simdjson c++ library with
// `serde` compatibility. For best performance, use `mimalloc` or `jemalloc`
// instead of the system allocator used by default. For example:
// use mimalloc::MiMalloc;
//
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

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

    // Parse the JSON string into a BorrowedValue.
    // BorrowedValue is designed for read-only access and avoids unnecessary
    // copying, making it very fast.
    match simd_json::to_borrowed_value(&mut json_string) {
        Ok(json) => {
            println!("Parsed JSON: {:?}", json);

            // Access values using ValueAccess traits
            if let Some(name) = json.get_str("name") {
                println!("Name: {}", name);
            }

            if let Some(age) = json.get_u64("age") {
                println!("Age: {}", age);
            }

            if let Some(city) = json.get_str("city") {
                println!("City: {}", city);
            }

            if let Some(numbers) = json.get_array("numbers") {
                println!("Numbers: {:?}", numbers);
            }

            if let Some(nested) = json.get("nested") {
                if let Some(nested_value) = nested.get_bool("value") {
                    println!("Nested Value: {}", nested_value);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
        }
    }

    // Example using OwnedValue (for modifications)
    // This is slower then the BorrowedValue as a tradeoff for getting rid of
    // lifetimes.
    let mut owned_json: OwnedValue =
        simd_json::to_owned_value(&mut json_string).unwrap();

    if let Some(name) = owned_json.get_mut("name") {
        *name = OwnedValue::from("Bob");
    }

    println!("Modified Owned JSON: {}", owned_json.to_string());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
