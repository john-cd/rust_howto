// TODO Reviewed ??-??-2026
// ANCHOR: example

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

/// Splits a sample path and prints the parts.
fn main() {
    let path = "src/playground/examples/split.rs";
    let (first, second, third) = split_path(path);

    println!("Path: {path}");
    println!("First segment: {first}");
    println!("Second segment: {second}");
    println!("Third segment: {third}");
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
