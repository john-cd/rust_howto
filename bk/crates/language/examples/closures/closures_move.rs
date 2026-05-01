#![allow(dead_code)]
// ANCHOR: example
use std::thread;

/// Demonstrates the use of the `move` keyword with closures.
fn main() {
    let mut data = vec![1, 2, 3];
    println!("Before defining closure: {data:?}"); // `list` is still available here.

    // `move` forces the closure to take ownership of the values it uses.
    // Without `move`, this would be an error because the closure might outlive
    // `main`, making the reference `&data` invalid.
    let handle = thread::spawn(move || {
        // `move` forces the closure to take ownership of `data`.
        println!("Data inside thread: {data:?}");
        data.push(4);
    });

    // println!("{data:?}"); // Error: `data` was moved into the closure.

    handle.join().unwrap();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
