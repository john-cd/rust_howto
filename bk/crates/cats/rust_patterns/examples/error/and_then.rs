// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates chaining fallible operations with `Result::and_then`.

/// Divides `value` by two when it is even.
fn divide_by_two(value: i32) -> Result<i32, &'static str> {
    if value % 2 == 0 {
        Ok(value / 2)
    } else {
        Err("not divisible by two")
    }
}

/// Divides `value` by three when it is divisible by three.
fn divide_by_three(value: i32) -> Result<i32, &'static str> {
    if value % 3 == 0 {
        Ok(value / 3)
    } else {
        Err("not divisible by three")
    }
}

/// Runs two fallible divisions in sequence.
fn main() {
    let divisible = divide_by_two(12).and_then(divide_by_three);
    let not_divisible = divide_by_two(5).and_then(divide_by_three);

    println!("12 divided by two and then three: {divisible:?}");
    println!("5 divided by two and then three: {not_divisible:?}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::divide_by_three;
    use super::divide_by_two;

    /// Passes the first successful value into the next operation.
    #[test]
    fn and_then_runs_the_next_operation_after_success() {
        assert_eq!(divide_by_two(12).and_then(divide_by_three), Ok(2));
    }

    /// Keeps the first error and skips later operations.
    #[test]
    fn and_then_short_circuits_after_an_error() {
        assert_eq!(
            divide_by_two(5).and_then(divide_by_three),
            Err("not divisible by two")
        );
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
