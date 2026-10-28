// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Convert a Rust string with the typed Objective-C Foundation bindings.

#[cfg(all(target_vendor = "apple", feature = "objc"))]
use objc2::rc::Retained;
#[cfg(all(target_vendor = "apple", feature = "objc"))]
use objc2_foundation::NSString;

/// Creates an owned Foundation string and checks its Rust round trip.
fn main() {
    #[cfg(all(target_vendor = "apple", feature = "objc"))]
    {
        // Foundation's typed constructor returns a strong reference, released
        // automatically when the Retained value leaves this scope.
        let string: Retained<NSString> = NSString::from_str("Hello from Rust!");
        assert_eq!(string.to_string(), "Hello from Rust!");
        println!("String length in UTF-16 units: {}", string.len_utf16());
    }
    #[cfg(not(all(target_vendor = "apple", feature = "objc")))]
    println!("Foundation example needs an Apple target and the objc feature.");
}
// ANCHOR_END: example

/// Runs the Foundation example or its platform fallback.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the objc2 example through its entry point.
    #[test]
    fn test_objc2_example_entry_point() {
        main();
    }
}
// [finish; fix; https://docs.rs/objc2/latest/objc2/](https://github.com/john-cd/rust_howto/issues/1034)
