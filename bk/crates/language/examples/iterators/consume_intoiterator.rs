// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Consuming any `IntoIterator` in a generic function.

/// This function demonstrates the use of `IntoIterator` as a trait bound,
/// allowing it to accept various iterators or iterable types (like vectors,
/// arrays, and ranges) and compute the sum of their elements.
/// Note that all Iterators implement `IntoIterator` by just returning
/// themselves.
fn sum_all<I>(iterable: I) -> i32
where
    I: IntoIterator<Item = i32>,
    // Additional bounds can be specified by restricting on `Item`.
{
    let mut sum = 0;
    for item in iterable {
        sum += item;
    }
    sum
}

/// Sums a vector and an array.
fn main() {
    let numbers_vec = vec![1, 2, 3, 4, 5];
    let numbers_array = [10, 20, 30];

    println!("Sum of vector: {}", sum_all(numbers_vec)); // Output: Sum of vector: 15.
    println!("Sum of array: {}", sum_all(numbers_array)); // Output: Sum of array: 60.
    println!("Sum of range: {}", sum_all(0..5)); // Output: Sum of range: 10.
    println!("Sum of iterator: {}", sum_all(std::iter::repeat_n(5, 2))); // Output: Sum of iterator: 10.
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the consume intoiterator example through its entry point.
    #[test]
    fn test_consume_intoiterator_example_entry_point() {
        main();
    }

    /// `sum_all` accepts any `IntoIterator` of `i32`.
    #[test]
    fn test_sum_all() {
        assert_eq!(sum_all(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_all([10, 20, 30]), 60);
        assert_eq!(sum_all(0..5), 10);
        assert_eq!(sum_all(std::iter::repeat_n(5, 2)), 10);
        assert_eq!(sum_all(Vec::new()), 0);
    }
}
