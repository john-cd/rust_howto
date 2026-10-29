// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the use of the `move` keyword with closures.

use std::thread;

/// Demonstrates the use of the `move` keyword with closures.
fn main() {
    let mut data = vec![1, 2, 3];
    println!("Before defining closure: {data:?}"); // `data` is still available here.

    // `move` forces the closure to take ownership of the values it uses.
    // Without `move`, this would borrow `data`, but `thread::spawn` requires
    // the closure to be `'static` because the new thread may outlive `main`.
    let handle = thread::spawn(move || {
        // `move` forces the closure to take ownership of `data`.
        println!("Data inside thread: {data:?}");
        data.push(4);
    });

    // println!("{data:?}"); // Error: `data` was moved into the closure.

    handle.join().expect("worker thread should not panic");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the closures move example through its entry point.
    #[test]
    fn test_closures_move_example_entry_point() {
        main();
    }

    // Output-only example: `main` only prints values; there is nothing else to
    // check.
}
