#![allow(dead_code)]
// ANCHOR: example
//! `Rc` is a smart pointer that allows multiple owners of the same data.
//!
//! `Rc` stands for "Reference Counted". It keeps track of the number of owners
//! of the data. When the last owner is dropped, the data is deallocated.
use std::rc::Rc;

fn main() {
    let a = Rc::new(vec![1.0, 2.0, 3.0]);
    // The two syntaxes below are equivalent for cloning an `Rc`:
    let b = a.clone();
    let c = Rc::clone(&a); // Preferred syntax.
    // `b` and `c` now both point to the same memory location as `a`.

    // Gets the number of (`Rc`) pointers to this allocation.
    assert_eq!(3, Rc::strong_count(&a));
    // Dropping one of the pointers decrements the (strong) reference count.
    drop(c);
    assert_eq!(2, Rc::strong_count(&a));
    // Note that the inherent functionality of `Rc` is implemented as associated
    // functions, not methods, thus the `Rc::rc_func(&rc)` calls.

    // `Rc` is a smart pointer, so we can dereference it.
    println!("{:?}", *a);
    // The `.` operator lets you call methods on the underlying vector.
    b.iter().for_each(|x| print!("{x} "));

    // We can get a mutable reference to the inner value, if there are NO OTHER
    // `Rc` (or `Weak`) pointers to the same allocation. Returns `None`
    // otherwise. Consider also `make_mut`, which will clone the inner value
    // when there are other `Rc` pointers.
    let mut y = Rc::new(4);
    *Rc::get_mut(&mut y).unwrap() = 5;
    assert_eq!(*y, 5);

    // We can also consume the `Rc` to return its inner value if the `Rc` has
    // exactly one strong reference. See also `Rc::into_inner`.
    let x = Rc::new(6);
    assert_eq!(Rc::try_unwrap(x), Ok(6));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
