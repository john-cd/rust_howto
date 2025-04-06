// ANCHOR: example
#![allow(clippy::approx_constant)]

use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

/// The `ordered-float` crate provides a wrapper type `OrderedFloat` for
/// floating-point numbers that can be used as keys in ordered collections
/// like `BTreeMap`.
///
/// The standard floating-point types (f32, f64) do not implement `Ord` because
/// NaN (Not-a-Number) values do not have a well-defined ordering.
/// `OrderedFloat` ensures that comparisons involving NaN values are consistent
/// with the requirements for ordered collections.
fn main() {
    // Create a BTreeMap with OrderedFloat keys
    let mut map: BTreeMap<OrderedFloat<f64>, &str> = BTreeMap::new();

    // Insert some values into the map
    map.insert(OrderedFloat(3.14), "pi");
    map.insert(OrderedFloat(2.718), "e");
    map.insert(OrderedFloat(1.618), "golden ratio");

    // Iterate over the map and print the values
    for (key, value) in &map {
        println!("{:.3}: {}", key, value);
    }

    // Check if a value exists in the map
    let key = OrderedFloat(2.718);
    if map.contains_key(&key) {
        println!("The map contains the key 2.718");
    } else {
        println!("The map does not contain the key 2.718");
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
