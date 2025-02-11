// ANCHOR: example

// `dhat` provides heap profiling and ad-hoc profiling capabilities to Rust,
// similar to those provided by `DHAT`, a powerful heap profiler that
// comes with `Valgrind`. Warning: This crate is experimental.

// First, add to your `Cargo.toml`:
// [dependencies]
// dhat = "0.3.3"
//
// # You should only use `dhat` in release builds.
// [profile.release]
// debug = 1
//
// # Create features that lets you easily switch profiling on and off:
// [features]
// dhat-heap = []    # if you are doing heap profiling
// dhat-ad-hoc = []  # if you are doing ad hoc profiling

// The heap profiling works by using a global allocator that wraps the system
// allocator and tracks all heap allocations.
// We set `dhat::Alloc` as the global allocator
// using the #[global_allocator] attribute.
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

// When you run the code below with:
// cargo run --features dhat-heap
// `dhat` will collect heap profiling information.
// The profiling data will be saved in a file named `dhat-heap.json`
// in the current directory. You can then use e.g.
// https://nnethercote.github.io/dh_view/dh_view.html
// to analyze the profiling data.
// The profiler also prints to `stderr`, like:
// dhat: Total:     9,200 bytes in 10 blocks (note: "block" = allocation)
// dhat: At t-gmax: 5,120 bytes in 2 blocks
// dhat: At t-end:  1,024 bytes in 1 blocks
fn heap_profiling() {
    // Start the heap profiling session
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    // Example code to profile:
    // Create a vector and pushes 1000 elements into it.
    let mut vec = Vec::new();
    for i in 0..1000 {
        vec.push(i);
    }
    #[cfg(feature = "dhat-heap")]
    println!("Memory profiling complete!");
}

// Ad hoc profiling involves manually annotating hot code points.
// Run with: cargo run --features dhat-ad-hoc
// The profiler prints to `stderr`:
// dhat: Total:     100,000 units in 100,000 events
// dhat: The data has been saved to dhat-ad-hoc.json, and is viewable with
// dhat/dh_view.html
fn adhoc_profiling() {
    #[cfg(feature = "dhat-ad-hoc")]
    let _profiler = dhat::Profiler::new_ad_hoc();

    fn hot_function() {
        #[cfg(feature = "dhat-ad-hoc")]
        dhat::ad_hoc_event(1);
    }

    for _ in 0..100000 {
        hot_function();
    }
}

pub fn main() {
    heap_profiling();
    adhoc_profiling();
}

// `dhat` also supports heap usage testing, where you can write tests and then
// check that they allocated as much heap memory as you expected.
#[test]
fn heap_usage_testing() {
    // `testing()` allows the use of dhat::assert! and related macros,
    // and disables saving of profile data on Profiler drop.
    let _profiler = dhat::Profiler::builder().testing().build();

    let _v1: Vec<i32> = vec![1, 2, 3, 4];
    let v2: Vec<u8> = vec![5, 6, 7, 8];
    drop(v2);

    let stats = dhat::HeapStats::get();

    // Total allocations and number of bytes:
    // dhat::assert_eq!(stats.total_blocks, 2);
    dhat::assert_eq!(stats.total_bytes, 20);

    // Allocations and number of bytes at the point of peak heap size:
    // dhat::assert_eq!(stats.max_blocks, 2);
    dhat::assert_eq!(stats.max_bytes, 20);

    // Now a single allocation remains alive.
    // dhat::assert_eq!(stats.curr_blocks, 1);
    dhat::assert_eq!(stats.curr_bytes, 16);
}
// Example adapted from https://docs.rs/dhat/latest/dhat/
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [P1](https://github.com/john-cd/rust_howto/issues/746) automate: cargo run --features dhat-heap / cargo run --features dhat-ad-hoc
