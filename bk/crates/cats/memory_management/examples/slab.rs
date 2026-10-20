// ANCHOR: example
//! Demonstrates using `slab` for pre-allocated storage.
//!
//! `slab` provides a slab allocator - a pre-allocated pool of memory
//! for a single data type. This avoids repeated heap allocations and
//! reduces memory fragmentation.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! slab = "0.4"
//! ```

use slab::Slab;

fn main() {
    // Create a new slab with no pre-allocated storage.
    let mut slab: Slab<String> = Slab::new();

    // `insert` stores a value and returns its integer key.
    let hello_key = slab.insert("hello".to_string());
    let world_key = slab.insert("world".to_string());

    println!("Key {hello_key} => {}", slab[hello_key]);
    println!("Key {world_key} => {}", slab[world_key]);

    // Look up a value by key.
    if let Some(val) = slab.get(hello_key) {
        println!("Found: {val}");
    }

    // Remove a value by key.
    let removed = slab.remove(hello_key);
    println!("Removed: {removed}");
    println!("Slab length: {}", slab.len());

    // Keys may be reused after removal.
    let reused_key = slab.insert("new entry".to_string());
    println!("Reused key {reused_key} => {}", slab[reused_key]);

    // Iterate over all entries.
    for (key, value) in &slab {
        println!("  [{key}] {value}");
    }

    // Pre-allocate capacity for a given number of entries.
    let mut slab2: Slab<i32> = Slab::with_capacity(10);
    for i in 0..5 {
        slab2.insert(i * i);
    }
    println!("slab2 capacity: {}, len: {}", slab2.capacity(), slab2.len());
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
