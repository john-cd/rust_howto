// ANCHOR: example
use std::thread;
use std::time::Duration;

fn main() {
    // closure can use type annotation. Multiple statements can be
    // enclosed in a block.
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
