// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use ignore::WalkBuilder;
// use std::path::Path;

// // In `Cargo.toml`:
// // ignore = "0.4"

// fn main() {
//     // Specify the directory to walk
//     let root = Path::new("path/to/directory");

//     // Create a WalkBuilder to walk the directory
//     let walker = WalkBuilder::new(root)
//         .standard_filters(true)  // Respect .gitignore and other ignore files
//         .build();

//     // Iterate over the entries
//     for result in walker {
//         match result {
//             Ok(entry) => {
//                 let path = entry.path();
//                 if path.is_file() {
//                     println!("File: {:?}", path);
//                 } else if path.is_dir() {
//                     println!("Directory: {:?}", path);
//                 }
//             }
//             Err(err) => {
//                 eprintln!("Error: {}", err);
//             }
//         }
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [P0](https://github.com/john-cd/rust_howto/issues/762)
