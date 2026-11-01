// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! UTF-8 paths with `camino`.
//!
//! Standard Rust paths (`Path`) can hold bytes that are not valid UTF-8, which
//! makes them awkward to print. `camino` provides paths that are always UTF-8,
//! which is simpler when you only deal with ordinary names.

use std::path::PathBuf;

use camino::Utf8PathBuf;

/// Builds a UTF-8 path by pushing a component onto an initial path.
fn utf8_path() -> Utf8PathBuf {
    // Create a new Utf8PathBuf. The `from` method works like the standard one.
    let mut path = Utf8PathBuf::from("data");
    path.push("files");
    path.push("résumé");
    println!("Initial path: {path}");

    // Push a new component onto the path.
    // This is guaranteed to be UTF-8.
    path.push("résumé-2025.txt");
    println!("Final path: {path}");
    path
}

/// Compares standard paths with UTF-8 paths.
fn main() {
    // The standard `PathBuf` can contain non-UTF-8 data:
    let mut path = PathBuf::from("/home/user");
    path.push("résumé.pdf"); // This component is UTF-8 because it comes from a Rust string.

    // A `PathBuf` can hold non-UTF-8 data when it comes from the operating
    // system, but this path was built only from UTF-8 string components.
    if let Some(s) = path.to_str() {
        println!("The path is: {s}");
    }

    // `Utf8Path` and `Utf8PathBuf` work just like their standard library
    // counterparts but guarantee their content is valid UTF-8, eliminating
    // the need to check.

    // Create a new Utf8PathBuf. The `from` method works like the standard one.
    let path = utf8_path();

    // You can directly use the path as a `&str` without any `to_str()` call.
    if path.as_str().ends_with(".txt") {
        println!("This is a text file.");
    }

    // You can also get e.g. the file name directly:
    if let Some(file_name) = path.file_name() {
        println!("File name is: {file_name}");
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the camino example through its entry point.
    #[test]
    fn test_camino_example_entry_point() {
        main();
    }

    /// The built path has the expected components.
    #[test]
    fn builds_utf8_path() {
        let path = utf8_path();
        let mut parent = Utf8PathBuf::from("data");
        parent.push("files");
        parent.push("résumé");
        let expected = parent.join("résumé-2025.txt");
        assert_eq!(path, expected);
        assert_eq!(path.file_name(), Some("résumé-2025.txt"));
        assert_eq!(path.extension(), Some("txt"));
        assert_eq!(path.parent(), Some(parent.as_path()));
    }

    /// Non-UTF-8 paths cannot be converted.
    #[cfg(unix)]
    #[test]
    fn rejects_non_utf8_paths() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt as _;

        let invalid = PathBuf::from(OsString::from_vec(vec![b'a', 0xFF]));
        assert!(invalid.to_str().is_none());
        assert!(Utf8PathBuf::from_path_buf(invalid).is_err());
    }
}
