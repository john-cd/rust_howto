#![allow(dead_code)]
// ANCHOR: example
use development_tools_procedural_macro_helpers::MyDebug;

#[derive(MyDebug)]
struct MyStruct;

fn main() {
    let s = MyStruct;
    println!("{:?}", s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
