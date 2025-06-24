#![allow(dead_code)]
// ANCHOR: example
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread and store its handle in `thread_one`.
    let thread_one = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let thread_two = thread::spawn(|| { /* ... */ });
    // More stuff...

    // Wait for threads to complete and handle potential panics.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
