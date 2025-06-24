#![allow(dead_code)]
// ANCHOR: example
use serde::Deserialize;

// Define a Configuration struct that can be deserialized from environment
// variables.
#[derive(Deserialize, Debug)]
struct Configuration {
    port: u16,
}

fn main() {
    // Deserialize the Configuration struct from environment variables.
    // It expects a PORT environment variable to be set.
    let c = envy::from_env::<Configuration>()
        .expect("Please provide the PORT env variable");

    // Deserialize the Configuration struct from environment variables with a
    // prefix. It expects a MY_APP__PORT environment variable to be set.
    let c2 = envy::prefixed("MY_APP__")
        .from_env::<Configuration>()
        .expect("Please provide MY_APP__PORT env variable");

    println!("c: {:?} c2: {:?}", c, c2);
}
// ANCHOR_END: example

#[test]
fn test() {
    unsafe {
        std::env::set_var("PORT", "80");
        std::env::set_var("MY_APP__PORT", "8080");
    }
    main();
}
