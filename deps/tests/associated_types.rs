#![allow(dead_code)]

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

// TODO
#[test]
fn test() {}
