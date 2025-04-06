// ANCHOR: example
use std::collections::HashMap;

/// Calculates the mode of a dataset.
///
/// The mode is the value that appears most frequently in a dataset.
fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    // Calculate the frequency of each value in the dataset.
    let frequencies = data.iter().fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1; // Increment the count for the current value.
        freqs
    });

    let mode = frequencies
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| *value);

    println!("Mode of the data is {:?}", mode);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
