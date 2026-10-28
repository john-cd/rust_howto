// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Different ways to concatenate strings.

/// Concatenates with the `+` operator.
fn concatenate_with_plus() {
    let a = String::from("hello");
    let b = " ";
    let c = String::from("world");
    let result: String = a + b + &c;
    // Note that `a` is moved here and can no longer be used.
    println!("{result}");
}

/// Concatenates with the `format!` macro.
fn concatenate_with_format() {
    let a = String::from("hello");
    let b = String::from("world");
    let result: String = format!("{a} {b}");
    println!("{result}");
}

/// Concatenates with `String::push_str`.
fn concatenate_with_push_str() {
    let mut a = String::from("hello");
    let b = " ";
    let c = "world";
    a.push_str(b);
    a.push_str(c);
    println!("{a}");
}

/// Concatenates strings after reserving their known combined byte length.
fn concatenate_with_capacity() {
    let parts = ["hello", " ", "world"];
    let capacity = parts.iter().map(|part| part.len()).sum();
    let mut result = String::with_capacity(capacity);
    for part in parts {
        result.push_str(part);
    }
    println!("{result}");
}

/// Concatenates with `join`.
fn concatenate_with_join() {
    let words = ["hello", "world"];
    let result: String = words.join(" ");
    println!("{result}");
}

/// Concatenates literals with the `concat!` macro.
fn concatenate_with_concat_macro() {
    let result: &str = concat!("hello", " ", "world");
    println!("{result}");
}

/// Concatenates with `concat`.
fn concatenate_with_concat() {
    let result: String = ["hello", " ", "world"].concat();
    println!("{result}");
}

use concat_string::concat_string;

// The `concat_string!` macro concatenates string slices into owned strings.
// concat_string! accepts any number of arguments that implement AsRef<str>
// and creates a String with the appropriate capacity.
/// Concatenates with the `concat_string!` macro.
fn concatenate_with_concat_string() {
    println!("{}", concat_string!("Hello", String::from(" "), "world"));
}

/// Concatenates with `String::push`.
fn concatenate_with_push() {
    let mut a = String::from("hello");
    let b = ' ';
    let c = 'w';
    // Appends the given `char` to the end of the `String`.
    a.push(b);
    a.push(c);
    println!("{a}");
}

/// Concatenates with `String::extend`.
#[allow(clippy::string_extend_chars)]
fn concatenate_with_extend() {
    let mut a = String::from("hello");
    let b = " world";
    a.extend(b.chars());
    println!("{a}");
}

/// Concatenates with `collect` on an iterator of strings.
fn concatenate_with_collect() {
    let words = ["hello", " ", "world"];
    let result: String = words.into_iter().collect();
    println!("{result}");
}

/// Concatenates with `collect` on an iterator of string slices.
fn concatenate_with_collect_iter() {
    let words = ["hello", " world"];
    let result: String = words.iter().copied().collect();
    println!("{result}");
}

use joinery::JoinableIterator as _;

/// Concatenates with the `joinery` crate.
fn concatenate_with_joinery() {
    let words = ["hello", "world"];

    // Join the words with a space separator.
    let result = words.iter().join_with(" ").to_string();

    println!("{result}");
}

use std::fmt::Write as _;

/// Concatenates with the `write!` macro.
fn concatenate_with_write_macro() {
    let part1 = "hello";
    let part2 = " world";
    let mut result = String::new();
    write!(&mut result, "{part1} {part2}")
        .expect("writing formatted text to a String cannot fail");
    println!("{result}");
}

/// Concatenates with `chain` on character iterators.
fn concatenate_with_chain() {
    let part1 = "hello";
    let part2 = " world";
    let result: String = part1.chars().chain(part2.chars()).collect();
    println!("{result}");
}

/// Runs each of the concatenation examples.
fn main() {
    concatenate_with_plus();
    concatenate_with_format();
    concatenate_with_push_str();
    concatenate_with_capacity();
    concatenate_with_join();
    concatenate_with_concat_macro();
    concatenate_with_concat();
    concatenate_with_concat_string();
    concatenate_with_push();
    concatenate_with_extend();
    concatenate_with_collect();
    concatenate_with_collect_iter();
    concatenate_with_joinery();
    concatenate_with_write_macro();
    concatenate_with_chain();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the string concat example through its entry point.
    #[test]
    fn test_string_concat_example_entry_point() {
        main();
    }
}
