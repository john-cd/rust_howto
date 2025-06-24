#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to use the `heck` crate to convert strings
//! between different casing conventions.
//!
//! The `heck` crate provides functions for converting strings to:
//! snake_case, kebab-case, PascalCase, lowerCamelCase, Title Case.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! heck = "0.4.1"
//! ```

use heck::ToKebabCase;
use heck::ToLowerCamelCase;
use heck::ToPascalCase;
use heck::ToSnakeCase;
use heck::ToTitleCase;

fn main() {
    let input = "hello_world example-string";

    println!("Original: {}", input);
    println!("Snake case: {}", input.to_snake_case()); // hello_world_example_string
    println!("Kebab case: {}", input.to_kebab_case()); // hello-world-example-string
    println!("Pascal case: {}", input.to_pascal_case()); // HelloWorldExampleString
    println!("Camel case: {}", input.to_lower_camel_case()); // helloWorldExampleString
    println!("Title case: {}", input.to_title_case()); // Hello World Example String
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
