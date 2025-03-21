// // ANCHOR: example
// use objc2::{class, msg_send, sel, sel_impl};

// fn main() {
//     unsafe {
//         let class_name = "NSString";
//         // Retrieve the Objective-C class object for the given class name
//         // ("NSString" in this case).
//         // NSString is a fundamental class in the
//         // Foundation framework of Apple's Cocoa and Cocoa Touch frameworks.
//         // It represents an immutable sequence of Unicode characters.
//         let class = class!(class_name);

//         // Send messages (invoke methods) to Objective-C objects.
//         let string = msg_send![class, alloc];
//         // Send the initWithString: message to the allocated object,
//         // initializing it with the given string.
//         let string = msg_send![string, initWithString: "Hello from Rust!"];
//         // Send the length message to the initialized string object, which
// returns its length.         let length = msg_send![string, length];
//         println!("String length: {}", length);
//     }
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// [finish; fix; https://docs.rs/objc2/latest/objc2/](https://github.com/john-cd/rust_howto/issues/1034)
