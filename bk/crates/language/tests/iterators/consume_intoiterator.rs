// ANCHOR: example

/// This function demonstrates the use of `IntoIterator` as a trait bound,
/// allowing it to accept various iterable types (like vectors, arrays, and
/// ranges) and compute the sum of their elements.
/// Note that all Iterators implement `IntoIterator` by just returning
/// themselves.
fn sum_all<I>(iterable: I) -> i32
where
    I: IntoIterator<Item = i32>,
{
    let mut sum = 0;
    for item in iterable.into_iter() {
        sum += item;
    }
    sum
}

fn main() {
    let numbers_vec = vec![1, 2, 3, 4, 5];
    let numbers_array = [10, 20, 30];

    println!("Sum of vector: {}", sum_all(numbers_vec)); // Output: Sum of vector: 15
    println!("Sum of array: {}", sum_all(numbers_array)); // Output: Sum of array: 60
    println!("Sum of range: {}", sum_all(0..5)); // Output: Sum of range: 10
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
