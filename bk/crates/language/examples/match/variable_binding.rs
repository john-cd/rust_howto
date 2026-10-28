// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Binding the content of an `Option` to a variable with `if let`.

/// Binds the value inside an `Option`.
fn main() {
    let x: Option<i32> = Some(5);

    if let Some(val) = x {
        // Declares a `val` variable and binds it to the inner value of `x`.
        println!("Got a value: {val}");
    }
    // `val` falls out of scope at the end of the block.
    // println!("{val}"); // ERROR: cannot find value `val` in this scope.
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the variable binding example through its entry point.
    #[test]
    fn test_variable_binding_example_entry_point() {
        main();
    }
}
