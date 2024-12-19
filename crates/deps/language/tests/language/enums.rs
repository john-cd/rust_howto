#![allow(dead_code)]
// ANCHOR: example

#[derive(Debug)]
enum Message {
    Quit,                       // Unit-like variant (no fields)
    Move { x: i32, y: i32 },    // Struct-like variant (named fields)
    Write(String),              // Tuple-like variant (numbered fields)
    ChangeColor(i32, i32, i32), // Another tuple-like variant
}

// Define methods on enums.
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    // `msg` is assigned one of the variants.
    // Note the :: between the name of the type and the name of the variant.
    let msg = Message::Quit;
    println!("{msg:?}");
    // or
    let msg = Message::Move { x: 10, y: 15 };
    println!("{msg:?}");
    // or
    let msg = Message::ChangeColor(127, 0, 0);
    println!("{msg:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
