#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to use the `windows` crate to call the
// //! `MessageBoxW` function from the Windows API.
// //! It displays a simple message box with the text "Hello, windows!" and
// //! the title "Greetings".
// #![cfg(target_os = "windows")]

// use windows::Win32::Foundation::HWND;
// use windows::Win32::Foundation::PWSTR;
// use windows::Win32::UI::WindowsAndMessaging::MB_OK;
// use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;

// /// Simple Windows application that displays a message box
// /// saying "Hello, windows!".
// fn main() {
//     unsafe {
//         MessageBoxW(
//             HWND(0),
//             PWSTR(
//                 "Hello, windows!"
//                     .encode_utf16()
//                     .collect::<Vec<u16>>()
//                     .as_mut_ptr(),
//             ),
//             PWSTR(
//                 "Greetings"
//                     .encode_utf16()
//                     .collect::<Vec<u16>>()
//                     .as_mut_ptr(),
//             ),
//             MB_OK,
//         );
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish NOW](https://github.com/john-cd/rust_howto/issues/823)
