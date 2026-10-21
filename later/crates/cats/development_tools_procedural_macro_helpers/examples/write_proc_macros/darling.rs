// TODO Reviewed ??-??-2026
#![allow(dead_code)]
// ANCHOR: example
use development_tools_procedural_macro_helpers::MyDarlingMacro;

pub trait MyDarlingTrait {
    fn hello();
}

#[derive(MyDarlingMacro)]
#[my_macro(name = "World")]
struct MyStruct;

fn main() {
    <MyStruct as MyDarlingTrait>::hello();
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[test]
fn test() {
    main();
}
