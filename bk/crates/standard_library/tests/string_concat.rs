#![allow(clippy::vec_init_then_push)]
#![allow(clippy::useless_vec)]
// ANCHOR: example
//! This example demonstrates various ways to concatenate strings in Rust.
//! It includes examples using `concat`, `join`, `format`, `push_str`, `+`, and
//! external macros.
#[macro_use(concat_string)]
extern crate concat_string;

#[macro_use(concat_strs)]
extern crate concat_strs;

static DATE: &str = "2024-01-15";
static T: &str = "T";
static TIME: &str = "12:00:09Z";

fn main() {
    // Using `concat` on a slice of string slices.
    let _datetime = &[DATE, T, TIME].concat();

    // Using `join` with a separator on a slice of string slices.
    let _datetime = &[DATE, TIME].join(T);

    // Using `join` with an empty separator on a slice of string slices.
    let _datetime = &[DATE, T, TIME].join("");

    // Using `join` with an empty separator on a slice of string slices.
    let _datetime = &[DATE, T, TIME].join("");

    // Using `iter().copied().collect()` to concatenate a slice of string
    // slices.
    let list = [DATE, T, TIME];
    // let _datetime: String = list.iter().map(|x| *x).collect();
    let _datetime: String = list.iter().copied().collect();

    // Using `iter().copied().collect()` to concatenate a vector of string
    // slices.
    let list = vec![DATE, T, TIME];
    // let _datetime: String = list.iter().map(|x| *x).collect();
    let _datetime: String = list.iter().copied().collect();

    // Using `format!` macro with positional arguments.
    let _datetime = &format!("{}{}{}", DATE, T, TIME);

    // Using `format!` macro with named arguments (Rust 1.58+).
    let _datetime = &format!("{DATE}{T}{TIME}");

    // Using `push_str` to build a string incrementally.
    let mut datetime = String::new();
    datetime.push_str(DATE);
    datetime.push_str(T);
    datetime.push_str(TIME);

    // Using `push` on a vector of strings and then `join`.
    let mut datetime = Vec::<String>::new();
    datetime.push(String::from(DATE));
    datetime.push(String::from(T));
    datetime.push(String::from(TIME));
    let _datetime = datetime.join("");

    // Using `push_str` with pre-allocated capacity.
    let mut datetime = String::with_capacity(20);
    datetime.push_str(DATE);
    datetime.push_str(T); // or 'T'
    datetime.push_str(TIME);

    // Using `+` operator with `String::from` for each part.
    let _datetime =
        &(String::from(DATE) + &String::from(T) + &String::from(TIME));

    // Using `+` operator with `String::from` for the first part and string
    // slices for the rest.
    let _datetime = &(String::from(DATE) + T + TIME);

    // Using `+` operator with `to_owned` for the first part and string slices
    // for the rest.
    let _datetime = &(DATE.to_owned() + T + TIME);

    // Using `+` operator with `to_string` for the first part and string slices
    // for the rest.
    let _datetime = &(DATE.to_string() + T + TIME);

    // Using the `concat_string!` macro.
    let _datetime = concat_string!(DATE, T, TIME);

    // Using the `concat_strs!` macro.
    let datetime = &concat_strs!(DATE, T, TIME);

    println!("{}", datetime);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
