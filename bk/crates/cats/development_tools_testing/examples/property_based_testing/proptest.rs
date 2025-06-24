#![allow(dead_code)]
// ANCHOR: example

/// A simple function to test: finds the maximum of two integers:
fn max_of_two(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// A function to test: checks if a number is even:
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    // Tests the `max_of_two` function using property-based testing.
    //
    // This test checks two properties:
    // 1. The result is greater than or equal to both inputs.
    // 2. The result is equal to one of the inputs.
    proptest! {

        // Generate random integers in the range -1000 to 1000.
        #[test]
        fn test_max_of_two(a in -1000..1000, b in -1000..1000) {
            let result = max_of_two(a, b);

            // Property 1: Result should be greater than or equal to both inputs:
            prop_assert!(result >= a);
            prop_assert!(result >= b);

            // Property 2: Result should be equal to one of the inputs:
            prop_assert!(result == a || result == b);
        }
    }

    // Tests the `is_even` function using property-based testing.
    //
    // This test checks several properties:
    // 1. `is_even(n)` should be true if and only if `n % 2 == 0`.
    // 2. `is_even(n + 2)` should be equal to `is_even(n)`.
    // 3. `is_even(n * 2)` should always be true.
    proptest! {
        // Generate integers across a wide range.
        #[test]
        fn test_is_even(n in -10000..10000) {
            // Property 1: Even numbers should always return true:
            prop_assert_eq!(is_even(n), n % 2 == 0);

            // Property 2: Demonstrate properties of even/odd numbers:
            prop_assert_eq!(is_even(n + 2), is_even(n));
            prop_assert_eq!(is_even(n * 2), true);
        }
    }

    // Example of generating and testing more complex structures:
    //
    // This test checks the following property:
    //
    // After sorting a vector, all adjacent pairs of elements should be in
    // non-decreasing order.
    proptest! {
        /// Generate a vector of random integers.
        #[test]
        fn test_vec_operations(
            mut vec in prop::collection::vec(0..100, 1..20)
        ) {
            // Sort the vector.
            vec.sort();

            // Property tests on sorted vector.
            prop_assert!(vec.windows(2).all(|w| w[0] <= w[1]));
        }
    }
}

fn main() {
    println!("Max of 5 and 3: {}", max_of_two(5, 3));
    println!("Is 4 even? {}", is_even(4));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
