#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates how to skip hidden files and directories during
//! directory traversal.

use walkdir::DirEntry;
use walkdir::WalkDir;

/// Checks if a directory entry should be included in the traversal.
///
/// This function filters out hidden files and directories (those starting with
/// a dot) except for the root directory itself.
fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with('.'))
        .unwrap_or(false)
}

fn main() {
    let w = WalkDir::new(".");
    w.into_iter()
        .filter_entry(is_not_hidden)
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()));
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
