#![allow(dead_code)]
// ANCHOR: example
#[cfg(any(target_os = "macos", target_os = "ios"))]
use objc2::class;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use objc2::msg_send;

/// The main function where the Objective-C interaction takes place.
/// This function uses `unsafe` because it interacts with raw pointers and
/// foreign code.
fn main() {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    unsafe {
        let class_name = "NSString";
        // Retrieve the Objective-C class object for the given class name
        // ("NSString" in this case).
        // NSString is a fundamental class in the
        // Foundation framework of Apple's Cocoa and Cocoa Touch frameworks.
        // It represents an immutable sequence of Unicode characters.
        let class = class!(class_name);

        // Send messages (invoke methods) to Objective-C objects.
        let string: *mut objc2::runtime::AnyObject = msg_send![class, alloc];
        // Send the initWithString: message to the allocated object,
        // initializing it with the given string.
        // Note: in actual objc2 this would need more setup but this represents
        // the concept let string: *mut objc2::runtime::AnyObject =
        // msg_send![string, initWithString: "Hello from Rust!"];
        // Send the length message to the initialized string object, which
        // returns its length.
        // let length: usize = msg_send![string, length];
        // println!("String length: {length}");
        let _ = string;
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
// [finish; fix; https://docs.rs/objc2/latest/objc2/](https://github.com/john-cd/rust_howto/issues/1034)
