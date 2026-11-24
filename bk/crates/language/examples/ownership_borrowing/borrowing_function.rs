// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Borrows a struct so the caller can keep using it after the call.

/// A struct that does not implement `Copy`, so it has move semantics.
#[derive(Debug)]
struct MyStruct(
    /// Boolean state used by the borrowing and synchronization example.
    bool,
);

/// Prints `s`, which is borrowed immutably.
fn calculate(s: &MyStruct) {
    println!("{s:?}");
}

/// Borrows a value, passes the reference to a function, then uses the value
/// again.
fn main() {
    let s1 = MyStruct(true);
    // Borrow `s1` without transferring ownership.
    let ref_s1 = &s1;

    calculate(ref_s1);

    // The caller still owns `s1` after `calculate` returns.
    println!("{s1:?}");

    // Immutable references are `Copy`, thus `ref_s1` is also still valid:
    println!("{ref_s1:?}");
    println!("{}", ref_s1.0);
}
// ANCHOR_END: example

/// Borrows a value, passes the reference to a function, then uses the value again.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the borrowing function example through its entry point.
    #[test]
    fn test_borrowing_function_example_entry_point() {
        main();
    }

    /// Borrowing leaves the value usable.
    #[test]
    fn test_borrow_keeps_ownership() {
        let s = MyStruct(true);
        calculate(&s);
        calculate(&s);
        assert!(s.0);
    }
}
