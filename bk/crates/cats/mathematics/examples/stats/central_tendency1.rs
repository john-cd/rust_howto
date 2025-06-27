#![allow(dead_code)]
// ANCHOR: example
use std::cmp::Ordering;

/// Partitions a slice of integers into three parts:
/// - `left`: elements less than the pivot
/// - `pivot`: the first element of the slice
/// - `right`: elements greater than or equal to the pivot
///
/// Returns `None` if the slice is empty, otherwise returns `Some((left, pivot,
/// right))`.
fn partition(data: &[i32]) -> Option<(Vec<i32>, i32, Vec<i32>)> {
    match data.len() {
        // If the slice is empty, there's nothing to partition.
        // Return None to indicate this.
        0 => None,
        // Otherwise, proceed with partitioning.
        _ => {
            // Split the slice into the first element (pivot) and the rest
            // (tail).
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) =
                tail.iter().fold((vec![], vec![]), |mut splits, next| {
                    {
                        let &mut (ref mut left, ref mut right) = &mut splits;
                        if next < &pivot {
                            left.push(*next);
                        } else {
                            right.push(*next);
                        }
                    }
                    splits
                });

            Some((left, pivot, right))
        }
    }
}

/// Selects the k-th smallest element from a slice of integers.
///
/// This function uses the quickselect algorithm to find the k-th smallest
/// element in the slice. It returns `None` if the slice is empty or if `k` is
/// out of bounds.
///
/// # Arguments
///
/// * `data` - The slice of integers to search.
/// * `k` - The index of the element to select (0-based).
fn select(data: &[i32], k: usize) -> Option<i32> {
    // Partition the data around a pivot.
    let part = partition(data);

    // Handle the result of the partition.
    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        }
    }
}

/// Calculates the median of a slice of integers.
///
/// The median is the middle value in a sorted list of numbers. If the list has
/// an even number of elements, the median is the average of the two middle
/// numbers.
///
/// # Arguments
///
/// * `data` - The slice of integers to calculate the median from.
fn median(data: &[i32]) -> Option<f32> {
    // Get the size of the data slice.
    let size = data.len();

    // Check if the size is even or odd.
    match size {
        // If the size is even, calculate the average of the two middle
        // elements. The two middle elements are at indices (size / 2) -
        // 1 and size / 2. Use the `select` function to find these
        // elements.
        even if even % 2 == 0 => {
            let fst_med = select(data, (even / 2) - 1);
            let snd_med = select(data, even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some((fst + snd) as f32 / 2.0),
                _ => None,
            }
        }
        // If the size is odd, the median is the middle element.
        // The middle element is at index size / 2.
        odd => select(data, odd / 2).map(|x| x as f32),
    }
}

fn main() {
    let data = [3, 1, 6, 1, 5, 8, 1, 8, 10, 11];

    let part = partition(&data);
    println!("Partition is {part:?}");

    let sel = select(&data, 5);
    println!("Selection at ordered index 5 is {sel:?}");

    let med = median(&data);
    println!("Median is {med:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [REVIEW](https://github.com/john-cd/rust_howto/issues/1352)
