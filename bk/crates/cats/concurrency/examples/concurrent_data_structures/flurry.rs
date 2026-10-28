#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates the basic usage of the `flurry::HashMap` for
//! concurrent data access. It showcases how to insert, read, remove, and
//! iterate over the map, both sequentially and concurrently.

use std::sync::Arc;
use std::thread;

use flurry::HashMap;

/// Performs sequential insertion, reading, removal, and inspection on a flurry HashMap.
fn sequential_operations(map: &Arc<HashMap<String, i32>>) {
    {
        let guard = map.guard();
        map.insert("key0".to_owned(), 0, &guard);
    }
    {
        let mref = map.pin();
        mref.insert("key1".to_owned(), 1);
        mref.insert("key2".to_owned(), 2);
        mref.insert("key3".to_owned(), 3);

        assert_eq!(mref.get("key1"), Some(&1));
        assert_eq!(mref.get("key4"), None);

        mref.remove("key0");
        assert_eq!(mref.get("key0"), None);
        assert!(mref.contains_key("key2"));

        assert_eq!(mref.len(), 3);
    }
}

/// Spawns writer and reader threads to test concurrent operations.
fn concurrent_operations(map: &Arc<HashMap<String, i32>>) -> anyhow::Result<()> {
    let m = Arc::clone(map);
    let writer_thread = thread::spawn(move || {
        for i in 4..=5 {
            m.pin().insert(format!("key{i}"), i);
        }
    });

    let m2 = Arc::clone(map);
    let reader_thread = thread::spawn(move || {
        for _ in 1..=2 {
            println!("Reading from concurrent reader:");
            for (key, value) in &m2.pin() {
                println!("{key}: {value}");
            }
        }
    });

    writer_thread
        .join()
        .map_err(|_| anyhow::anyhow!("writer thread failed"))?;
    reader_thread
        .join()
        .map_err(|_| anyhow::anyhow!("reader thread failed"))?;

    let mref = map.pin();
    for i in 1..=5 {
        assert_eq!(mref.get(&format!("key{i}")), Some(&i));
    }
    Ok(())
}

/// Demonstrates sequential and concurrent usage of flurry::HashMap.
fn main() -> anyhow::Result<()> {
    let map = Arc::new(HashMap::new());
    sequential_operations(&map);
    concurrent_operations(&map)?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that concurrent map updates are visible after the workers join.
    /// Runs the flurry example through its entry point.
    #[test]
    fn test_flurry_example_entry_point() {
        main().expect("flurry recipe should pass its assertions");
    }

    #[test]
    fn test_sequential_operations() {
        let map = Arc::new(HashMap::new());
        sequential_operations(&map);
        let mref = map.pin();
        assert_eq!(mref.get("key1"), Some(&1));
        assert_eq!(mref.get("non_existent"), None);
    }

    #[test]
    fn test_concurrent_operations() -> anyhow::Result<()> {
        let map = Arc::new(HashMap::new());
        sequential_operations(&map);
        concurrent_operations(&map)?;
        Ok(())
    }
}
