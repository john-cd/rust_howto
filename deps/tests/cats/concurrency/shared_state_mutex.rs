// ANCHOR: example
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // We wrap Mutex in Arc to allow for multiple owners.
    // Arc<T> is safe to use in concurrent situations.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // `clone` is somewhat a misnomer; it creates another pointer to the
        // same Mutex, increasing the strong reference count.
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(
            move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }, /* releases the lock automatically when the MutexGuard
                * goes out of scope. */
        );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
