// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A slice is a view into a contiguous sequence of elements in a collection.
//! It does not own the data it points to.

/// Create slices from an array:
fn slice_from_array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // Create a slice referencing elements at indices 1, 2, and 3:
    let slice: &[i32] = &array[1..4];
    println!("{slice:?}"); // Output: [2, 3, 4].

    // `[..]` refers to the entire collection.
    let all: &[i32] = &array[..];
    println!("Entire array: {all:?}");

    // We can also coerce an array to a slice:
    let arr_slice: &[i32] = &[10, 20];
    println!("{arr_slice:?}");
}

/// Create slices from a `Vec`:
fn slice_from_vector() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];

    let slice: &[i32] = &vector[1..4]; // [2, 3, 4].
    println!("{slice:?}");

    // Mutable slice:
    let mutable_slice: &mut [i32] = &mut vector[2..];
    mutable_slice[0] = 10; // Modifies the original vector.
    println!("{mutable_slice:?}"); // Output: [10, 4, 5].
}

/// Creates slices from an array and from a vector.
fn main() {
    slice_from_array();
    slice_from_vector();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the slices example through its entry point.
    #[test]
    fn test_slices_example_entry_point() {
        main();
    }

    /// Both functions run, and slicing out of range panics.
    #[test]
    fn test_slice_functions_run() {
        slice_from_array();
        slice_from_vector();
    }

    /// Mutating a slice modifies the original vector.
    #[test]
    fn test_mutable_slice_writes_through() {
        let mut vector = vec![1, 2, 3, 4, 5];
        let mutable_slice = &mut vector[2..];
        mutable_slice[0] = 10;
        assert_eq!(vector, vec![1, 2, 10, 4, 5]);
    }

    /// Out-of-range slicing panics.
    #[test]
    #[should_panic(expected = "range end index 6 out of range")]
    fn test_slice_out_of_range() {
        let array = [1, 2, 3, 4, 5];
        let end = array.len() + 1;
        let _ = &array[1..end];
    }
}
