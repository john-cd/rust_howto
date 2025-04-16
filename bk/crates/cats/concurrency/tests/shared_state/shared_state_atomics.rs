// ANCHOR: example
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

// Declare a static atomic counter to track the number of live threads.
static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
    println!("Live threads: {}", old_thread_count + 1);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [review](https://github.com/john-cd/rust_howto/issues/1349)
