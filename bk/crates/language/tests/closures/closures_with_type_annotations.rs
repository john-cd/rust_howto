// ANCHOR: example
use std::thread;
use std::time::Duration;

/// Demonstrates closures with type annotations.
fn main() {
    // Closures can use type annotations, as shown below.
    // Multiple statements can be enclosed in a block.
    let expensive_closure = |num: u64| -> u64 {
        println!("Calculating...");
        thread::sleep(Duration::from_secs(num));
        num
    };
    println!("Return value: {}", expensive_closure(1));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
