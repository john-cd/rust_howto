#![allow(dead_code)]
// ANCHOR: example
use papaya::HashMap;

// `papaya` is a concurrent hash-table for read-heavy workloads.
// It avoids the overhead and deadlock potential of traditional locks by using
// "pinning." Pins act like lock guards.

// Use a map from multiple threads.
fn main() {
    // Create a map.
    let map = HashMap::new();
    std::thread::scope(|s| {
        // Insert some values.
        s.spawn(|| {
            // Pin the map.
            // Pinning and unpinning the table is relatively cheap but not free.
            // Therefore, pin reuse is encouraged, within reason, noting that,
            // as long as you are holding on to a pin, you
            // are preventing the map from performing garbage collection.
            let m = map.pin();
            for i in 'A'..='Z' {
                // Inserts a key-value pair into the map.
                // If the map did not have this key present, None is returned.
                // If the map did have this key present, the value is updated,
                // and the old value is returned.
                m.insert(i, 1);
            }
        });

        // Remove the values.
        // Note that the map is accessed from multiple threads.
        s.spawn(|| {
            let m = map.pin();
            for i in 'A'..='Z' {
                m.remove(&i);
            }
        });

        // Read the values.
        // Note that
        // - Read and write operations may overlap in time.
        // - There is no support for locking the entire table nor individual
        //   keys
        // to prevent concurrent access, except through external fine-grained
        // locking.
        // - Read operations (such as get) reflect the results of
        // the most-recent write.
        s.spawn(|| {
            for (key, value) in map.pin().iter() {
                println!("{key}: {value}");
            }
        });

        // Atomic operations:
        let m = map.pin();
        // Updates an existing entry or inserts a default value atomically.
        m.update_or_insert('#', |e| e + 1, 1);
        // In this case, the key did not exist and 1 was inserted.
        assert_eq!(m.get(&'#'), Some(&1));
        // Updates an existing entry atomically.
        assert_eq!(m.update('#', |e| e + 1), Some(&2));
    });
    // Example adapted from <https://docs.rs/papaya/latest/papaya/>
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
