// ANCHOR: example
// Bring `HashMap` in scope
use std::collections::HashMap;

fn main() {
    // We now refer to `HaspMap` without using its path
    let mut mymap: HashMap<u32, String> = HashMap::new();

    // Let's add something to it then print...
    mymap.entry(42).or_insert("my favorite number".into());
    println!("{:?}", mymap);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
