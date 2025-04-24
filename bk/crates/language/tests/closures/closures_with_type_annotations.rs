// ANCHOR: example
use std::thread;
use std::time::Duration;

/// Demonstrates closures with type annotations.
fn main() {
    // Closures can use type annotations, as shown below.
    // Multiple statements can be enclosed in a block.
    let _expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
