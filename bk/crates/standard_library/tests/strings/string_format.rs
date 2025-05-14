// ANCHOR: example

/// Example of using the `print` and `println!` macros.
fn print() {
    print!("This prints the string literal. ");
    println!("This prints a newline after the string.");
    let x = 5;
    let y = 10;
    // `print` and related macros accept { } as placeholders.
    println!("x = {x} and y + 2 = {}", y + 2);
}

/// `print` calls `format` under the covers:
fn format() {
    let name = "Alice";
    let age = 30;
    let city = String::from("Seattle");

    // Basic placeholder formatting.
    let greeting: String =
        format!("Hello, my name is {} and I am {} years old.", name, age);
    println!("{}", greeting); // Output: Hello, my name is Alice and I am 30 years old.

    // Named arguments for clarity.
    let description =
        format!("{subject} is a wonderful city to live in.", subject = city);
    println!("{}", description); // Output: Seattle is a wonderful city to live in.

    // Positional arguments (less common but can be useful).
    let info = format!("{0} {1} {0}", "+", "-");
    println!("{}", info); // Output: + - +

    // Formatting numbers with specific precision and alignment.
    let pi = std::f32::consts::PI;
    let formatted_pi = format!("Pi to two decimal places: {:.2}", pi);
    println!("{}", formatted_pi); // Output: Pi to two decimal places: 3.14.

    let number = 123;
    let aligned_number = format!("Right aligned with width 10: {:>10}", number);
    println!("{}", aligned_number); // Output: Right aligned with width 10:        123.

    let binary = format!("Binary representation: {:b}", number);
    println!("{}", binary); // Output: Binary representation: 1111011.

    let hex = format!("Hexadecimal representation: {:x}", 255);
    println!("{}", hex); // Output: Hexadecimal representation: ff.

    let hex_uppercase = format!("Uppercase Hexadecimal: {:X}", 255);
    println!("{}", hex_uppercase); // Output: Uppercase Hexadecimal: FF.

    // Debug formatting for structs and enums (using `:?`):

    /// An example struct that implements the `Debug` trait.
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 10, y: 20 };
    let debug_output = format!("Debug output of Point: {:?}", p); // `p` must implement `Debug`.
    println!("{}", debug_output); // Output: Debug output of Point: Point { x: 10, y: 20 }

    // Pretty debug formatting (using `:#?`):
    let pretty_debug = format!("Pretty debug output:\n{:#?}", p);
    println!("{}", pretty_debug);
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
    format();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
