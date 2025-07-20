#![allow(dead_code)]
// ANCHOR: example
use std::clone::Clone;
use std::fmt::Debug;

/// This function takes a reference to a type that must implement both the
/// `Debug` and `Clone` traits. Note the `+`
fn func<T: Debug + Clone>(t: &T) {
    println!("{:?}", t.clone());
    // We can call `clone` because `T` implements `Clone`.
    // We can also print `t` with `:?` because it implements `Debug`.
}

/// It  may be equivalently be written in a `where` clause:
fn func2<T>(t: &T)
where
    T: Debug + Clone,
{
    println!("{:?}", t.clone());
}

/// You can also use the similar `impl Trait` syntax.
/// `impl Trait` is an opaque type that implements the specified traits.
/// Parentheses are necessary.
fn func3(item: &(impl Debug + Clone)) {
    println!("{:?}", item.clone());
}

#[derive(Debug, Clone)]
struct S;

fn main() {
    let s = S;
    // Sinc `S` implements both `Debug` and `Clone`, we can call the above
    // functions:
    func(&s);
    func2(&s);
    func3(&s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
