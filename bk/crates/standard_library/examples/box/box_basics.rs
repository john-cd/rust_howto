// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Basic `Box` usage.

/// An example struct, which does not implement the `Copy` trait.
/// This means that when an instance of `MyStruct` is moved, the original
/// instance is no longer valid.
#[derive(Debug)]
struct MyStruct(i32);

/// Moves a value from the stack to the heap with a `Box`.
fn main() {
    // `in_stack` is an instance of `MyStruct` allocated on the stack.
    let in_stack = MyStruct(5);

    // Create a `Box`. This moves `in_stack` to the heap,
    // and `in_heap` becomes a pointer to this heap-allocated data.
    let in_heap: Box<MyStruct> = Box::new(in_stack);
    // `in_stack` is no longer valid because it has been moved into the `Box`.
    // ERROR: println!("{in_stack:?}");

    // Dereference the `Box` to get the value,
    // moving the value from the heap to the stack:
    let in_stack = *in_heap;
    println!("{in_stack:?} holds {}", in_stack.0);
    // ERROR: println!("{in_heap:?}"); // `in_heap` has been moved.

    // If the inner value is `Copy`:
    let x: u32 = 7;
    // `x` is copied; `y` owns a new `u32` on the heap.
    let y: Box<u32> = Box::new(x);
    println!("x: {x}, *y: {}", *y);

    // `Box` owns its value.
    // When `my_boxed_string` goes out of scope (is dropped),
    // the `String` on the heap is deallocated.
    {
        let my_boxed_string = Box::new(String::from("Hello, Box!"));
        println!("my_boxed_string: {my_boxed_string}");
    } // `my_boxed_string` goes out of scope here, and the memory is freed.
    println!("my_boxed_string is out of scope.");
    // ERROR: println!("my_boxed_string: {my_boxed_string}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_basics_example_entry_point() {
        main();
    }

    #[test]
    fn test_move_value_to_heap_and_back() {
        let in_heap: Box<MyStruct> = Box::new(MyStruct(5));
        assert_eq!(format!("{in_heap:?}"), "MyStruct(5)");
        // Dereferencing the box moves the value back to the stack.
        let in_stack = *in_heap;
        assert_eq!(format!("{in_stack:?}"), "MyStruct(5)");
    }

    #[test]
    fn test_box_of_copy_type_does_not_move_original() {
        let x: u32 = 7;
        let y = Box::new(x);
        assert_eq!(*y, x);
    }
}
