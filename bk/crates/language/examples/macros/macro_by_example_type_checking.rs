#![allow(dead_code)]
// ANCHOR: example
macro_rules! double {
    ($x:expr) => {
        ($x) * 2
    };
}

fn main() {
    println!("{}", double!(5));
    // ERROR cannot multiply `&str` by `{integer}`
    // println!("{}", double!("hello"));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
