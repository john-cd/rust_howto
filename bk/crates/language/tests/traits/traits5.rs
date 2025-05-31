#![allow(dead_code)]
// ANCHOR: example
use std::clone::Clone;
use std::fmt::Debug;

/// `a_function` takes a reference to a type that implements both `Debug` and
/// `Clone`. Note the `+`
fn a_function(item: &(impl Debug + Clone)) {
    println!("{:?}", item.clone());
}

/// `some_function` takes two references to types `T` and `U` that both must
/// implement `Debug` and `Clone`. Note the `+`
fn some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Debug + Clone,
    U: Debug + Clone,
{
    42
}

#[derive(Debug, Clone)]
struct S;

fn main() {
    let s = S;
    // `S` implements both `Debug` and `Clone`, thus we can call `some_function`
    // with it.
    a_function(&s);

    some_function(&s, &s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
