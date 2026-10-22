//! Variables: immutability, mutability, constants and statics.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]
#![allow(unused)]

// ANCHOR: example
/// Declares and modifies variables.
fn main() {
    // By default, variables are immutable.
    // Once we give the variable a value, the value won't change.
    // This default immutability helps prevent common programming errors,
    // especially in concurrent programming.
    let i = 5;
    // We cannot change the value of an immutable variable.
    // ERROR: i = 6;
    println!("i: {i}");

    // Rust's immutability is deep, not shallow:
    // it applies to the inner fields of a struct, tuple, array, etc.
    let s = MyStruct { flag: true };
    // s.data = false; // ERROR: cannot assign to `s.data`, as `s` is not
    // declared as mutable.
    println!("s: {s:?}");
    let t = (2, 2);
    // ERROR: t.0 = 1;
    println!("t: {t:?}");
    let arr = [1, 2, 3];
    // ERROR: arr[0] = 2;
    println!("arr: {arr:?}");

    // Declare a mutable variable with `let mut`:
    let mut st = String::new();
    // We can modify or reassign a mutable variable.
    st.push_str("42");
    st = "43".to_string();
    println!("st: {st}");
    // We cannot change its type:
    // ERROR: st = 42;
}

/// A struct with a flag.
#[derive(Debug)]
struct MyStruct {
    /// The flag.
    flag: bool,
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
