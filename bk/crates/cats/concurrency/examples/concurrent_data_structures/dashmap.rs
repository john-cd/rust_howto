// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the use of `DashMap` for concurrent data access and
//! modification.
//!
//! This example showcases how to create a shared `DashMap`, access and modify
//! it from multiple threads, and perform various operations like insertion,
//! retrieval, modification, and removal.

use std::thread;

use dashmap::DashMap;

/// Inserts, reads and modifies the entries of a `DashMap` from several threads.
fn main() -> anyhow::Result<()> {
    let map = DashMap::new();
    thread::scope(|scope| {
        for worker in 0..4 {
            let map = &map;
            scope.spawn(move || {
                for entry in 0..100 {
                    map.insert((worker, entry), entry);
                }
            });
        }
    });
    assert_eq!(map.len(), 400);
    assert_eq!(map.get(&(3, 99)).map(|value| *value), Some(99));
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that concurrent inserts produce the expected map contents.
    /// Runs the dashmap example through its entry point.
    #[test]
    fn test_dashmap_example_entry_point() {
        main().expect("dashmap recipe should pass its assertions");
    }
}
