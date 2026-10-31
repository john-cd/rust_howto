// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Building and inspecting paths with `Path` and `PathBuf`.
//!
//! `Path` and `PathBuf` handle path parts portably: `join` appends a component
//! with the correct separator (`/` or `\`), and `file_name`, `extension` and
//! `parent` take it apart. Never build paths by gluing strings together.

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

/// Builds `./temp/downloads/archive.tar.gz` step by step.
fn build_archive_path() -> PathBuf {
    // Create an owned `PathBuf` to build a new path:
    let mut path_buffer = PathBuf::from("./temp");

    // Manipulate the `PathBuf`:
    path_buffer.push("downloads"); // Append a directory.
    path_buffer.push("archive.zip"); // Append a filename.

    println!("Built path: {}", path_buffer.display());

    // You can change the extension:
    path_buffer.set_extension("tar.gz");
    println!("Changed extension: {}", path_buffer.display());
    path_buffer
}

/// Builds paths and inspects their components.
fn main() {
    // Create a `Path` from a string literal:
    let _path = Path::new("./temp/documents/report.txt");

    let mut path_buffer = build_archive_path();

    // You can remove the last component:
    path_buffer.pop(); // Removes "archive.tar.gz"
    println!("Popped component: {}", path_buffer.display());

    inspect_path(&path_buffer);

    // You can also use `collect` to build a `PathBuf` from its parts:
    let path_buffer: PathBuf = ["/", "home", "user"].iter().collect();
    println!("Another path: {}", path_buffer.display());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the manipulate paths example through its entry point.
    #[test]
    fn test_manipulate_paths_example_entry_point() {
        main();
    }

    /// The built path has the expected components and extension.
    #[test]
    fn builds_archive_path() {
        let path = build_archive_path();
        assert_eq!(path, Path::new("./temp/downloads/archive.tar.gz"));
        assert_eq!(path.extension(), Some(std::ffi::OsStr::new("gz")));
        assert_eq!(path.file_stem(), Some(std::ffi::OsStr::new("archive.tar")));
        assert_eq!(path.parent(), Some(Path::new("./temp/downloads")));
    }

    /// Pushing an absolute path replaces the buffer; popping the root fails.
    #[test]
    fn push_absolute_replaces_and_pop_root_fails() {
        let mut path = PathBuf::from("a/b");
        #[expect(
            clippy::path_buf_push_overwrite,
            reason = "demonstrates that pushing an absolute path replaces the buffer"
        )]
        path.push("/etc");
        assert_eq!(path, Path::new("/etc"));
        let mut root = PathBuf::from("/");
        assert!(!root.pop());
        let collected: PathBuf = ["/", "home", "user"].iter().collect();
        assert_eq!(collected, Path::new("/home/user"));
        inspect_path("");
        inspect_path("/");
    }
}
