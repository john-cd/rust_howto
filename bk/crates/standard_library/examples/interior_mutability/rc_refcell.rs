#![allow(dead_code)]
// ANCHOR: example
//! A common pattern in Rust is using `Rc<RefCell<T>>` to allow multiple
//! owners of mutable data in a single-threaded scenario.

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Create a shared mutable vector
    let shared_vec: Rc<RefCell<Vec<i32>>> =
        Rc::new(RefCell::new(vec![1, 2, 3]));

    // Create another owner of the same data
    let owner2 = Rc::clone(&shared_vec);

    // Create yet another owner
    let owner3 = Rc::clone(&shared_vec);

    // Modify the data through the first owner
    shared_vec.borrow_mut().push(4);

    // Modify the data through the second owner
    owner2.borrow_mut().push(5);

    // Read the data through the third owner
    let read_data = owner3.borrow();
    println!("Shared vector contents: {:?}", *read_data);

    assert_eq!(*read_data, vec![1, 2, 3, 4, 5]);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
