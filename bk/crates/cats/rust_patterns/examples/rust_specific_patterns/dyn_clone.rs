// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates how to clone trait objects using the `dyn_clone` crate.
//!
//! A trait object (`Box<dyn Trait>`) cannot be cloned directly, because `Clone`
//! requires knowing the concrete size. The `dyn_clone` crate adds a `clone_box`
//! mechanism so you can clone boxed trait objects.

use dyn_clone::DynClone;

// Define a trait named `MyTrait` that requires implementors to also implement
// `DynClone`.
/// A trait whose trait objects can be cloned.
trait MyTrait: DynClone {
    /// Prints the value.
    fn recite(&self);
}

// Implement `MyTrait` for `String``.
impl MyTrait for String {
    fn recite(&self) {
        println!("{self} ♫");
    }
}

/// Clones a boxed trait object and calls a method on both objects.
fn main() {
    let line = "The slithy structs did gyre and gimble the namespace";

    // Build a trait object holding a String.
    // This requires `String` to implement `MyTrait`.
    let original_trait_object: Box<dyn MyTrait> = Box::new(String::from(line));

    // Call the `MyTrait` function.
    original_trait_object.recite();

    // Clone the trait object.
    let cloned_trait_object: Box<dyn MyTrait> =
        dyn_clone::clone_box(&*original_trait_object);

    // Call the `MyTrait` function on the cloned object.
    cloned_trait_object.recite();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the dyn clone example through its entry point.
    #[test]
    fn test_dyn_clone_example_entry_point() {
        main();
    }

    /// A trait object that counts how many times it recited.
    #[derive(Clone)]
    struct Counter(std::rc::Rc<std::cell::Cell<u32>>);

    impl MyTrait for Counter {
        fn recite(&self) {
            self.0.set(self.0.get() + 1);
        }
    }

    /// A boxed trait object can be cloned, and the clone is usable after the
    /// original is dropped.
    #[test]
    fn boxed_trait_objects_can_be_cloned() {
        let count = std::rc::Rc::new(std::cell::Cell::new(0));
        let original: Box<dyn MyTrait> = Box::new(Counter(count.clone()));
        let cloned = dyn_clone::clone_box(&*original);
        original.recite();
        cloned.recite();
        drop(original);
        cloned.recite();
        assert_eq!(count.get(), 3);
    }

    /// The `String` implementation can be cloned too.
    #[test]
    fn strings_can_be_recited_after_cloning() {
        let original: Box<dyn MyTrait> = Box::new(String::from("line"));
        let cloned = dyn_clone::clone_box(&*original);
        original.recite();
        cloned.recite();
    }
}
