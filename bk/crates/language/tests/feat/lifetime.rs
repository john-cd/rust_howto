#![allow(dead_code)]
// ANCHOR: example

/// A struct that holds a reference to a string slice.
///
/// The lifetime parameter `'a` specifies that the `ImportantExcerpt`
/// cannot outlive the string slice it references.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl ImportantExcerpt<'_> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    let ie = ImportantExcerpt { part: "a part" };
    println!("{}", ie.level());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
