#![allow(dead_code)]
// ANCHOR: example
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        // The `id_val` variable is bound to `msg.id` if it's in 3..=7.
        Message::Hello { id: id_val @ 3..=7 } => {
            println!("Found an id in range: {}.", id_val);
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range (no binding here).");
        }
        Message::Hello { id } => {
            println!("Found some other id: {}.", id);
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
