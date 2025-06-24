#![allow(dead_code)]
// ANCHOR: example
#![allow(clippy::unnecessary_literal_unwrap)]
//! Demonstrates the use of `unwrap`, `expect`, `unwrap_or`, and
//! `unwrap_or_else` with `Option`.

/// `unwrap()` and `expect()` retrieve the inner value of a `Option`.
/// BEWARE: if the `Option` is `None`, they will cause your program to panic.
///
/// You should only use `unwrap()` or `expect()` when you are absolutely certain
/// that the `Option` will be `Some`.
fn unwrap_expect() {
    let x = Some(10);
    let _value = x.unwrap(); // `value` is 10.

    // This will panic!
    // let y: Option<i32> = None;
    // let _another_value = y.unwrap();

    // `unwrap()` panics with a default message.
    // `expect("custom message")` panics with your specified custom message,
    // which is useful for debugging.
    // If calling `expect` is always safe, explain why in the message.
    let z = Some("A string");
    let _value =
        z.expect("Can't panic: the `Option` is guaranteed to be `Some`.");
}

fn unwrap_or_unwrap_or_else() {
    let config_setting = Some(100);
    let _ = config_setting.unwrap_or(50); // 100.

    let config_setting: Option<u32> = None;
    let _ = config_setting.unwrap_or(50); // 50.

    let config_setting: Option<u32> = None;
    let _ = config_setting.unwrap_or_default(); // 0.

    // `unwrap_or_else` is useful when the default value is expensive to
    // compute, and you only want to compute it if the `Option` is `None`.
    let config_setting: Option<u32> = None;
    let _ = config_setting.unwrap_or_else(|| {
        println!("Calculating expensive default...");
        75
    }); // 75.
}

fn main() {
    unwrap_expect();
    unwrap_or_unwrap_or_else();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
