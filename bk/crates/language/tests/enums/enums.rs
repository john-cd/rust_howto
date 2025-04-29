#![allow(dead_code)]
// ANCHOR: example
/// An enum to represent different types of messages.
#[derive(Debug)]
enum Message {
    Quit,                       // Unit-like variant (no fields).
    Move { x: i32, y: i32 },    // Struct-like variant (named fields).
    Write(String),              // Tuple-like variant (numbered fields).
    ChangeColor(i32, i32, i32), // Another tuple-like variant.
}

/// Define methods on enums.
impl Message {
    fn call(&self) {
        // The `match` keyword allows us to compare a value against a series of
        // patterns. Each pattern can have associated code that will be
        // run if the value matches the pattern.
        match self {
            Message::Quit => println!("The Quit variant has no data to show."),
            Message::Move { x, y } => println!(
                "Move in the x direction {x} and in the y direction {y}"
            ),
            Message::Write(text) => println!("Message: {text}"),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}")
            }
        }
        // Enums make your code safer, because the compiler knows all the
        // possible variants a value can have, and makes sure the
        // `match` expression handles all possibilities.
    }
}

fn main() {
    // `msg` is assigned one of the variants.
    // Note the :: between the name of the type and the name of the variant.
    let msg = Message::Quit;
    println!("{msg:?}");
    // Or
    let msg = Message::Move { x: 10, y: 15 };
    println!("{msg:?}");
    // Or
    let msg = Message::ChangeColor(127, 0, 0);
    println!("{msg:?}");

    let msg = Message::Write(String::from("hello"));
    msg.call();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
