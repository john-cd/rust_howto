// ANCHOR: example
use std::cell::RefCell;

fn main() {
    // Create a RefCell containing a vector of integers
    let data = RefCell::new(vec![1, 2, 3, 4, 5]);

    // Borrow the data immutably
    {
        let borrowed_data = data.borrow();
        println!("Borrowed (immutable): {:?}", borrowed_data);

        // We can borrow the data immutably again
        let _borrowed_data2 = data.borrow();
    } // The immutable borrow ends here (when the `Ref` guard returned by `borrow` exits scope.)

    // Borrow the data mutably and modify it
    {
        let mut borrowed_data = data.borrow_mut();
        borrowed_data.push(6);
        println!("Borrowed (mutable): {:?}", borrowed_data);

        // We can't borrow the data again while it's borrowed mutably
        assert!(data.try_borrow().is_err());
        assert!(data.try_borrow_mut().is_err());
    } // The mutable borrow ends here

    // Borrow the data immutably again to check the modification
    {
        if let Ok(borrowed_data) = data.try_borrow() {
            println!("Borrowed (immutable again): {:?}", borrowed_data);
        }
    }

    // We can also consume the `RefCell`, returning the wrapped value.
    let _data = data.into_inner();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
