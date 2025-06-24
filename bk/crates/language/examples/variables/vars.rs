#![allow(dead_code)]
#![allow(unused)]
#![allow(clippy::needless_late_init)]
// ANCHOR: example
/// Variables are used to store values.
fn main() {
    // Rust is a statically typed language, meaning that
    // the type of every variable must be known at compile time.
    let a: i32 = -1;

    // However, Rust's compiler is very good at type inference,
    // so you often don't need to explicitly write out the type.
    let b = -2; // Infers an integer (`i32` by default).
    let c = 3.4; // Infers a float (`f64` by default).

    // It is possible to separate the declaration of a variable and its
    // assignment. However, the variable must be assigned before first use.
    let d;
    // println!("d: {}", d);
    // ERROR: Used binding `d` is possibly-uninitialized.
    d = "separate assignment";
    println!("d: {}", d);

    // Variables have block scope. This means a variable is valid from the point
    // it is declared until the end of the current code block (`{}`) it is
    // within.
    let outer_var = 10;
    println!("Outer var: {}", outer_var);
    {
        // Start of an inner scope.
        let inner_var = 20;
        println!("Inner var: {}", inner_var);
    } // End of inner scope - `inner_var` goes out of scope.
    // println!("Inner var outside inner scope: {}", inner_var); // ERROR.
    println!("Outer var outside inner scope: {}", outer_var);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
