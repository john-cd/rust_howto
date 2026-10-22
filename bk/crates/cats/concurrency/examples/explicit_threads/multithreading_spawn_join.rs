#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Spawning threads and waiting for them with `join`.

use std::thread;
use std::time::Duration;

/// Spawns two threads and waits for both to finish.
fn main() {
    // Spawn a new thread and store its handle in `thread_one`.
    let thread_one = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
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

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
