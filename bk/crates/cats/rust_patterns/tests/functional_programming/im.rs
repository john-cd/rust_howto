// ANCHOR: example
//! # Immutable Data Structures with `im`
//!
//! The `im` crate provides immutable data structures.
//! These data structures are especially useful in functional
//! programming, where data is not modified in place but rather
//! new data structures are created with the desired changes.
//!
//! This said, `im` provides a safe default choice for the most common kinds of
//! data structures, allowing you to defer careful thinking about the right data
//! structure for the job until you need to start looking for optimizations.
//! Most operations are O(log n) or better.
//!
//! The most obvious benefit of _immutable data structures_ - avoiding the
//! accidental mutation of data - is already handled so well by Rust’s type
//! system that it’s just not something a Rust programmer needs to worry about.
//! Their most prominent advantage in Rust is therefore _structural sharing_. If
//! two data structures are mostly copies of each other, most of the memory they
//! take up will be shared between them. Therefore, making copies of an
//! immutable data structure is fast and cheap, with actual copying only
//! occurring when modifications are made. `im` is thus often the right choice
//! for larger data sets modified infrequently.
//!
//! Because `im` makes copies of shared nodes in its data structures before
//! updating them, the values you store must implement `Clone`. If you want to
//! store values for which cloning is expensive, or values that don’t implement
//! `Clone`, you need to wrap them in `Rc` or `Arc`.
use im::HashMap;
use im::HashSet;
use im::Vector;
use im::hashmap;
use im::vector;

fn main() {
    // Immutable Vector (based on RRB trees),
    // i.e. a sequence of elements in insertion order.
    // Practically every operation is O(log n), except push/pop on both sides,
    // which will be O(1) amortized, and O(log n) in the worst case.

    let mut v1 = Vector::new();
    // Push values to the back of a vector.
    v1.push_back(1);
    v1.push_back(2);
    v1.push_back(3);

    // Clone the Vector and modify.
    // `im` uses _lazy cloning_. The initial `clone` is instant, and as you
    // modify the cloned data structure, it will clone chunks only where you
    // change them, so that if you change the entire thing you will
    // eventually have performed a full clone.
    let mut v2 = v1.clone();
    v2.push_back(4);

    println!("Original vector: {:?}", v1);
    println!("Modified vector: {:?}", v2);

    // Create a new vector with the value at index index updated.
    let v3 = v2.update(1, 10);
    println!("Updated vector: {:?}", v3);

    // Simpler: use the `vector!` macro.
    assert_eq!(5, vector![1, 2, 3, 4, 5].len());

    // The `im` maps - HashMap and OrdMap - generally perform similarly to their
    // equivalents in the standard library, but tend to run a bit slower on the
    // basic operations. On the other hand, they offer the cheap copy and
    // structural sharing between copies that you’d expect from immutable
    // data structures.

    // Let's explore the Immutable HashMap (based on hash array mapped tries).
    // Most operations on this map are O(logx n) for a suitably high x that it
    // should be nearly O(1) for most maps. Keys will need to implement
    // `Hash` and `Eq`.
    assert_eq!(
        3,
        hashmap! {
        1 => 11,
        2 => 22,
        3 => 33
        }
        .len()
    );

    // Or construct a hash map with a single mapping.
    let map0 = HashMap::unit("key0", 0);
    assert_eq!(map0.get("key0"), Some(&0));

    // Or use `insert`:
    let mut map1 = HashMap::new();
    map1.insert("key1", 100);
    map1.insert("key2", 200);

    // Clone the map and modify.
    let mut map2 = map1.clone();
    map2.insert("key3", 300);

    println!("Original map: {:?}", map1);
    println!("Modified map: {:?}", map2);

    // Immutable HashSet
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(2);

    // Create a new set with a modification.
    let mut set2 = set1.clone();
    set2.insert(3);

    println!("Original set: {:?}", set1);
    println!("Modified set: {:?}", set2);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
