#![allow(dead_code)]
// ANCHOR: example
use std::path::Path;
use std::path::PathBuf;

/// Displays the path and extract file names, extensions, and other path
/// components from a path.
///
/// Note that many path-handling functions use `AsRef<Path>` as their input
/// type, and therefore generically accept `&Path`, `&PathBuf`, `&str`,
/// `&OsStr`...
fn inspect_path<P: AsRef<Path>>(path: P) {
    let p: &Path = path.as_ref();

    // The `display()` method safely prints paths that may contain non-Unicode
    // data. This may perform lossy conversion, depending on the platform.
    println!("\nInspecting: {}", p.display());

    // `parent` returns the parent directory as an `Option<&Path>`:
    if let Some(parent) = p.parent() {
        println!("  Parent: {}", parent.display());
    }

    // Returns the last component of the path (e.g., `file.txt`) as an
    // `Option<&OsStr>`. `file_name` is a misnomer for a path to a directory:
    if let Some(filename) = p.file_name() {
        println!("  Last Component: {filename:?}");
    }

    //  Returns the file extension (without the leading dot) as an
    // `Option<&OsStr>`:
    if let Some(ext) = p.extension() {
        println!("  Extension: {ext:?}");
    }

    print!("  All Components:\n    ");
    for component in p.components() {
        print!(" {component:?}");
    }
    println!();
}

fn main() {
    // Create a `Path` from a string literal:
    let _path = Path::new("./temp/documents/report.txt");

    // Create an owned `PathBuf` to build a new path:
    let mut path_buffer = PathBuf::from("./temp");

    // Manipulate the `PathBuf`:
    path_buffer.push("downloads"); // Append a directory.
    path_buffer.push("archive.zip"); // Append a filename.

    println!("Built path: {}", path_buffer.display());

    // You can change the extension:
    path_buffer.set_extension("tar.gz");
    println!("Changed extension: {}", path_buffer.display());

    // You can remove the last component:
    path_buffer.pop(); // Removes "archive.tar.gz"
    println!("Popped component: {}", path_buffer.display());

    inspect_path(&path_buffer);

    // You can also use `collect` to build a `PathBuf` from its parts:
    let path_buffer: PathBuf = ["/", "home", "user"].iter().collect();
    println!("Another path: {}", path_buffer.display());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
