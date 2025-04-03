// ANCHOR: example
use parking_lot::Once;
use parking_lot::OnceState;

// Demonstrates the use of `parking_lot::Once` for one-time initialization.

// `Once` is a synchronization primitive which can be used to run a one-time
// initialization.
static INIT: Once = Once::new();

static mut VAL: usize = 0;

// This function will only call `expensive_computation` once, and will
// otherwise always return the value returned from the first invocation.
fn get_cached_val() -> usize {
    // Accessing a `static mut` is unsafe much of the time, unless we do so
    // in a synchronized fashion (e.g. write once or read all)
    unsafe {
        // The given closure will be executed if this is the first time
        // call_once has been called.
        INIT.call_once(|| {
            VAL = expensive_computation();
            println!("This is printed only once!");
        });
        // A closure has completed successfully.
        assert_eq!(INIT.state(), OnceState::Done);
        VAL
    }
}

fn expensive_computation() -> usize {
    // ...
    42
}

fn main() {
    // A closure has not been executed yet
    assert_eq!(INIT.state(), OnceState::New);
    for _ in 0..3 {
        assert_eq!(get_cached_val(), 42);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
