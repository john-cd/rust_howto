// ANCHOR: example
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::MutexGuard;

use anyhow::anyhow;
use lazy_static::lazy_static;

// `lazy_static` allows you to define statically initialized values that are
// computed lazily at runtime. It can be particularly useful for initializing
// data that is expensive to compute or that needs to be shared across multiple
// threads.

lazy_static! {
    // - Any type within the macro needs to fulfill the Sync trait.
    // - If a type has a destructor, it will not run when the process exits.
    // - The `static ref` keywords are passed to the macro, they are not part of the language per se.

    // CONFIG is a lazily-initialized HashMap that holds configuration data.
    static ref CONFIG: HashMap<String, String> = {
        let mut m = HashMap::new();
        m.insert("host".to_string(), "localhost".to_string());
        m.insert("port".to_string(), "8080".to_string());
        m
    };
    // You can call a method or a function:
    static ref COUNT: usize = CONFIG.len();

    // COUNTER is a lazily-initialized Mutex-protected integer counter.
    // The Mutex ensures that the COUNTER can be safely incremented from multiple threads, if needed.
    static ref COUNTER: Mutex<i32> = Mutex::new(0);
}

fn main() -> anyhow::Result<()> {
    // Accessing the CONFIG.
    println!(
        "Host: {}",
        CONFIG.get("host").ok_or(anyhow!("No host key"))?
    );
    println!(
        "Port: {}",
        CONFIG.get("port").ok_or(anyhow!("No port key"))?
    );

    // The macro generates a unique type that implements `Deref<TYPE>` and
    // stores it in a static with name `NAME`. Here, we use `*` to dereference
    // and retrieve the inner value.
    println!("Count: {}", *COUNT);

    // Working with the COUNTER:
    // The (mutable) shared state within the Mutex can only be accessed once the
    // lock is held. Our non-atomic increment is safe because we're the only
    // thread which can access the shared state when the lock is held.
    // We `unwrap()` the return value to assert that we are not expecting
    // threads to ever fail while holding the lock.
    {
        let mut counter: MutexGuard<'_, i32> = COUNTER.lock().unwrap();
        *counter += 1;
        println!("Counter: {}", *counter);
    } // The lock is unlocked here.

    // And again, on a different thread:
    std::thread::spawn(move || {
        let lock_result = COUNTER.try_lock();
        // If the lock cannot be acquired at this time (or is poisoned),
        // `Err` is returned.
        if let Ok(mut counter) = lock_result {
            *counter += 1;
            println!("Counter: {}", *counter);
        } else {
            println!("try_lock failed");
        }
    })
    .join()
    .expect("thread::spawn failed");

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review lazy_constant.rs and lazy_static.rs and global_mut_state.rs](https://github.com/john-cd/rust_howto/issues/939)
