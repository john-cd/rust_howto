#![allow(dead_code)]
// ANCHOR: example
//! Variable binding moves or copies the bound value by default.
//! Use `ref` or `ref mut` to get a reference instead.
//!
//! `ref` and `ref mut` are most often used in `match`, `if let`, and `while
//! let` expressions.

fn main() {
    // A sample tuple struct with an inner field that is not `Copy`:
    #[derive(Debug)]
    struct My(String);

    let my = My(String::from("Hello"));

    match my {
        My(string) => {
            // `string` is moved here.
            println!(
                "Type of `string`: {}",
                std::any::type_name_of_val(&string)
            );
        } // `string` is dropped.
    }
    // Hence `my` can't be used anymore.
    // println!("{my:?}");
    // error[E0382]: borrow of partially moved value: `my`.

    // To obtain a reference from a value instead of moving it,
    // you can use the `ref` keyword, which modifies the binding,
    // so that a reference is created for the value, then the reference is
    // assigned to the new variable.
    let mut robot_name = Some(String::from("Bender"));

    #[allow(clippy::single_match)]
    match robot_name {
        Some(ref name) => println!("Name (immutable ref): {name}"), /* `name` is `&String` */
        None => (),
    }

    // Use `ref mut` to obtain a mutable reference:
    if let Some(ref mut name_mut) = robot_name {
        name_mut.push_str(" Rodriguez"); /* `name_mut` is `&mut String` */
    }
    // `robot_name` remains available, since we did not move its contents.
    println!("Full name: {robot_name:?}"); // Some("Bender Rodriguez")

    // Note that `ref` works in `let` bindings as well, but its use is
    // discouraged:
    let value = 80;
    // UGLY: let ref port = value;
    // BETTER:
    let _port: &i32 = &value;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
