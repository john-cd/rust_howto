#![allow(dead_code)]
// ANCHOR: example
/// Calculates the mean of a set of numbers.
fn main() {
    // Sample data set.
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    // Calculate the sum of the data.
    let sum = data.iter().sum::<i32>() as f32;
    // Get the number of elements in the data set.
    let count = data.len();

    let mean = match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    };

    println!("Mean of the data is {mean:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
