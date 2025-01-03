// // ANCHOR: example
// use dhat::Dhat;
// use dhat::DhatAlloc;

// // We set DhatAlloc as the global allocator using the #[global_allocator]
// attribute. #[global_allocator]
// static ALLOC: DhatAlloc = DhatAlloc;

// fn main() {
//     // Start the heap profiling session
//     let _dhat = Dhat::start_heap_profiling();

//     // Example code to profile
//     // Create a vector and pushes 1000 elements into it.
//     let mut vec = Vec::new();
//     for i in 0..1000 {
//         vec.push(i);
//     }

//     println!("Memory profiling complete!");
//     // When you run this code, dhat will collect heap profiling information.
//     // The profiling data will be saved in a file named `dhat-heap.json` in
// the current directory.     // You can then use the `dhat` tool to analyze the
// profiling data. }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/746)
