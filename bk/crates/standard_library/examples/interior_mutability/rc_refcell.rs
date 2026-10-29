// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A common pattern in Rust is using `Rc<RefCell<T>>` to allow multiple
//! owners of mutable data in a single-threaded scenario.

use std::cell::RefCell;
use std::rc::Rc;

/// Shares a mutable vector between several owners with `Rc<RefCell<_>>`.
fn main() {
    // Create a vector that can be shared and mutated through `RefCell`.
    let shared_vec: Rc<RefCell<Vec<i32>>> =
        Rc::new(RefCell::new(vec![1, 2, 3]));

    // Clone the `Rc` to create another owner of the same vector.
    let owner2 = Rc::clone(&shared_vec);

    // Add a third owner of the same vector.
    let owner3 = Rc::clone(&shared_vec);

    // Mutate the vector through the first owner.
    shared_vec.borrow_mut().push(4);

    // Mutate the same vector through the second owner.
    owner2.borrow_mut().push(5);

    // Borrow the shared vector through the third owner.
    let read_data = owner3.borrow();
    println!("Shared vector contents: {:?}", *read_data);

    assert_eq!(*read_data, vec![1, 2, 3, 4, 5]);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_refcell_example_entry_point() {
        main();
    }

    #[test]
    fn test_mutation_is_visible_to_all_owners() {
        let shared: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![1]));
        let other = Rc::clone(&shared);
        assert_eq!(Rc::strong_count(&shared), 2);
        other.borrow_mut().push(2);
        shared.borrow_mut().push(3);
        assert_eq!(*shared.borrow(), vec![1, 2, 3]);
        assert_eq!(*other.borrow(), vec![1, 2, 3]);
    }

    #[test]
    fn test_overlapping_borrows_are_detected_at_runtime() {
        let shared = Rc::new(RefCell::new(0));
        let reader = shared.borrow();
        assert!(shared.try_borrow_mut().is_err());
        drop(reader);
        assert!(shared.try_borrow_mut().is_ok());
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn test_borrow_mut_while_borrowed_panics() {
        let shared = Rc::new(RefCell::new(0));
        let _reader = shared.borrow();
        let _writer = shared.borrow_mut();
    }
}
