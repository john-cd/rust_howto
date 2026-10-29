// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates the difference between taking a reference to a `Box` and using
//! `as_ref()`.

use std::boxed::Box;

/// Demonstrates the difference between taking a reference to a `Box` and using
/// `as_ref()`.
fn main() {
    let my_box: Box<String> = Box::new("hello from box".to_owned());

    // 1. Using `&my_box` - reference *to the Box*:
    #[allow(clippy::borrowed_box)]
    let ref_to_box: &Box<String> = &my_box;
    println!("Debug of `ref_to_box`: {ref_to_box:?}");
    // To access the inner string through `ref_to_box`, you'd need to
    // dereference twice:
    println!("Content via deref on `ref_to_box`: {}", **ref_to_box);

    // 2. Using `my_box.as_ref()` - reference *to the contained value*:
    let ref_to_contained_string: &String = my_box.as_ref();
    println!("Content via `as_ref()`: {ref_to_contained_string}");

    println!(
        "Content as `&str` via `Deref`: {}",
        &**ref_to_contained_string
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_pointer_asref_example_entry_point() {
        main();
    }

    #[test]
    fn test_box_as_ref_and_deref_give_the_same_content() {
        let my_box: Box<String> = Box::new("hello from box".to_owned());
        let via_as_ref: &String = my_box.as_ref();
        let via_deref: &str = via_as_ref.as_str();
        assert_eq!(via_as_ref, "hello from box");
        assert_eq!(via_deref, "hello from box");
        assert!(std::ptr::eq(via_as_ref, &*my_box));
    }
}
