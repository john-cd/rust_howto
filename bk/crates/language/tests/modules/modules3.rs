// ANCHOR: example
// This allows us to use `HashMap` directly without having to
// specify its full path.
use std::collections::HashMap;

fn main() {
    // We now refer to `HashMap` without using its path.
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
