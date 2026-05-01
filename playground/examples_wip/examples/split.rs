#![allow(dead_code)]
// ANCHOR: example
// Indices of / in the path
fn split_path(path: &str) -> (&str, &str, &str) {
    let mut first = "";
    let mut second = "";
    let mut third = "";
    let mut splits = path
        .char_indices()
        .filter_map(|(i, c)| if c == '/' { Some(i) } else { None });

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

fn main() {
    let path = "src/playground/examples/split.rs";
    let (first, second, third) = split_path(path);

    println!("Path: {path}");
    println!("First segment: {first}");
    println!("Second segment: {second}");
    println!("Third segment: {third}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
