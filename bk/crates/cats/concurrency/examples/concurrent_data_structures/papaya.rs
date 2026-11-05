//! TODO Reviewed ??-??-2026.

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

/// Populates disjoint character ranges into the map across scoped threads.
fn populate_concurrent_ranges(map: &HashMap<char, i32>) {
    std::thread::scope(|s| {
        // Pin the map.
        // Pinning and unpinning the table is relatively cheap but not free.
        // Therefore, pin reuse is encouraged, within reason, noting that,
        // as long as we are holding on to a pin, you
        // are preventing the map from performing garbage collection.
        s.spawn(|| {
            let m = map.pin();
            for i in 'A'..='Z' {
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

        // Read operations (such as get) reflect the results of the most-recent write.
        s.spawn(|| {
            let _entries_seen = map.pin().iter().count();
        });
    });
}

/// Applies atomic insertions and updates to the specified key.
fn apply_atomic_updates(map: &HashMap<char, i32>, key: char) -> (i32, Option<i32>) {
    let m = map.pin();
    // Updates an existing entry or inserts a default value atomically.
    let initial_lookup = *m.update_or_insert(key, |e| e + 1, 1);
    // Updates an existing entry atomically.
    let updated_value = m.update(key, |e| e + 1).copied();
    (initial_lookup, updated_value)
}

// Use a map from multiple threads.
/// Inserts into and reads from a shared `papaya` map from several threads.
fn main() -> anyhow::Result<()> {
    // Create a map.
    let map = HashMap::new();
    populate_concurrent_ranges(&map);

    let (initial, updated) = apply_atomic_updates(&map, '#');
    // Key did not exist previously, so initial insertion yielded 1.
    assert_eq!(initial, 1);
    assert_eq!(updated, Some(2));
    assert_eq!(map.pin().get(&'#'), Some(&2));

    assert_eq!(map.pin().iter().count(), 53);
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
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

    /// Verifies that atomic updates on a missing key insert the default value.
    #[test]
    fn atomic_update_or_insert_sets_default_for_new_key() {
        let map = HashMap::new();
        let (initial, updated) = apply_atomic_updates(&map, '@');
        assert_eq!(initial, 1);
        assert_eq!(updated, Some(2));
        assert_eq!(map.pin().get(&'@'), Some(&2));
    }

    /// Verifies that updating an existing key increments its value.
    #[test]
    fn atomic_update_increments_existing_entry() {
        let map = HashMap::new();
        map.pin().insert('!', 10);
        let m = map.pin();
        assert_eq!(m.update('!', |v| v + 5).copied(), Some(15));
        assert_eq!(m.get(&'!'), Some(&15));
    }

    /// Verifies that populating concurrent ranges inserts all 52 letters.
    #[test]
    fn populate_concurrent_ranges_inserts_all_expected_keys() {
        let map = HashMap::new();
        populate_concurrent_ranges(&map);
        let m = map.pin();
        assert_eq!(m.iter().count(), 52);
        for c in 'A'..='Z' {
            assert_eq!(m.get(&c), Some(&1));
        }
        for c in 'a'..='z' {
            assert_eq!(m.get(&c), Some(&2));
        }
    }

    /// Verifies that an empty papaya map reports zero length and empty iterator.
    #[test]
    fn empty_map_has_zero_entries() {
        let map = HashMap::<char, i32>::new();
        let m = map.pin();
        assert!(m.is_empty());
        assert_eq!(m.iter().count(), 0);
    }
}
