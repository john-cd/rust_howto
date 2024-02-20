#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Configuration {
    port: u16,
    items_per_page: u16,
}

#[test]
#[ignore]
fn test() {
    let c = envy::from_env::<Configuration>()
        .expect("Please provide PORT and ITEMS_PER_PAGE env vars");

    let c2 = envy::prefixed("MY_APP__")
        .from_env::<Configuration>()
        .expect(
            "Please provide MY_APP__PORT and MY_APP__ITEMS_PER_PAGE env vars",
        );

    println!("c: {:?} c2: {:?}", c, c2);
}
