#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates the use of `as_ref`, `as_deref`, and `as_mut` methods on
//! `Result`.

fn get_message(success: bool) -> Result<String, &'static str> {
    if success {
        Ok(String::from("Rust"))
    } else {
        Err("Failed to get message")
    }
}

fn main() {
    let result: Result<String, &'static str> = get_message(true);

    // `as_ref` converts from `&Result<T, E>` to Result<&T, &E>`.
    let ref_result: Result<&String, &&str> = result.as_ref();
    println!("as_ref(): {:?}", ref_result);

    // Use `as_deref` to coerce the `Ok` variant of the original `Result` via
    // `Deref`. Here, `&String` is coerced to `&str`.
    let deref_result: Result<&str, &&str> = result.as_deref();
    println!("as_deref(): {:?}", deref_result);

    // Use `as_mut` to get a mutable reference:
    let mut result_mut = get_message(true);
    let mut_ref: Result<&mut String, &mut &str> = result_mut.as_mut();
    if let Ok(s) = mut_ref {
        s.push_str(" is awesome!");
    }
    println!("as_mut(): {:?}", result_mut);

    // There is also a `as_deref_mut` method.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
