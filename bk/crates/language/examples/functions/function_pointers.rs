#![allow(dead_code)]
// ANCHOR: example
/// Example function: add one to the input.
fn add_one(x: i32) -> i32 {
    x + 1
}

/// This function takes a function pointer in argument.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let func_ptr: fn(i32) -> i32 = add_one; // Assign a function pointer to a variable.
    println!("Calling via the pointer: {}", func_ptr(10)); // Output: 11.

    let answer = do_twice(add_one, 5); // Pass `add_one` as an argument.
    println!("The answer is: {}", answer); // Output: 12 ( (5+1) + (5+1) )
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
