#![allow(dead_code)]
// ANCHOR: example
/// Calculates the mean (average) of a slice of i32 integers.
///
/// # Arguments
///
/// * `data` - A slice of i32 integers.
///
/// # Returns
///
/// Returns `Some(f32)` containing the mean if the slice is not empty, otherwise
/// returns `None`.
fn mean(data: &[i32]) -> Option<f32> {
    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

/// Calculates the standard deviation of a slice of i32 integers.
///
/// # Arguments
///
/// * `data` - A slice of i32 integers.
///
/// # Returns
///
/// Returns `Some(f32)` containing the standard deviation if the slice is not
/// empty, otherwise returns `None`.
fn std_deviation(data: &[i32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data
                .iter()
                .map(|value| {
                    let diff = data_mean - (*value as f32);

                    diff * diff
                })
                .sum::<f32>()
                / count as f32;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let data_mean = mean(&data);
    println!("Mean is {data_mean:?}");

    let data_std_deviation = std_deviation(&data);
    println!("Standard deviation is {data_std_deviation:?}");

    let zscore = match (data_mean, data_std_deviation) {
        (Some(mean), Some(std_deviation)) => {
            let diff = data[4] as f32 - mean;

            Some(diff / std_deviation)
        }
        _ => None,
    };
    println!(
        "Z-score of data at index 4 (with value {}) is {:?}",
        data[4], zscore
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
