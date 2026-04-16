// ANCHOR: example
use heapless::FnvIndexMap;
use heapless::String;
use heapless::Vec;

fn main() {
    // `heapless` collections have a fixed capacity determined at compile time.
    // Here we define a fixed-size vector with capacity of 3.
    let mut vec: Vec<u32, 3> = Vec::new();

    vec.push(1).unwrap();
    vec.push(2).unwrap();
    vec.push(3).unwrap();
    // Because they have a fixed size, operations like `push` or `insert` can
    // fail if the collection is full. In that case, `push` returns back the
    // item.
    if let Err(err) = vec.push(4) {
        println!("Error pushing to vector: {err:?}");
    };

    println!("Vector: {vec:?}");
    println!("Vector length: {}", vec.len());
    println!("Vector capacity: {}", vec.capacity());

    if let Some(last) = vec.pop() {
        println!("Popped: {last}");
    }

    // Declare a fixed-size string with capacity of 16:
    let mut string: String<16> = String::from("Hello");

    assert!(string.push_str(", world!").is_ok());

    println!("String: {string}");
    println!("String length: {}", string.len());
    println!("String capacity: {}", string.capacity());

    // Returns an error, if we exceed the capacity:
    let result = string.push_str(" It is too much!");
    if let Err(err) = result {
        println!("Error pushing to string: {err:?}"); // `err` is simply `()`.
    }

    // Define a fixed-size map (using the `Fnv` hash for performance):
    let mut map: FnvIndexMap<&str, u32, 4> = FnvIndexMap::new();

    map.insert("one", 1).unwrap();
    map.insert("two", 2).unwrap();
    map.insert("three", 3).unwrap();
    map.insert("four", 4).unwrap();
    // If a key already exists in the map, the key remains and retains in its
    // place in the order, its corresponding value is updated with the new
    // value and the older value is returned inside `Some(_)`.
    assert_eq!(map.insert("three", 33), Ok(Some(3)));

    println!("Map: {map:?}");

    if let Some(value) = map.get("two") {
        println!("Value for 'two': {value}");
    }

    let result = map.insert("five", 5);
    if let Err(err) = result {
        println!("Error inserting to map: {err:?}");
    }

    // Iterate over the map:
    for (key, value) in &map {
        println!("Key: {key}, Value: {value}");
    }

    // Clear the map:
    map.clear();
    println!("Map is empty: {}", map.is_empty());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
