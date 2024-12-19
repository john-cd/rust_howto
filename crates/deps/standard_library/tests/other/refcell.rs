// ANCHOR: example
use std::cell::RefCell;

// The `RefCell` type in Rust is used for _interior mutability_,
// a pattern that allows you to mutate data even when there are immutable
// references to it. `RefCell` dynamically borrow checks at runtime,
// unlike Rust's standard borrowing rules (checks at compile time).
// Attempts to violate borrowing rules (like having multiple mutable borrows)
// will cause a panic at runtime. `RefCell` is single-threaded.
// The corresponding `Sync` version of `RefCell<T>` is `RwLock<T>`.
fn main() {
    // Create a RefCell containing a vector of integers
    let data = RefCell::new(vec![1, 2, 3, 4, 5]);

    // Borrow the data immutably
    {
        let borrowed_data = data.borrow();
        println!("Borrowed (immutable): {:?}", borrowed_data);
    } // The immutable borrow ends here

    // Borrow the data mutably and modify it
    {
        let mut borrowed_data = data.borrow_mut();
        borrowed_data.push(6);
        println!("Borrowed (mutable): {:?}", borrowed_data);
    } // The mutable borrow ends here

    // Borrow the data immutably again to check the modification
    {
        let borrowed_data = data.borrow();
        println!("Borrowed (immutable again): {:?}", borrowed_data);
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [P0](https://github.com/john-cd/rust_howto/issues/894)
