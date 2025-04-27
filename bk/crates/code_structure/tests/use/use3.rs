// ANCHOR: example
// This `use` statement allows us to use `HashMap` directly without having to
// specify its full path.
// IDIOM: we imported the `struct` itself, not its parent module.
use std::collections::HashMap;

// `HashMap` is declared in the standard library (`std`), which is an external
// crate. We therefore prefix the path with the crate's name.

fn main() {
    // We now refer to `HashMap` without using its full path.
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
