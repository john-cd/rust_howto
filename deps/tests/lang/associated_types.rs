#![allow(dead_code)]
// ANCHOR: example

trait Iterator {
    type Item; // <-- associated type
               // in Impl, use e.g. `Iterator<Item = u32>`

    fn next(&mut self) -> Option<Self::Item>;
}

// Generic type with default
trait Add<Rhs = Self> {
    type Output; // <-- associated type

    fn add(self, rhs: Rhs) -> Self::Output;
}

fn main() {}
// ANCHOR_END: example

// TODO

#[test]
fn test() {
    main();
}
