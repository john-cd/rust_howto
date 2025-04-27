// ANCHOR: example
//! This example demonstrates how to use the `use` keyword to bring items into
//! scope without having to write their full path.

/// Modules are used to organize code into logical units.
mod a {
    /// Nested module.
    /// We prefixed `mod b` with `pub` to make it public.
    pub mod b {
        /// Define a (public) function within a module.
        pub fn fn_in_b() {
            println!("in b!");
        }
    }
}

fn main() {
    // Call a function in module `b` using its full path.
    a::b::fn_in_b();

    // Bring the `b` module in scope.
    // A relative path starts from the current module and uses
    // `self`, or an identifier in the current module.
    use a::b;
    // You could also write: `use self::a::b;`.

    // Call a function using its shortened path.
    b::fn_in_b();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
