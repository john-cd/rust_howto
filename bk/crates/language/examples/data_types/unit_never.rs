// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The unit type `()` and the never type `!`.

// The unit type is used when there is nothing to return:
/// Returns the unit type explicitly.
#[allow(clippy::unused_unit)]
fn a_func() -> () {
    println!("a_func was called.");
}

// `()` is most commonly seen implicitly.
// Functions without a `-> ...` implicitly have return type `()`.
// The following is equivalent to the above:
/// Returns the unit type implicitly.
fn b_func() {
    println!("b_func was called.");
}
// The semicolon ; discards the result of an expression at the end of a block,
// so that the block returns `()`.

// The `!` type, also called "never", represents the type of computations which
// never resolve to any value at all.
/// Never returns; it panics.
fn diverge() -> ! {
    panic!("This function never returns!")
}
// This function could also contain an infinite loop,
// or call the `exit` function.

/// Calls functions that return the unit type.
fn main() {
    a_func();
    b_func();

    // `!` coerces to any type, so a diverging branch fits anywhere:
    let result: Result<u32, ()> = Ok(1);
    let n = match result {
        Ok(n) => n,
        Err(()) => diverge(),
    };
    println!("n = {n}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the unit never example through its entry point.
    #[test]
    fn test_unit_never_example_entry_point() {
        main();
    }

    /// Verifies that `diverge` panics.
    #[should_panic]
    #[test]
    fn test_panic() {
        diverge();
    }
}
