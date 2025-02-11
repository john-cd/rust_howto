// ANCHOR: example
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn expensive_function() {
    thread::sleep(Duration::from_secs(1));
}

fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
