#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the use of the `lazy_static` crate.
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert("James", vec!["user", "admin"]);
        map.insert("Jim", vec!["user"]);
        map
    };
}

// Example function: display the access privileges for a given name.
fn show_access(name: &str) {
    // Access the global static:
    let access = PRIVILEGES.get(name);
    println!("{name}: {access:?}");
}

fn main() {
    let access = PRIVILEGES.get("James");
    println!("James: {access:?}");

    show_access("Jim");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [review lazy_constant.rs and lazy_static.rs and global_mut_state.rs](https://github.com/john-cd/rust_howto/issues/939)
