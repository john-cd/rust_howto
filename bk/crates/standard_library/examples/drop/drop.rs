// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Running code when a value is dropped, with the `Drop` trait.
//!
//! `Drop` runs code when a value goes out of scope, which is how Rust releases
//! files, locks and memory without a garbage collector. This idea is called
//! RAII (resource acquisition is initialization). Values are dropped in reverse
//! order of creation.

/// A struct that prints a message when it is dropped.
struct CustomStruct {
    /// Some data.
    data: String,
}

impl Drop for CustomStruct {
    // This is the only method required by the Drop trait. It takes a mutable
    // reference to self. The code inside this method will be executed when
    // an instance of CustomStruct is dropped.
    fn drop(&mut self) {
        println!("Dropping CustomStruct with data `{}`!", self.data);
    }
}

/// Creates and drops values.
fn main() {
    let _c = CustomStruct {
        data: String::from("my data"),
    };
    let _d = CustomStruct {
        data: String::from("another data"),
    };

    println!("CustomStructs created.");
    // `_c` and `_d` remain in scope until the end of `main`.

    let e = CustomStruct {
        data: String::from("explicit drop example"),
    };
    println!("CustomStruct 'e' created.");
    // We can explicitly drop a value using `std::mem::drop`, but usually,
    // Rust handles it automatically.
    drop(e);
    println!("CustomStruct 'e' dropped before end of main.");

    println!("End of main.");
    // After this statement, `_d` is dropped first, then `_c` (reverse
    // declaration order).
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_drop_example_entry_point() {
        main();
    }

    /// Same shape as `CustomStruct`, but records when it is dropped.
    struct Recorder {
        name: &'static str,
        log: Rc<RefCell<Vec<&'static str>>>,
    }

    impl Drop for Recorder {
        fn drop(&mut self) {
            self.log.borrow_mut().push(self.name);
        }
    }

    #[test]
    fn test_values_are_dropped_in_reverse_order_of_declaration() {
        let log = Rc::new(RefCell::new(Vec::new()));
        {
            let _c = Recorder {
                name: "c",
                log: Rc::clone(&log),
            };
            let _d = Recorder {
                name: "d",
                log: Rc::clone(&log),
            };
            assert!(log.borrow().is_empty());
        }
        assert_eq!(*log.borrow(), vec!["d", "c"]);
    }

    #[test]
    fn test_explicit_drop_runs_destructor_immediately() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let e = Recorder {
            name: "e",
            log: Rc::clone(&log),
        };
        assert!(log.borrow().is_empty());
        drop(e);
        assert_eq!(*log.borrow(), vec!["e"]);
    }

    #[test]
    fn test_custom_struct_can_be_created_and_dropped() {
        let c = CustomStruct {
            data: String::from("data"),
        };
        assert_eq!(c.data, "data");
        drop(c);
    }
}
