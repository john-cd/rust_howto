// ANCHOR: example
#![allow(unused_imports)]

// Use a glob (the `*`) to bring all public objects within a module
// (here `default`) in scope. Use sparingly.
use std::default::*;
// The most common use of globs is to import a "prelude", a group of items
// commonly used together. See the "code organization" chapter.
use std::io::prelude::*;
// You can combine multiple `use` lines together with { }.
use std::io::{Read, Write};
// Idiom: use `self` to refer to the module itself.
// The following is equivalent to `use std::iter; use std::iter::empty;`.
use std::iter::{self, empty};
// Use `as` to define aliases, for example in case of name conflict.
use std::mem::drop as destruct;

fn main() {
    // Use `std::default::Default` without writing down the whole path,
    // because we imported all public objects from the `default` module.
    let v: Vec<u8> = Default::default();

    // Use the alias defined above:
    destruct(v);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
