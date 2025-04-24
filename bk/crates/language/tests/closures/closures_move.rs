// ANCHOR: example
use std::thread;

/// Demonstrates the use of the `move` keyword with closures.
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list); // `list` is still available here.

    // `move` forces the closure to take ownership of the values it uses.
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
