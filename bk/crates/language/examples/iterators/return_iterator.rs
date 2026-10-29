// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Returning an iterator from a function with `impl Iterator`.

/// Return an iterator from a function using the `impl Iterator<Item = T>`
/// syntax as the return type, where `T` is the type of the items the iterator
/// will yield.
fn count_up_to(max: i32) -> impl Iterator<Item = i32> {
    // In this case, we convert a range into an iterator,
    // then transform it further.
    (0..max).map(|x| x + 1)
}
// You could also return a boxed trait object e.g.,
// `Box<dyn Iterator<Item = T>>`.

/// Iterates over the numbers returned by `count_up_to`.
fn main() {
    let my_iterator = count_up_to(5);

    for number in my_iterator {
        println!("{number}");
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the return iterator example through its entry point.
    #[test]
    fn test_return_iterator_example_entry_point() {
        main();
    }

    /// The iterator counts from 1 to `max` inclusive.
    #[test]
    fn test_count_up_to() {
        assert_eq!(count_up_to(5).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
        assert_eq!(count_up_to(0).count(), 0);
        assert_eq!(count_up_to(-3).count(), 0);
    }
}
