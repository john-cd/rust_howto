#![allow(dead_code)]
// ANCHOR: example
use std::io::Write;

/// Example of using the `print` and `println!` macros:
fn print() {
    // The first argument of `print` must be a string literal.
    print!("This prints the string literal. ");
    std::io::stdout().flush().unwrap(); // Emit message immediately, if stdout is line-buffered.

    println!("This prints a newline after the string.");

    // `print` and related macros accept `{...}` as placeholders.
    // Placeholders can refer to an argument or variable.
    // The empty `{}` means "the next argument".
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}.", y + 2);
}

/// Variants with `eprint`, `format` and `write`:
fn variants() {
    // `eprint` and `eprintln` write to `io::stderr` instead of `io::stdout`:
    eprintln!("Error: Could not complete task {}.", 42);

    // `format` creates a `String`:
    let info = format!("{0}-{1}", "First", "Second");
    println!("{info}");

    // `write` and `writeln` write formatted data into a writer.
    // The writer may be any value with a `write_fmt` method,
    // and usually implements the `fmt::Write` or `io::Write` traits.
    // Here this is a simple buffer String:
    use std::fmt::Write;
    let mut output = String::new();
    let z = "Rust";
    write!(&mut output, "Hello {z}!")
        .expect("Error occurred while trying to write to a String.");
}

/// `print` and friends call `format_args` under the covers.
/// They support a large number of placeholder formats:
fn format() {
    let name = "Alice";
    let age = 30;

    // Substitute variables into placeholders:
    println!("Hello, my name is {name} and I am {age} years old.");
    // Output: Hello, my name is Alice and I am 30 years old.

    // Use named arguments for clarity:
    print!(
        "{subject} is a wonderful city to live in. ",
        subject = String::from("Seattle")
    );
    // Output: Seattle is a wonderful city to live in.

    // Use positional arguments (less common but can be useful).
    let info = format!("{0} {1} {0}", "+", "-");
    println!("{info}"); // Output: + - +

    // You can format numbers with specific precision and alignment:
    let pi = std::f32::consts::PI;
    println!("Pi to two decimal places: {pi:.2}");
    // Output: Pi to two decimal places: 3.14.

    let number = 123;
    println!("Right aligned with width 10: {number:>10}");
    // Output: Right aligned with width 10:        123.

    // Print binary, hexadecimal...
    println!("Binary representation: {number:b}");
    // Output: Binary representation: 1111011.

    println!("Hexadecimal representation: {:x}", 255);
    // Output: Hexadecimal representation: ff.

    println!("Uppercase Hexadecimal: {:X}", 255);
    // Output: Uppercase Hexadecimal: FF.

    // If no format letter is specified (as in `{}` or `{:6}`),
    // `format_args` uses the `Display` trait.

    // `:?` and friends use the `Debug` trait.

    /// An example struct that implements the `Debug` trait.
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 10, y: 20 };
    println!("Debug output of Point: {p:?}"); // `p` must implement `Debug`.
    // Output: Debug output of Point: Point { x: 10, y: 20 }

    // Pretty debug formatting (using `:#?`):
    println!("Pretty debug output:\n{p:#?}");
    // Output:
    // Pretty debug output:
    // Point {
    //     x: 10,
    //     y: 20,
    // }

    // Prints and returns the value of a given expression for quick and dirty
    // debugging. Note that the `dbg!` macro works exactly the same in
    // release builds.
    let _q = dbg!(Point { x: 5, y: 5 });
}

fn main() {
    print();
    variants();
    format();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
