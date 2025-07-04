#![allow(dead_code)]
// ANCHOR: example
use std::sync::Arc;
use std::thread;

use flurry::HashMap;

/// This example demonstrates the basic usage of the `flurry::HashMap` for
/// concurrent data access. It showcases how to insert, read, remove, and
/// iterate over the map, both sequentially and concurrently.
fn main() {
    // Create a new Flurry HashMap.
    // Wrap in an Arc, so that it can be shared between threads
    let map = Arc::new(HashMap::new());
    {
        // Many of the access methods take a reference to a Guard.
        // You obtain a Guard using `HashMap::guard`, and you can
        // use references to the same guard to make one or more API
        // calls.
        let guard = map.guard();
        map.insert("key0".to_string(), 0, &guard);
        // Keep in mind that for as long as you hold onto this Guard, you are
        // preventing the collection of garbage generated by the map.
    }
    {
        // Alternatively, get a reference to the map with the current thread
        // pinned.
        assert!(!map.pin().is_empty());
        // Keep in mind that for as long as you hold onto this, you are
        // preventing the collection of garbage generated by the map.

        // You can also re-use a map pin like so:
        let mref = map.pin();

        // Insert key-value pair into the map.
        // If the map did not have this key present, None is returned.
        // If the map did have this key present, the value is updated,
        // and the old value is returned. The key is left unchanged.
        mref.insert("key1".to_string(), 1);
        mref.insert("key2".to_string(), 2);
        mref.insert("key3".to_string(), 3);

        // Read values.
        // Retrieval operations generally do not block, so may overlap with
        // update operations (including insert). Retrievals reflect the
        // results of the most recently completed update operations
        // holding upon their onset.
        println!("Value for key1: {:?}", mref.get(&"key1".to_string())); // Output: Some(1)
        println!("Value for key4: {:?}", mref.get(&"key4".to_string())); // Output: None

        println!("\nRemoving key0");
        mref.remove(&"key0".to_string());
        println!(
            "Value for key0 after removal: {:?}",
            mref.get(&"key0".to_string())
        );

        println!("\nChecking if key2 exists");
        println!("{}", mref.contains_key(&"key2".to_string()));

        // Operations that inspect the map as a whole, rather than a single key,
        // operate on a snapshot of the underlying table. For example, iterators
        // return elements reflecting the state of the hash table at some point
        // at or since the creation of the iterator.

        // Iterate over the map (read-only).
        println!("Iterating over the map:");
        for (key, value) in mref.iter() {
            println!("{key}: {value}");
        }

        // Aggregate status methods like `len` are typically useful only when a
        // map is not undergoing concurrent updates in other threads.
        assert!(mref.len() == 3);
    }

    let m = map.clone();
    let writer_thread = thread::spawn(move || {
        for i in 4..=5 {
            m.pin().insert(format!("key{i}"), i);
            // Simulate some work
            thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    let m2 = map.clone();
    let reader_thread = thread::spawn(move || {
        for _ in 1..=2 {
            // Read multiple times
            println!("Reading from concurrent reader:");
            for (key, value) in m2.pin().iter() {
                println!("{key}: {value}");
            }
            thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    writer_thread.join().unwrap();
    reader_thread.join().unwrap();

    // Final state of the map.
    println!("\nFinal state of the map:");
    for (key, value) in map.pin().iter() {
        println!("{key}: {value}");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [review https://github.com/jonhoo/flurry/blob/main/benches/flurry_dashmap.rs NOW](https://github.com/john-cd/rust_howto/issues/1152)
