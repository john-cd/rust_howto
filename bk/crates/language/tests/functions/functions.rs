// ANCHOR: example

/// Function with no parameters, no return type.
fn say_hello() {
    println!("Hello, Rust!");
}

/// Function with one parameter called `name` of type `&str` (string slice).
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with two parameters: `a` and `b`, both of type `i32`.
fn add_and_display(a: i32, b: i32) {
    let sum = a + b;
    println!("{} + {} = {}", a, b, sum);
}

/// Function that takes two `i32` values and returns their sum as an `i32`.
fn multiply(x: i32, y: i32) -> i32 {
    x * y // No semicolon: this expression's value is returned.
}

/// Function bodies follow the same rules than blocks:
fn foo(x: i32) -> i32 {
    let y = {
        let z = 3;
        x + z // Blocks return the value of their last expression, if there is no semi-colon.
    };
    y // Similarly, functions return the value of their last expression, if there is no semi-colon.
}

/// Function that implicitly returns the unit type `()`.
/// This is equivalent to `fn print_coordinates(x: i32, y: i32) -> ()`.
fn print_coordinates(x: i32, y: i32) {
    println!("Coordinates: ({}, {})", x, y); // Note the semicolon.
    // No last expression needed to return `()`.
}

/// Function using an explicit `return` keyword (rarely needed).
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num); // Return early, breaking out of the `for` loop.
        }
    }
    None // Return `None` if no even number is found (last expression).
}

fn main() {
    // Call the functions above.
    say_hello();
    greet("Rustacean");
    add_and_display(5, 3);
    println!("2 * 3 = {}", multiply(2, 3));
    println!("Foo: {}", foo(1));
    print_coordinates(1, 2);
    println!(
        "First even number: {:?}",
        find_first_even(vec![1, 2, 3].as_slice())
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
