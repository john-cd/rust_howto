#![allow(dead_code)]
// ANCHOR: example
#![cfg(target_os = "linux")]
//! This example demonstrates the use of the `same_file` crate.

use std::io;
use std::path::Path;
use std::path::PathBuf;

// `is_same_file` returns true if the two file paths may correspond to the
// same file.
use same_file::is_same_file;

/// `contains_loop` checks if a given path contains a symbolic link loop.
///
/// It does this by iteratively truncating the path and checking if the
/// truncated path is the same file as the original path. If a loop is
/// found, it returns the two paths that form the loop. Returns None otherwise.
///
/// P: AsRef<Path> accepts PathBuf, Path...
fn contains_loop<P: AsRef<Path>>(
    path: P,
) -> io::Result<Option<(PathBuf, PathBuf)>> {
    let path: &Path = path.as_ref();
    // Copy into a mutable PathBuf
    let mut path_buf: PathBuf = path.to_path_buf();
    // Truncate path_buf in succession: /stuff/much -> /stuff -> /
    while path_buf.pop() {
        if is_same_file(&path_buf, path)? {
            return Ok(Some((path_buf, path.to_path_buf())));
            // Investigate the parent path against its own parents as well
        } else if let Some((looped_path1, looped_path2)) =
            contains_loop(&path_buf)?
        {
            return Ok(Some((looped_path1, looped_path2)));
        }
    }
    Ok(None)
}

fn main() {
    assert!(is_same_file("/tmp/foo", "/tmp/./foo").unwrap_or(false));

    assert_eq!(
        contains_loop("/tmp/foo/bar/baz/qux/bar/baz").unwrap(),
        Some((
            PathBuf::from("/tmp/foo"),
            PathBuf::from("/tmp/foo/bar/baz/qux")
        ))
    );
    println!("Loop found.");
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    // mkdir -p /tmp/foo/bar/baz
    if let Ok(exists) = std::fs::exists("/tmp/foo/bar/baz") {
        if !exists {
            std::fs::create_dir_all("/tmp/foo/bar/baz")?;
        }
    }
    // ln -s /tmp/foo/ /tmp/foo/bar/baz/qux
    if let Ok(exists) = std::fs::exists("/tmp/foo/bar/baz/qux") {
        if !exists {
            std::os::unix::fs::symlink("/tmp/foo/", "/tmp/foo/bar/baz/qux")?;
        }
    }
    main();
    Ok(())
}
