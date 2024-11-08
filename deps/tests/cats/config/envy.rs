// ANCHOR: example
#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Configuration {
    port: u16,
}

fn main() {
    let c = envy::from_env::<Configuration>()
        .expect("Please provide the PORT env variable");

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
