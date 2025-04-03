// ANCHOR: example
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// Demonstrates safe concurrent access to shared data using `Arc` and `Mutex`.

fn main() {
    // Using Arc (Atomic Reference Counting) and Mutex (Mutual Exclusion)
    // to safely share data between threads.
    // `Arc` allows multiple threads to have shared ownership of the data.
    // `Mutex` ensures that only one thread can access the data at a time.
    let data = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..3 {
        let data = Arc::clone(&data);
        // Create 3 threads, each of which increments the shared data by 1
        let handle = thread::spawn(move || {
            let mut num = data.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *data.lock().unwrap());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
