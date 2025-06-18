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
        Message::Write(text) => println!("Text message: {}", text),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Quit => println!("Quit"),
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
