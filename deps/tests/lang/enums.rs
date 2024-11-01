#![allow(dead_code)]
// ANCHOR: example

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // struct-like
    Write(String),           // tuple-like
    ChangeColor(i32, i32, i32),
}

// Define methods on enums.
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let _home = Message::ChangeColor(127, 0, 0); // <-- note the ::
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
