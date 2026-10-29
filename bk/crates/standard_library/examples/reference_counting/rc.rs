// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! `Rc` is a smart pointer that allows multiple owners of the same data.
//!
//! `Rc` stands for "Reference Counted". It keeps track of the number of owners
//! of the data. When the last owner is dropped, the data is deallocated.

use std::rc::Rc;

/// Shares a vector between several owners with `Rc` and counts the references.
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
    // The `.` operator lets us call methods on the underlying vector.
    b.iter().for_each(|x| print!("{x} "));

    // We can get a mutable reference to the inner value, if there are NO OTHER
    // `Rc` (or `Weak`) pointers to the same allocation. Returns `None`
    // otherwise. Consider also `make_mut`, which will clone the inner value
    // when there are other `Rc` pointers.
    let mut y = Rc::new(4);
    *Rc::get_mut(&mut y).expect("the Rc should be uniquely owned") = 5;
    assert_eq!(*y, 5);

    // We can also consume the `Rc` to return its inner value if the `Rc` has
    // exactly one strong reference. See also `Rc::into_inner`.
    let x = Rc::new(6);
    assert_eq!(Rc::try_unwrap(x), Ok(6));
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_example_entry_point() {
        main();
    }

    #[test]
    fn test_strong_count_follows_clones_and_drops() {
        let a = Rc::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(Rc::strong_count(&a), 1);
        let b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);
        assert!(Rc::ptr_eq(&a, &b));
        drop(b);
        assert_eq!(Rc::strong_count(&a), 1);
    }

    #[test]
    fn test_get_mut_requires_unique_ownership() {
        let mut y = Rc::new(4);
        *Rc::get_mut(&mut y).unwrap() = 5;
        assert_eq!(*y, 5);
        let _other = Rc::clone(&y);
        assert!(Rc::get_mut(&mut y).is_none());
    }

    #[test]
    fn test_try_unwrap() {
        assert_eq!(Rc::try_unwrap(Rc::new(6)), Ok(6));
        let shared = Rc::new(6);
        let other = Rc::clone(&shared);
        let err = Rc::try_unwrap(shared).unwrap_err();
        assert_eq!(*err, 6);
        drop(other);
    }

    #[test]
    fn test_weak_does_not_keep_the_value_alive() {
        let a = Rc::new(1);
        let weak = Rc::downgrade(&a);
        assert_eq!(weak.upgrade().as_deref(), Some(&1));
        drop(a);
        assert!(weak.upgrade().is_none());
    }
}
