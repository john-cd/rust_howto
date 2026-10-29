// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Passing and calling function pointers.

/// Example function: add one to the input.
fn add_one(x: i32) -> i32 {
    x + 1
}

/// This function takes a function pointer in argument.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

/// Calls a function through a pointer and passes it to another function.
fn main() {
    let func_ptr: fn(i32) -> i32 = add_one; // Assign a function pointer to a variable.
    println!("Calling via the pointer: {}", func_ptr(10)); // Output: 11.

    let answer = do_twice(add_one, 5); // Pass `add_one` as an argument.
    println!("The answer is: {answer}"); // Output: 12 ( (5+1) + (5+1) )
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the function pointers example through its entry point.
    #[test]
    fn test_function_pointers_example_entry_point() {
        main();
    }

    /// `add_one` increments its argument.
    #[test]
    fn test_add_one() {
        assert_eq!(add_one(10), 11);
    }

    /// `do_twice` applies the function pointer twice and sums the results.
    #[test]
    fn test_do_twice() {
        assert_eq!(do_twice(add_one, 5), 12);
        assert_eq!(do_twice(|x| x * 3, 2), 12);
    }
}
