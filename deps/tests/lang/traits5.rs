#![allow(dead_code)]
// ANCHOR: example

use std::clone::Clone;
use std::fmt::Debug;

// Note the `+`
fn a_function(item: &(impl Debug + Clone)) {
    println!("{:?}", item.clone());
}

fn some_function<T, U>(_t: &T, _u: &U) -> i32
where
    T: Debug + Clone, // note the `+`
    U: Debug + Clone,
{
    42
}

#[derive(Debug, Clone)]
struct S;

fn main() {
    let s = S;
    a_function(&s);
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
