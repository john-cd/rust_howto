#![allow(dead_code)]
// ANCHOR: example
//! `RefCell<T>` is a smart pointer that provides interior mutability.
//! It allows you to mutate data even when there are immutable references to it.
//! It enforces borrowing rules _at runtime_, panicking if they are violated.
//!
//! `RefCell<T>` is useful when you need to modify data that is behind an
//! immutable reference.

use std::cell::RefCell;

fn main() {
    // Create a RefCell containing a vector of integers.
    let data = RefCell::new(vec![1, 2, 3, 4, 5]);

    // Borrow the data immutably.
    {
        let borrowed_data = data.borrow();
        println!("Borrowed (immutable): {borrowed_data:?}");

        // Multiple immutable borrows are allowed.
        // We can borrow the data immutably again here.
        let _borrowed_data2 = data.borrow();
    }
    // The immutable borrow ends here (when the `Ref` guard returned by `borrow`
    // exits scope.)

    // Borrow the data mutably and modify it.
    {
        let mut borrowed_data = data.borrow_mut();
        borrowed_data.push(6);
        println!("Borrowed (mutable): {borrowed_data:?}");

        // Only one mutable borrow is allowed at a time.
        // We can't borrow the data again while it's borrowed mutably. This
        // would panic if we used `borrow` or `borrow_mut`.
        assert!(data.try_borrow().is_err());
        assert!(data.try_borrow_mut().is_err());
    }
    // The mutable borrow ends here.

    // Borrow the data immutably again to check the modification.
    {
        if let Ok(borrowed_data) = data.try_borrow() {
            println!("Borrowed (immutable again): {borrowed_data:?}");
        }
        // `try_borrow` returns a `Result` to handle the case where the data is
        // already borrowed mutably.
    }

    // We can also consume the `RefCell`, returning the wrapped value.
    let _data = data.into_inner();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
