// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the use of `unwrap`, `expect`, `unwrap_or`, and
//! `unwrap_or_else` with `Option`.

#![allow(clippy::unnecessary_literal_unwrap)]

/// `unwrap()` and `expect()` retrieve the inner value of a `Option`.
/// BEWARE: if the `Option` is `None`, they will cause your program to panic.
///
/// You should only use `unwrap()` or `expect()` when we are absolutely certain
/// that the `Option` will be `Some`.
fn unwrap_expect() {
    let x = Some(10);
    let _value = x.unwrap(); // `value` is 10.

    // This will panic!
    // let y: Option<i32> = None;
    // let _another_value = y.unwrap();

    // `unwrap()` panics with a default message.
    // `expect("custom message")` panics with the specified custom message,
    // which is useful for debugging.
    // If calling `expect` is always safe, explain why in the message.
    let z = Some("A string");
    let _value =
        z.expect("Can't panic: the `Option` is guaranteed to be `Some`.");
}

/// Provides default values with `unwrap_or`, `unwrap_or_default` and
/// `unwrap_or_else`.
fn unwrap_or_unwrap_or_else() {
    let config_setting = Some(100);
    let _ = config_setting.unwrap_or(50); // 100.

    let config_setting: Option<u32> = None;
    let _ = config_setting.unwrap_or(50); // 50.

    let config_setting: Option<u32> = None;
    let _ = config_setting.unwrap_or_default(); // 0.

    // `unwrap_or_else` is useful when the default value is expensive to
    // compute, and we only want to compute it if the `Option` is `None`.
    let config_setting: Option<u32> = None;
    let _ = config_setting.unwrap_or_else(|| {
        println!("Calculating expensive default...");
        75
    }); // 75.
}

/// Runs the `unwrap` examples.
fn main() {
    unwrap_expect();
    unwrap_or_unwrap_or_else();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_unwrap_example_entry_point() {
        main();
    }

    #[test]
    fn test_unwrap_on_some() {
        assert_eq!(Some(10).unwrap(), 10);
        assert_eq!(Some("A string").expect("is Some"), "A string");
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_unwrap_on_none_panics() {
        let x: Option<i32> = None;
        x.unwrap();
    }

    #[test]
    #[should_panic(expected = "it was None")]
    fn test_expect_on_none_panics_with_message() {
        let x: Option<i32> = None;
        x.expect("it was None");
    }

    #[test]
    fn test_unwrap_or_variants() {
        assert_eq!(Some(100).unwrap_or(50), 100);
        assert_eq!(None::<u32>.unwrap_or(50), 50);
        assert_eq!(None::<u32>.unwrap_or_default(), 0);
        let calls = std::cell::Cell::new(0);
        let value = None::<u32>.unwrap_or_else(|| {
            calls.set(calls.get() + 1);
            75
        });
        assert_eq!((value, calls.get()), (75, 1));
        // The closure is not evaluated when the value is present.
        assert_eq!(Some(1).unwrap_or_else(|| unreachable!()), 1);
    }
}
