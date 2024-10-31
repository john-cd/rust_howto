use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static GLOBAL_THREAD_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let old_thread_count = GLOBAL_THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
    println!("live threads: {}", old_thread_count + 1);
}

#[test]
fn test() {
    main();
}
