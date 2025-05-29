#![allow(unused)]
// ANCHOR: example
fn main() {
    // By default, variables are immutable.
    // Once we give the variable a value, the value won't change.
    // This default immutability helps prevent common programming errors,
    // especially in concurrent programming.
    let i = 5;
    // You cannot change the value of an immutable variable.
    // ERROR: i = 6;
    println!("i: {}", i);

    // Rust's immutability is deep, not shallow:
    // it applies to the inner fields of a struct, tuple, array, etc.
    let s = MyStruct { flag: true };
    // s.data = false; // ERROR: cannot assign to `s.data`, as `s` is not
    // declared as mutable.
    println!("s: {:?}", s);
    let t = (2, 2);
    // ERROR: t.0 = 1;
    println!("t: {:?}", t);
    let arr = [1, 2, 3];
    // ERROR: arr[0] = 2;
    println!("arr: {:?}", arr);

    // Declare a mutable variable with `let mut`:
    let mut st = String::new();
    // You can modify or reassign a mutable variable.
    st.push_str("42");
    st = "43".to_string();
    println!("st: {}", st);
    // You cannot change its type:
    // ERROR: st = 42;
}

#[derive(Debug)]
struct MyStruct {
    flag: bool,
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
