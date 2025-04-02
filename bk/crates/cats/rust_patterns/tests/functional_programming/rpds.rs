// ANCHOR: example
use rpds::HashTrieMap;
use rpds::Vector;

// RPDS provides purely functional (immutable) data structures with structural
// sharing for efficiency.

fn main() {
    // Persistent Vector with immutable updates.
    let v1: Vector<i32> = Vector::new().push_back(1).push_back(2).push_back(3);

    // Create a new vector with index 1 set to 20
    let v2 = v1.set(1, 20).unwrap();

    println!("Original v1: {}", v1); // [1, 2, 3]
    println!("Modified v2: {}", v2); // [1, 20, 3]
    println!("v1 is unchanged: {}", v1); // [1, 2, 3]

    // Iterating
    for item in v2.iter() {
        println!("Item: {}", item);
    }

    // Persistent map implemented with a hash array mapped trie.
    let m1: HashTrieMap<String, i32> = HashTrieMap::new();
    let m2 = m1.insert("one".to_string(), 1).insert("two".to_string(), 2);
    let m3 = m2.insert("three".to_string(), 3);

    println!("m2 size: {}", m2.size()); // 2
    println!("m3 size: {}", m3.size()); // 3

    // Lookup
    println!("Value for 'two': {:?}", m3.get(&"two".to_string())); // Some(2)

    // Removal (creates a new map)
    let m4 = m3.remove(&"one".to_string());
    println!("m4 after removal: {}", m4);
    println!("m3 is unchanged: {}", m3);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
