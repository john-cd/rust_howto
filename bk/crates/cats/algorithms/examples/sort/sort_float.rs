// Reviewed 10-27-2026.

// ANCHOR: example
//! Sort floats using their IEEE 754 total ordering, including NaN.

/// Sort a vector of floats.
fn main() {
    let mut vec = vec![1.1, f64::NAN, 1.15, 5.5, 1.123, -0.0, 0.0];

    // `total_cmp` defines an ordering for all floating-point values, including
    // NaN and negative and positive zero.
    vec.sort_by(f64::total_cmp);

    assert_eq!(&vec[..5], &[-0.0, 0.0, 1.1, 1.123, 1.15]);
    assert_eq!(vec[5], 5.5);
    assert!(vec[6].is_nan());

    println!("{vec:?}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_float_example_entry_point() {
        main();
    }
}
