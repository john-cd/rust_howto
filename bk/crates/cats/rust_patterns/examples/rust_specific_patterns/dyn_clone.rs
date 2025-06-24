#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates how to clone trait objects using the `dyn_clone` crate.

use dyn_clone::DynClone;

// Define a trait named `MyTrait` that requires implementors to also implement
// `DynClone`.
trait MyTrait: DynClone {
    fn recite(&self);
}

// Implement `MyTrait` for `String``.
impl MyTrait for String {
    fn recite(&self) {
        println!("{} ♫", self);
    }
}

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

#[test]
fn test() {
    main();
}
