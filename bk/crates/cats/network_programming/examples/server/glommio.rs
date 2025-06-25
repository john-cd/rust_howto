#![allow(dead_code)]
// ANCHOR: example
#![cfg(target_os = "linux")]
//! This example demonstrates the usage of the `glommio` library for
//! asynchronous I/O operations in Rust.
//!
//! `glommio` is a library providing a safe Rust interface for asynchronous,
//! thread-local I/O, based on the Linux `io_uring` interface plus Rust's async
//! support. `glommio` is therefore Linux-only, with a kernel version 5.8 or
//! newer recommended.

use std::fs::File;
use std::io::Write;

// use glommio::prelude::*;
use glommio::Latency;
use glommio::LocalExecutorBuilder;
use glommio::Placement;
use glommio::Shares;
// use glommio::Task;
use glommio::executor;

/// The `main` function demonstrates the basic usage of `glommio` for file I/O
/// and task scheduling. It creates a file, writes data to it, and then
/// demonstrates how to use task queues for cooperative multitasking.
fn main() {
    // Spawn a new LocalExecutor in a new thread with a given task.
    LocalExecutorBuilder::default()
        .spawn(|| async move {
            // Create a new file or open an existing one:
            let mut file = File::create("temp/example3.txt")
                .expect("Failed to create file");

            // Write some data to the file:
            file.write_all(b"Hello, Glommio!")
                .expect("Failed to write to file");
        })
        .expect("Failed to spawn local executor");

    // Although pinned threads are not required for `glommio`,
    // by creating N executors and binding each to a specific CPU,
    // one can use this crate to implement a cooperative thread-per-core system,
    // where context switches essentially never happen,
    // allowing much higher efficiency.
    // The following will now never leave CPU 0.
    LocalExecutorBuilder::new(Placement::Fixed(0))
        .spawn(|| async move {
            // For a thread-per-core system to work well, it is paramount
            // that some form of scheduling can happen within the thread.
            // Traditional applications use many threads to divide
            // the many aspects of its workload and rely on the
            // operating system and runtime to schedule these threads fairly
            // and switch between these as necessary.
            // For a thread-per-core system, each thread must handle its
            // own scheduling at the application level. This example
            // creates two task queues: tq1 has 2 shares, tq2 has 1 share.
            // This means that if both want to use the CPU to its maximum,
            // tq1 will have 2/3 of the CPU time (2 / (1 + 2)) and tq2 will
            // have 1/3 of the CPU time. Those shares are dynamic and can be
            // changed at any time.
            let tq1 = executor().create_task_queue(
                Shares::Static(2),
                Latency::NotImportant,
                "test1",
            );
            let tq2 = executor().create_task_queue(
                Shares::Static(1),
                Latency::NotImportant,
                "test2",
            );
            let t1 = glommio::spawn_local_into(
                async move {
                    // your code here
                },
                tq1,
            )
            .unwrap();
            let t2 = glommio::spawn_local_into(
                async move {
                    // your code here
                },
                tq2,
            )
            .unwrap();

            t1.await;
            t2.await;
        })
        .unwrap();
    println!("glommio example finished");
}
// Example adapted from <https://docs.rs/glommio/latest/glommio/>
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [finish](https://github.com/john-cd/rust_howto/issues/810)
// review <https://itnext.io/modern-storage-is-plenty-fast-it-is-the-apis-that-are-bad-6a68319fbc1a>
// <https://www.datadoghq.com/blog/engineering/introducing-glommio/>
// <https://docs.rs/glommio/latest/glommio/>
// <https://github.com/DataDog/glommio>
