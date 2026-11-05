// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example functions for splitting a path into its components.

// Indices of / in the path
/// Splits `path` into a directory, a file name and an extension.
fn split_path(path: &str) -> (&str, &str, &str) {
    let mut first = "";
    let mut second = "";
    let mut third = "";
    let mut splits = path
        .char_indices()
        .filter_map(|(i, c)| (c == '/').then_some(i));

    if let Some(first_index) = splits.next() {
        first = &path[0..first_index];

        if let Some(second_index) = splits.next() {
            second = &path[first_index + 1..second_index];

            if let Some(third_index) = splits.next() {
                third = &path[third_index + 1..];
            }
        }
    }

    (first, second, third)
}

/// Runs the sample path splitting example.
fn main() {
    run_example();
}

/// Splits a sample path and prints the parts.
fn run_example() {
    let path = "src/playground/examples/split.rs";
    let (first, second, third) = split_path(path);

    println!("Path: {path}");
    println!("First segment: {first}");
    println!("Second segment: {second}");
    println!("Third segment: {third}");
}
// ANCHOR_END: example

/// Runs the path splitting example for the workspace test harness.
pub fn run() {
    run_example();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the path splitting example entry point runs.
    #[test]
    fn test_split_example_entry_point() {
        main();
    }

    /// Verifies the split of a path with at least three separators.
    #[test]
    fn splits_sample_path() {
        assert_eq!(
            split_path("src/playground/examples/split.rs"),
            ("src", "playground", "split.rs")
        );
    }

    /// Verifies the partial results for short paths.
    #[test]
    fn short_paths_leave_trailing_parts_empty() {
        assert_eq!(split_path(""), ("", "", ""));
        assert_eq!(split_path("file.rs"), ("", "", ""));
        assert_eq!(split_path("a/b"), ("a", "", ""));
        assert_eq!(split_path("a/b/c"), ("a", "b", ""));
    }
}
