// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Working with `OsString` and `OsStr`, the platform-native strings.

use std::env;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

/// Builds an `OsString` and converts it to and from other string types.
fn main() {
    // Create an `OsString` (owned platform-native string):
    let mut os_string = OsString::new();
    os_string.push("Hello");
    os_string.push(" ");
    os_string.push("world");

    // Convert to OsStr (borrowed platform-native string).
    let os_str: &OsStr = os_string.as_os_str();

    // Conversion to `String` if valid UTF-8:
    match os_str.to_str() {
        Some(s) => println!("Valid UTF-8: {s}"),
        None => println!("Invalid UTF-8 sequence"),
    }

    // Create `OsString` from regular `String`:
    let regular_string = String::from("example.txt");
    let _os_string_from_regular = OsString::from(regular_string);

    // Working with environment variables:
    let path_var: OsString = env::var_os("PATH").unwrap_or_default();
    println!("PATH: {path_var:?}");

    // Iterate through `PATH` entries:
    let paths = env::split_paths(&path_var);
    for path in paths.take(3) {
        // Show first 3 paths only.
        // `path` is a `PathBuf`:
        println!("Path entry: {path:?}");
    }

    // Working with file paths:
    let file_path = Path::new("config").join("settings.json");
    println!("File path: {file_path:?}");

    // Extract file name as `OsStr`:
    if let Some(file_name) = file_path.file_name() {
        println!("File name as OsStr: {file_name:?}");
    }

    // Working with command arguments:
    let mut cmd = Command::new("echo");
    cmd.arg(&os_string); // Can use `OsString` directly as argument.

    // Convert `Path` to `OsString`:
    let path_buf = PathBuf::from("/usr/local/bin");
    let path_os_string: OsString = path_buf.into_os_string();
    println!("Path as OsString: {path_os_string:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the osstring example through its entry point.
    #[test]
    fn test_osstring_example_entry_point() {
        main();
    }

    /// `OsString` supports pushing and converts to `&str` when valid.
    #[test]
    fn builds_and_converts() {
        let mut s = OsString::new();
        s.push("Hello");
        s.push(" ");
        s.push("world");
        assert_eq!(s.as_os_str().to_str(), Some("Hello world"));
        assert_eq!(OsString::from("example.txt"), OsStr::new("example.txt"));
    }

    /// Paths join and expose their file name.
    #[test]
    fn path_components() {
        let p = Path::new("config").join("settings.json");
        assert_eq!(p.file_name(), Some(OsStr::new("settings.json")));
        assert_eq!(
            PathBuf::from("/usr/local/bin").into_os_string(),
            OsString::from("/usr/local/bin")
        );
    }

    /// `PATH` entries can be split.
    #[test]
    fn path_variable_is_split() {
        let joined = env::join_paths(["/a", "/b"]).unwrap();
        let parts: Vec<_> = env::split_paths(&joined).collect();
        assert_eq!(parts, [PathBuf::from("/a"), PathBuf::from("/b")]);
    }

    /// Invalid UTF-8 is not convertible to `&str`.
    #[cfg(unix)]
    #[test]
    fn invalid_utf8_is_detected() {
        use std::os::unix::ffi::OsStrExt as _;
        assert!(OsStr::from_bytes(&[0xFF, 0xFE]).to_str().is_none());
    }
}
