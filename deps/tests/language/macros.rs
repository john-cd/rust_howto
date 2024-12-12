#![allow(clippy::useless_vec)]
// ANCHOR: example
fn main() {
    // Macro used as an expression
    // Define a vector
    let _x = vec![1, 2, 3];

    // Macro used as a statement
    // Print the string
    println!("Hello!");

    // Macro definition
    macro_rules! pat {
        ($i:ident) => {
            Some($i)
        };
    }

    // Macro used in a pattern
    // Destructure an Option
    if let pat!(x) = Some(1) {
        assert_eq!(x, 1);
    }

    print_tuple((1, 3));
    use_thread_local();
}

macro_rules! Tuple {
    { $A:ty, $B:ty } => { ($A, $B) };
}

// Macro used in a type
// Define a tuple type
type T2 = Tuple!(i32, i32);

fn print_tuple(tupl: T2) {
    println!("{} {}", tupl.0, tupl.1);
}

fn use_thread_local() {
    use std::cell::RefCell;
    // Macro used as an item
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
}

macro_rules! const_maker {
    ($t:ty, $v:tt) => {
        const CONST: $t = $v;
    };
}

#[allow(dead_code)]
trait T {
    // Macro used as an associated item
    const_maker! {i32, 7}
}

// Macro calls within macros.
//
// When used, the outer macro `example` is expanded,
// then the inner macro `println` is expanded.
macro_rules! _example {
    () => {
        println!("Macro call in a macro!")
    };
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
