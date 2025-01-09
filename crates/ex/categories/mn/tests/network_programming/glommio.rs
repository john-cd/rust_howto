// // ANCHOR: example
// use std::fs::File;
// use std::io::Write;

// use glommio::Latency;
// use glommio::LocalExecutorBuilder;
// use glommio::Placement;
// use glommio::Shares;
// use glommio::Task;
// use glommio::executor;

// // Asynchronous thread-per-core applications in Rust.
// //
// // `glommio` is a library providing a safe Rust interface for asynchronous,
// // thread-local I/O, based on the Linux `io_uring` interface and Rust's async
// // support.
// // `glommio` is Linux-only, with a kernel version 5.8 or newer recommended.

// fn main() {
//     LocalExecutorBuilder::new()
//         .spawn(|| async move {
//             // Spawn a new LocalExecutor in a new thread with a given task.
//             // Create a new file or open an existing one
//             let mut file = File::create("temp/example.txt")
//                 .expect("Failed to create file");

//             // Write some data to the file
//             file.write_all(b"Hello, Glommio!")
//                 .expect("Failed to write to file");
//         })
//         .unwrap();

// // Although pinned threads are not required for use of glommio, by
// // creating N executors and binding each to a specific CPU one can use
// // this crate to implement a thread-per-core system where context
// // switches essentially never happen, allowing much higher efficiency.
// // This will now never leave CPU 0.
//     LocalExecutorBuilder::new(Placement::Fixed(0))
//         .spawn(|| async move {
//  // For a Thread-per-core system to work well, it is paramount
//  // that some form of scheduling can happen within the thread.
//  // Traditional applications use many threads to divide
//  // the many aspects of its workload and rely on the
//  // operating system and runtime to schedule these threads fairly
//  // and switch between these as necessary. For a
//  // thread-per-core system, each thread must handle its
//  // own scheduling at the application level. This example
//  // creates two task queues: tq1 has 2 shares, tq2 has 1 share.
//  // This means that if both want to use the CPU to its maximum,
//  // tq1 will have 2/3 of the CPU time (2 / (1 + 2)) and tq2 will
//  // have 1/3 of the CPU time. Those shares are dynamic and can be
//  // changed at any time.
//             let tq1 = executor().create_task_queue(
//                 Shares::Static(2),
//                 Latency::NotImportant,
//                 "test1",
//             );
//             let tq2 = executor().create_task_queue(
//                 Shares::Static(1),
//                 Latency::NotImportant,
//                 "test2",
//             );
//             let t1 = glommio::spawn_local_into(
//                 async move {
//                     // your code here
//                 },
//                 tq1,
//             )
//             .unwrap();
//             let t2 = glommio::spawn_local_into(
//                 async move {
//                     // your code here
//                 },
//                 tq2,
//             )
//             .unwrap();

//             t1.await;
//             t2.await;
//         })
//         .unwrap();
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/810)
// // review https://itnext.io/modern-storage-is-plenty-fast-it-is-the-apis-that-are-bad-6a68319fbc1a
