// ANCHOR: example
use std::sync::Arc;
use std::thread;

use dashmap::DashMap;

fn main() {
    // Create a shared DashMap with an Arc
    let map: Arc<DashMap<&str, i32, _>> = Arc::new(DashMap::new());
    // or use: DashMap::with_capacity(20)

    // Create multiple threads
    let mut threads = Vec::new();
    for i in 0..4 {
        let map_clone = map.clone();
        let thread_id = i;
        threads.push(thread::spawn(move || {
            // Access and modify the map from each thread
            match thread_id {
                0 => {
                    map_clone.insert("key1", thread_id);
                    println!("Thread {} inserted key1", thread_id);
                }
                1 => {
                    map_clone.insert("key2", thread_id);
                    println!("Thread {} inserted key2", thread_id);
                }
                2 => match map_clone.get("key1") {
                    Some(value) => {
                        println!("Thread {} read key1: {}", thread_id, *value);
                    }
                    _ => {
                        println!("Thread {} couldn't find key1", thread_id);
                    }
                },
                3 => match map_clone.get_mut("key2") {
                    Some(mut value) => {
                        *value += 10;
                        println!(
                            "Thread {} incremented key2 value to {}",
                            thread_id, *value
                        );
                    }
                    _ => {
                        println!("Thread {} couldn't find key2", thread_id);
                    }
                },
                _ => panic!("Unknown thread ID"),
            }
        }));
    }

    // Wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(map.remove("key1").unwrap().1, 0); // returns Option<(K, V)>

    assert!(map.contains_key("key2"));

    map.remove_if("key2", |_, val| *val == 11);

    // Access the final state of the map from the main thread
    println!("final count: {}", map.iter().count());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
