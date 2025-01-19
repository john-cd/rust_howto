// ANCHOR: example
use subtle::Choice;
use subtle::ConditionallySelectable;
use subtle::ConstantTimeEq;
use subtle::CtOption;

// Pure-Rust traits and utilities for constant-time cryptographic
// implementations.
//
// This code is a low-level library, intended for implementing cryptographic
// protocols. It consists of a Choice type, and a collection of traits using
// Choice instead of bool, which are intended to execute in constant-time.
// The Choice type is a wrapper around a u8 that holds a 0 or 1.

fn main() {
    // Example 1: Constant-time equality
    let a = 10u8;
    let b = 10u8;
    let c = 20u8;

    let a_eq_b: Choice = a.ct_eq(&b);
    let a_eq_c: Choice = a.ct_eq(&c);

    println!("a == b: {}", a_eq_b.unwrap_u8() == 1); // 1 for true, 0 for false
    println!("a == c: {}", a_eq_c.unwrap_u8() == 1);

    // Example 2: Conditional selection
    let x = 5u32;
    let y = 10u32;

    let condition = Choice::from(1); // 1 means true
    let selected: u32 = u32::conditional_select(&x, &y, condition);
    println!("Selected (condition true): {}", selected); // Output: 10

    let condition = Choice::from(0); // 0 means false
    let selected: u32 = u32::conditional_select(&x, &y, condition);
    println!("Selected (condition false): {}", selected); // Output: 5

    // Example 3: CtOption for handling potential failures in constant time
    // The CtOption<T> type represents an optional value similar to the
    // Option<T> type but is intended for use in constant time APIs.
    fn try_parse(s: &str) -> CtOption<u32> {
        if s.len() < 3 {
            CtOption::new(0, Choice::from(0)) // Return 0 and a false choice if the string is too short
        } else {
            CtOption::new(s.len() as u32, Choice::from(1)) // Return length and a true choice otherwise
        }
    }

    let short_str = "ab";
    let long_str = "abcd";

    let short_result: CtOption<u32> = try_parse(short_str);
    let long_result: CtOption<u32> = try_parse(long_str);

    let is_some: Choice = long_result.is_some();
    let is_none: Choice = short_result.is_none();
    println!("Long string is some: {}", is_some.unwrap_u8() == 1);
    println!("Short string is none: {}", is_none.unwrap_u8() == 1);

    // Example 4: Using `unwrap_or` to expose the underlying value
    let opt: CtOption<u8> = CtOption::new(10u8, Choice::from(1));
    let val: u8 = opt.unwrap_or(0);
    println!("Unwrapped value: {}", val);

    let none_opt: CtOption<u8> = CtOption::new(5u8, Choice::from(0));
    let default_val = none_opt.unwrap_or(0);
    println!("Unwrapped default value: {}", default_val);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [P1](https://github.com/john-cd/rust_howto/issues/704)
