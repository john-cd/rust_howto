// ANCHOR: example
use skiplist::SkipMap;

pub fn main() {
    let mut map = SkipMap::new();

    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");

    for (k, v) in map.iter() {
        println!("{}: {}", k, v);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
