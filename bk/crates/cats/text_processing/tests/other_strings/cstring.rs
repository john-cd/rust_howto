// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use std::ffi::CStr;
// use std::ffi::CString;
// use std::os::raw::c_char;

// // Example FFI function that takes a C string
// unsafe {
//     extern "C" {
//         fn strlen(s: *const c_char) -> usize;
//     }
// }

// fn main() {
//     // Creating a CString (owned C-compatible string)
//     let rust_string = "Hello, C world!";
//     let c_string = CString::new(rust_string).expect("CString creation
// failed");

//     // Get raw pointer for FFI calls
//     let raw_ptr: *const c_char = c_string.as_ptr();

//     unsafe {
//         // Convert back to a CStr (borrowed C string)
//         let c_str = CStr::from_ptr(raw_ptr);

//         // Convert CStr to Rust string
//         let back_to_rust = c_str.to_str().expect("Invalid UTF-8");
//         println!("Converted back: {}", back_to_rust);
//     }

//     // Safely call C function
//     let len = unsafe { strlen(c_string.as_ptr()) };
//     println!("String length according to C: {}", len);

//     // Working with null-terminated strings from C
//     unsafe {
//         // Simulating a string received from C code
//         // (In real code, this would come from an external C function)
//         let c_hello = b"Hello\0" as *const u8 as *const c_char;

//         // Safely wrap in CStr
//         let borrowed = CStr::from_ptr(c_hello);

//         // Convert to Rust String
//         let rust_str = borrowed.to_str().expect("Invalid UTF-8");
//         println!("From C: {}", rust_str);

//         // Create owned version
//         let owned = CString::from(borrowed);
//         println!("Owned: {:?}", owned);
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// [WIP finish](https://github.com/john-cd/rust_howto/issues/1139)
