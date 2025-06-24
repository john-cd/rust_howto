#![allow(dead_code)]
// ANCHOR: example
/// Demonstrates the use of tuples and arrays in Rust, including
/// their declaration, access, destructuring, iteration, and mutation.
fn main() {
    println!("=== TUPLES ===");

    // Tuple with mixed types:
    // Tuples are fixed-size collections of values of potentially different
    // types.
    let person: (String, i32, bool) = (String::from("Alice"), 30, true);

    // Access tuple elements by index:
    // Tuple elements are accessed using a dot followed by the index
    // (starting from 0).
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Active: {}", person.2);

    // Destructure a tuple:
    // Tuple elements can be extracted into separate variables.
    let (name, age, active) = person;
    println!(
        "Destructured - Name: {}, Age: {}, Active: {}",
        name, age, active
    );

    // Nested tuples:
    // Tuples can contain other tuples, allowing for complex data structures.
    let complex_data = (("Coordinates", (10.5, 20.8)), 42, [1, 2, 3]);
    println!(
        "Nested tuple access: {}, {}",
        (complex_data.0).0,
        (complex_data.0).1.0
    );

    // Unit (empty tuple).
    // The unit type `()` is a special tuple with no elements.
    let empty: () = ();
    println!("Empty tuple: {:?}", empty);

    // Tuple with a single element:
    // A single-element tuple requires a trailing comma to distinguish it from a
    // parenthesized expression.
    let single = (42,); // Note the comma.
    println!("Single element tuple: {:?}", single);

    // Functions with tuples.
    // Tuples can be passed to and returned from functions.
    fn swap_tuple(tuple: (i32, i32)) -> (i32, i32) {
        (tuple.1, tuple.0)
    }

    let original = (10, 20);
    let swapped = swap_tuple(original);
    println!("\nOriginal tuple: {:?}", original);
    println!("Swapped tuple: {:?}", swapped);

    println!("\n=== ARRAYS ===");

    // Fixed-size array with explicit type:
    // Arrays are fixed-size collections of elements of the same type.
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Array with repeated values:
    // Arrays can be initialized with a repeated value
    // using the `[value; size]` syntax.
    let _zeros = [0; 10]; // Creates [0, 0, 0, 0, 0, 0, 0, 0, 0, 0].

    // Array elements are accessed using square brackets
    // and the index (starting from 0).
    println!("First element: {}", numbers[0]);
    println!("Last element: {}", numbers[4]);

    // The `len()` method returns the number of elements in the array.
    println!("Array length: {}", numbers.len());

    // Slices from an array:
    // Slices provide a view into a contiguous sequence of elements in an array.
    let slice = &numbers[1..4]; // [2, 3, 4].
    println!("Slice: {:?}", slice);

    // Array of tuples:
    // Arrays can contain tuples, allowing for structured data within an array.
    let pairs: [(i32, &str); 3] = [(1, "one"), (2, "two"), (3, "three")];

    // Iterating over an array:
    // The `iter()` method provides an iterator over the array elements.
    println!("Iterating over array:");
    for num in numbers.iter() {
        println!("  Value: {}", num);
    }

    // The `enumerate()` method provides both the index
    // and the value for each element.
    println!("Iterating with index:");
    for (i, num) in numbers.iter().enumerate() {
        println!("  numbers[{}] = {}", i, num);
    }

    // Iterating over an array of tuples allows access
    // to both parts of each tuple.
    println!("Iterating over array of tuples:");
    for (number, name) in pairs.iter() {
        println!("  {} is written as {}", number, name);
    }

    // Multi-dimensional array:
    // Arrays can be nested to create multi-dimensional arrays.
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    println!("Multi-dimensional array:");
    for row in matrix.iter() {
        println!("  {:?}", row);
    }

    // Arrays can be mutated if declared with `mut`.
    let mut mutable_array = [1, 2, 3, 4, 5];
    mutable_array[2] = 99;
    println!("Mutated array: {:?}", mutable_array);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
