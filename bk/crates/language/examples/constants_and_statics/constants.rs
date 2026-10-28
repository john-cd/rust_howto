#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! `const` Example.

/// The length, which is inlined wherever it is used.
const LEN: usize = 5;

/// Static copy of `LEN`, initialized from the constant.
static MY_LEN: usize = LEN;

/// Number of seconds in one hour.
const SECONDS_IN_ONE_HOUR: u32 = 60 * 60;

/// Compile-time sum that also uses the size of `u32`.
const C: usize = LEN + MY_LEN + std::mem::size_of::<u32>();

/// Wraps an integer array whose length is selected by a const generic.
struct Foo<const N: usize>([i32; N]);

/// Prints a message when its value is dropped.
struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("{0}", self.0);
    }
}

/// Constant value whose destructor is not run when it is unused.
const POD: PrintOnDrop = PrintOnDrop("This message does not appear.");

/// Constant used to demonstrate that a copied temporary is dropped.
const POD2: PrintOnDrop = PrintOnDrop("Dropped.");

/// Demonstrates basic constant usage in arrays, match statements, static initializers, and const generics.
fn demonstrate_const_usage() {
    println!("The length is: {LEN}");

    let arr: [i32; LEN] = [0; LEN];
    println!("arr: {arr:?}");

    let x = 5;
    match x {
        LEN => {
            println!("x == LEN");
        }
        _ => unreachable!(),
    }

    let _foo = Foo::<LEN>([0; 5]);

    let refer: &'static usize = &LEN;
    println!("The length again: {refer}");
}

/// Demonstrates destructor / drop behavior with constants.
fn demonstrate_const_drop() {
    let _pod = POD2;
    let _pod1 = POD2;
    let _pod2 = &POD2;
    println!("Before drop.");
}

/// Prints constants.
fn main() {
    demonstrate_const_usage();
    demonstrate_const_drop();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the constants example through its entry point.
    #[test]
    fn test_constants_example_entry_point() {
        main();
    }

    #[test]
    fn test_demonstrate_const_usage() {
        demonstrate_const_usage();
        assert_eq!(LEN, 5);
        assert_eq!(SECONDS_IN_ONE_HOUR, 3600);
        assert_eq!(C, 5 + 5 + std::mem::size_of::<u32>());
    }

    #[test]
    fn test_demonstrate_const_drop() {
        demonstrate_const_drop();
    }
}
