#![allow(dead_code)]
// ANCHOR: example
//! Basic `Box` usage.

/// An example struct, which does not implement the `Copy` trait.
/// This means that when an instance of `MyStruct` is moved, the original
/// instance is no longer valid.
#[derive(Debug)]
struct MyStruct(i32);

fn main() {
    // `in_stack` is an instance of `MyStruct` allocated on the stack.
    let in_stack = MyStruct(5);

    // Create a `Box`. This moves `in_stack` to the heap,
    // and `in_heap` becomes a pointer to this heap-allocated data.
    let in_heap: Box<MyStruct> = Box::new(in_stack);
    // `in_stack` is no longer valid because it has been moved into the `Box`.
    // ERROR: println!("{:?}", in_stack);

    // Dereference the `Box` to get the value,
    // moving the value from the heap to the stack:
    let in_stack = *in_heap;
    println!("{:?}", in_stack);
    // ERROR: println!("{:?}", in_heap); // `in_heap` has been moved.

    // If the inner value is `Copy`:
    let x: u32 = 7;
    // `x` is copied, `y` owns a new `i32` on the heap.
    let y: Box<u32> = Box::new(x);
    println!("x: {}, *y: {}", x, *y);

    // `Box` owns its value.
    // When `my_boxed_string` goes out of scope (is dropped),
    // the String on the heap is deallocated.
    {
        let my_boxed_string = Box::new(String::from("Hello, Box!"));
        println!("my_boxed_string: {}", my_boxed_string);
    } // `my_boxed_string` goes out of scope here, and the memory is freed.
    println!("my_boxed_string is out of scope.");
    // ERROR: println!("my_boxed_string: {}", my_boxed_string);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
