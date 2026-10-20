// ANCHOR: example
use linked_hash_map::LinkedHashMap;

fn main() {
    let mut map = LinkedHashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);

    // Iteration respects insertion order
    for (k, v) in map.iter() {
        println!("{}: {}", k, v);
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
