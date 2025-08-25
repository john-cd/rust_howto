// ANCHOR: example
#![allow(unexpected_cfgs)]
#![allow(dead_code)]

// Define a very simple function to verify:
fn add(a: i32, b: i32) -> i32 {
    // For testing purposes, we make it fail for only 1 in billions of possible
    // inputs:
    if a == 42 {
        panic!("Oh no, a failing corner case!");
    }
    a + b // Note the possibility of overflow as well
}

// When Kani builds your code (using `cargo kani`), it
// - sets `cfg(kani)` for target crate compilation (including dependencies).
// - injects the kani crate.
// - sets `cfg(kani_host)` for host build targets, such as any build script and
//   procedural macro crates.
//
// The following Kani verification code is isolated in its own module and only
// compiled when Kani is in use.
#[cfg(kani)]
mod verification {

    use super::*;

    // Add a #[kani::proof] attributes to verification functions.
    // It can only be added to functions without parameters.
    //
    // Proof that adding zero to any number returns the same number:
    #[kani::proof]
    fn add_zero_identity() {
        // We use `kani::any()` to represent all possible `i32` values:
        let a = kani::any();
        let result = add(a, 0);
        // State the property that should be true:
        kani::assert(result == a, "Adding zero should not change the value");
    }

    // Proof that addition is indeed commutative (a + b = b + a):
    #[kani::proof]
    fn addition_is_commutative() {
        let a = kani::any();
        let b = kani::any();

        let result1 = add(a, b);
        let result2 = add(b, a);

        kani::assert(result1 == result2, "Addition should be commutative");
    }

    // Proof that adding two non-negative numbers results in a non-negative
    // number:
    #[kani::proof]
    fn add_is_non_negative() {
        let a = kani::any();
        // We set preconditions:
        // We assume that 'a' is non-negative.
        kani::assume(a >= 0);

        let b = kani::any();
        // Same for 'b'.
        kani::assume(b >= 0);

        let result = add(a, b);

        kani::assert(
            result >= 0,
            "Sum of two non-negative numbers should be non-negative",
        );
    }

    // Kani reports a number of failures, associated with overflows and
    // with the corner-case, which would be hard to detect via unit or property
    // testing alone, result in a failure:
    // Check 1: kani::estimate_size.assertion.1
    //          - Status: FAILURE
    //          - Description: "Oh no, a failing corner case!"
    //          - Location: crates/cats/development_tools/src/kani.rs:41:13 in
    //            function kani::estimate_size

    // By default, Kani only reports failures, not how the failure happened.
    // Kani offers an (experimental) concrete playback feature that provides
    // a concrete example of a value of x that triggers the failure.
}

fn main() {
    println!("2 + 3 = {}", add(2, 3));
}
// Examples adapted from the tutorial: <https://model-checking.github.io/kani/tutorial-first-steps.html>
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
