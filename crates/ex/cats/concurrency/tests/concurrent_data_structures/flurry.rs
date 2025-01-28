// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example
// use std::thread;

// use flurry::HashMap;

// // Cloning the HashMap before moving it into the threads. Flurry's HashMap is
// // designed for concurrent reads without requiring locks, but writes
// (inserts, // removes) require exclusive access. Cloning creates separate,
// independent maps // that can be modified concurrently without data races.

// How Flurry Works (Important Concepts):

// Copy-on-Write (COW): Flurry uses a copy-on-write strategy. When a write
// operation occurs, a copy of the underlying data structure is made, the write
// is performed on the copy, and then an atomic pointer swap makes the new copy
// the current version. This allows for concurrent reads without locks.
// Read-Only Iterators: The iterators returned by map.iter() are read-only and
// can be used concurrently. Cloning: Cloning a Flurry HashMap is a relatively
// cheap operation because it doesn't perform a deep copy of the data
// immediately. Instead, it shares the underlying data structure until a write
// operation occurs on one of the clones

// fn main() {
//     // Create a new Flurry HashMap.
//     let map = HashMap::new();

//     // Insert some initial values.
//     map.insert("key1".to_string(), 1);
//     map.insert("key2".to_string(), 2);
//     map.insert("key3".to_string(), 3);

//     // Read values.
//     println!("Value for key1: {:?}", map.get(&"key1".to_string())); //
// Output: Some(1)     println!("Value for key4: {:?}",
// map.get(&"key4".to_string())); // Output: None

//     // Iterate over the map (read-only).
//     println!("Iterating over the map:");
//     for (key, value) in map.iter() {
//         println!("{}: {}", key, value);
//     }

//     // Concurrent inserts and reads.
//     let map_clone = map.clone(); // Important: Clone the map for concurrent
// access!

//     let writer_thread = thread::spawn(move || {
//         for i in 4..10 {
//             map_clone.insert(format!("key{}", i), i);
//             thread::sleep(std::time::Duration::from_millis(10)); // Simulate
// some work         }
//     });

//     let reader_thread = thread::spawn(move || {
//         for _ in 0..20 {
//             // Read multiple times
//             println!("Reading from concurrent reader:");
//             for (key, value) in map_clone.iter() {
//                 println!("{}: {}", key, value);
//             }
//             thread::sleep(std::time::Duration::from_millis(50)); // Simulate
// some work         }
//     });

//     writer_thread.join().unwrap();
//     reader_thread.join().unwrap();

//     // Final state of the map.
//     println!("\nFinal state of the map:");
//     for (key, value) in map.iter() {
//         println!("{}: {}", key, value);
//     }

//     // Example with remove
//     let map_remove = map.clone();

//     println!("\nRemoving key1");
//     map_remove.remove(&"key1".to_string());
//     println!(
//         "Value for key1 after removal: {:?}",
//         map_remove.get(&"key1".to_string())
//     );

//     println!("\nFinal map after removal");

//     for (key, value) in map_remove.iter() {
//         println!("{}: {}", key, value);
//     }

//     // Example with contains_key
//     println!("\nChecking if key2 exists");
//     println!("{}", map.contains_key(&"key2".to_string()));

//     println!("\nChecking if key100 exists");
//     println!("{}", map.contains_key(&"key100".to_string()));
// }

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/684)
