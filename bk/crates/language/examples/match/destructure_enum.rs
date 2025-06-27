#![allow(dead_code)]
// ANCHOR: example
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Write(String::from("hello"));

    match msg {
        Message::Write(text) => println!("Text message: {text}"),
        Message::Move { x, y } => println!("Move to x: {x}, y: {y}"),
        Message::Quit => println!("Quit"),
        // Must be exhaustive.
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
