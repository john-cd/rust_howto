// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A concurrent hash map from the `papaya` crate, used from several threads.
//!
//! A concurrent hash map lets many threads read and write at once. Papaya's
//! reads never block, and the map is built for read-heavy workloads such as
//! caches and registries.

use papaya::HashMap;

// `papaya` is a concurrent hash-table for read-heavy workloads.
// It avoids the overhead and deadlock potential of traditional locks by using
// "pinning." Pins act like lock guards.

// Use a map from multiple threads.
/// Inserts into and reads from a shared `papaya` map from several threads.
fn main() -> anyhow::Result<()> {
    // Create a map.
    let map = HashMap::new();
    std::thread::scope(|s| {
        // Insert some values.
        s.spawn(|| {
            // Pin the map.
            // Pinning and unpinning the table is relatively cheap but not free.
            // Therefore, pin reuse is encouraged, within reason, noting that,
            // as long as we are holding on to a pin, you
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

        // Insert a disjoint range from another thread.
        s.spawn(|| {
            let m = map.pin();
            for i in 'a'..='z' {
                m.insert(i, 2);
            }
        });

        // Read the values.
        // Note that:
        // - Read and write operations may overlap in time.
        // - There is no support for locking the entire table nor individual
        //   keys to prevent concurrent access, except through external
        //   fine-grained locking.
        // - Read operations (such as get) reflect the results of the
        //   most-recent write.
        s.spawn(|| {
            let _entries_seen = map.pin().iter().count();
        });

        // Atomic operations:
        let m = map.pin();
        // Updates an existing entry or inserts a default value atomically.
        m.update_or_insert('#', |e| e + 1, 1);
        // In this case, the key did not exist and `1` was inserted.
        assert_eq!(m.get(&'#'), Some(&1));
        // Updates an existing entry atomically.
        assert_eq!(m.update('#', |e| e + 1), Some(&2));
    });
    assert_eq!(map.pin().iter().count(), 53);
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies concurrent inserts and atomic updates leave the expected
    /// entries.
    /// Runs the papaya example through its entry point.
    #[test]
    fn test_papaya_example_entry_point() {
        main().expect("papaya recipe should pass its assertions");
    }
}
