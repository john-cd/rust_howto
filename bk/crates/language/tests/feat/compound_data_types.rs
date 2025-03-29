// ANCHOR: example
fn main() {
    println!("=== TUPLES ===");

    // Declaring a tuple with mixed types
    let person: (String, i32, bool) = (String::from("Alice"), 30, true);

    // Accessing tuple elements by index
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Active: {}", person.2);

    // Destructuring a tuple
    let (name, age, active) = person;
    println!(
        "Destructured - Name: {}, Age: {}, Active: {}",
        name, age, active
    );

    // Nested tuples
    let complex_data = (("Coordinates", (10.5, 20.8)), 42, [1, 2, 3]);
    println!(
        "Nested tuple access: {}, {}",
        (complex_data.0).0,
        (complex_data.0).1.0
    );

    // Unit (empty tuple)
    let empty: () = ();
    println!("Empty tuple: {:?}", empty);

    // Tuple with a single element
    let single = (42,); // Note the comma
    println!("Single element tuple: {:?}", single);

    println!("\n=== ARRAYS ===");

    // Fixed-size array with explicit type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Array with repeated values
    let _zeros = [0; 10]; // Creates [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    // Accessing array elements
    println!("First element: {}", numbers[0]); // Starts at zero
    println!("Last element: {}", numbers[4]);

    // Getting array length
    println!("Array length: {}", numbers.len());

    // Array slices
    let slice = &numbers[1..4]; // [2, 3, 4]
    println!("Slice: {:?}", slice);

    // Array of tuples
    let pairs: [(i32, &str); 3] = [(1, "one"), (2, "two"), (3, "three")];

    // Iterating over an array
    println!("Iterating over array:");
    for num in numbers.iter() {
        println!("  Value: {}", num);
    }

    // Iterating with index
    println!("Iterating with index:");
    for (i, num) in numbers.iter().enumerate() {
        println!("  numbers[{}] = {}", i, num);
    }

    // Iterating over array of tuples
    println!("Iterating over array of tuples:");
    for (number, name) in pairs.iter() {
        println!("  {} is written as {}", number, name);
    }

    // Multi-dimensional array
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    println!("Multi-dimensional array:");
    for row in matrix.iter() {
        println!("  {:?}", row);
    }

    // Mutating arrays
    let mut mutable_array = [1, 2, 3, 4, 5];
    mutable_array[2] = 99;
    println!("Mutated array: {:?}", mutable_array);

    // Functions with tuples
    fn swap_tuple(tuple: (i32, i32)) -> (i32, i32) {
        (tuple.1, tuple.0)
    }

    let original = (10, 20);
    let swapped = swap_tuple(original);
    println!("\nOriginal tuple: {:?}", original);
    println!("Swapped tuple: {:?}", swapped);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
