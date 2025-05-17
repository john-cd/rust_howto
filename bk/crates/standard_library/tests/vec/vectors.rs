#![allow(clippy::vec_init_then_push)]
#![allow(clippy::useless_vec)]
#![allow(clippy::unnecessary_fold)]
// ANCHOR: example

fn main() {
    // Vectors are resizable arrays.

    // Create a `Vec`.
    let _numbers: Vec<i32> = vec![1, 2, 3];
    // Note that an array would work as well in this specific case.

    // The above is a shortcut for:
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // Read a value at a given index.
    // 1. Panics if out of bounds.
    let _third: &i32 = &numbers[2];
    // 2. Returns `None` if out of bounds.
    let _third: Option<&i32> = numbers.get(2);

    // Manipulate the `Vec`.
    numbers.push(4); // Add an element to the end.
    numbers.pop();

    // Iterate over a `Vec`.
    println!("Iterate using a `for` loop:");
    for num in &numbers {
        println!("{}", num);
    }

    println!("Iterate using `iter()`:");
    numbers.iter().for_each(|num| println!("{}", num));

    // Modify the vector while iterating (requires `mut`).
    for num in &mut numbers {
        *num += 50; // Use the dereference operator to access the value.
    }

    // Search in the `Vec` using `position`.
    if let Some(index) = numbers.iter().position(|&x| x == 52) {
        println!("Found the value `52` at index: {}", index);
    } else {
        println!("Not found.");
    }

    // Sort.
    let mut sorted_numbers = numbers.clone();
    sorted_numbers.sort();
    println!("Sorted numbers: {:?}", sorted_numbers);

    // Filter elements (keep only even numbers).
    let even_numbers: Vec<i32> =
        numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();
    println!("Even numbers: {:?}", even_numbers);

    // Map elements (square each number).
     // Collect the results into a new `Vec`.
    let squared_numbers: Vec<i32> = numbers.iter().map(|&x| x * x).collect();.
    println!("Squared numbers: {:?}", squared_numbers);

    // Reduce.
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    let product: i32 = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Product: {}", product);

    // Create a slice of the `Vec`.
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Iterate over a slice.
    println!("Iterate over a slice:");
    for num in slice.iter() {
        println!("{}", num);
    }

    // Mutable slices.
    let mut mutable_numbers = vec![1, 2, 3, 4, 5];
    let mutable_slice: &mut [i32] = &mut mutable_numbers[1..4];

    // Iterate over a mutable slice and modify elements.
    for num in mutable_slice.iter_mut() {
        *num *= 2;
    }
    println!(
        "Mutable numbers after slice mutation: {:?}",
        mutable_numbers
    );

    // Chain iterators for complex operations.
    let numbers2: Vec<i32> = vec![10, 20, 30, 40, 50]; // Another vector for demonstration.
    let filtered_squared_sum: i32 =
        numbers2.iter().filter(|&x| x > &20).map(|&x| x * x).sum();

    println!("Filtered squared sum: {}", filtered_squared_sum);

    // Use `enumerate` to get both index and value during iteration.
    println!("Enumerate:");
    for (index, value) in numbers2.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }

    // Use `zip` to combine two iterators (numbers and letters).
    let letters = vec!['a', 'b', 'c'];
    println!("Zip:");
    for (number, letter) in numbers2.iter().zip(letters.iter()) {
        println!("Number: {}, Letter: {}", number, letter);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
