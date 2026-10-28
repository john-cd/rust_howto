// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates transforming successful values and errors with `Result`.

/// Shows that `map` and `map_err` transform opposite variants of a result.
fn main() {
    let success: Result<i32, &str> = Ok(10);
    let doubled = success.map(|value| value * 2);
    println!("Doubled value: {doubled:?}");

    let failure: Result<i32, &str> = Err("service unavailable");
    let contextual_error =
        failure.map_err(|error| format!("request failed: {error}"));
    println!("Contextual error: {contextual_error:?}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::main;

    /// Transforms the success value while keeping it in `Ok`.
    /// Runs the `map` example through its entry point.
    #[test]
    fn map_transforms_the_success_value() {
        let result: Result<i32, &str> = Ok(10);
        assert_eq!(result.map(|value| value * 2), Ok(20));
    }

    /// Transforms the error while keeping it in `Err`.
    #[test]
    fn map_err_adds_error_context() {
        let result: Result<i32, &str> = Err("service unavailable");
        assert_eq!(
            result.map_err(|error| format!("request failed: {error}")),
            Err("request failed: service unavailable".to_owned())
        );
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
