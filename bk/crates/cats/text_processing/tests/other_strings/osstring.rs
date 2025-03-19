// ANCHOR: example
use std::env;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Create an OsString (owned platform-native string)
    let mut os_string = OsString::new();
    os_string.push("Hello");
    os_string.push(" ");
    os_string.push("world");

    // Convert to OsStr (borrowed platform-native string)
    let os_str: &OsStr = os_string.as_os_str();

    // Conversion to String if valid UTF-8
    match os_str.to_str() {
        Some(s) => println!("Valid UTF-8: {}", s),
        None => println!("Invalid UTF-8 sequence"),
    }

    // Create OsString from regular String
    let regular_string = String::from("example.txt");
    let _os_string_from_regular = OsString::from(regular_string);

    // Working with environment variables
    let path_var: OsString = env::var_os("PATH").unwrap_or_default();
    println!("PATH: {:?}", path_var);

    // Iterate through PATH entries
    let paths = env::split_paths(&path_var);
    for path in paths.take(3) {
        // Show first 3 paths only
        // path is a PathBuf
        println!("Path entry: {:?}", path);
    }

    // Working with file paths
    let file_path = Path::new("config").join("settings.json");
    println!("File path: {:?}", file_path);

    // Extract file name as OsStr
    if let Some(file_name) = file_path.file_name() {
        println!("File name as OsStr: {:?}", file_name);
    }

    // Working with command arguments
    let mut cmd = Command::new("echo");
    cmd.arg(&os_string); // Can use OsString directly as argument

    // Convert Path to OsString
    let path_buf = PathBuf::from("/usr/local/bin");
    let path_os_string: OsString = path_buf.into_os_string();
    println!("Path as OsString: {:?}", path_os_string);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
