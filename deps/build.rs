//! Generate tests from the Markdown
//! See https://crates.io/crates/skeptic

use walkdir::WalkDir;
use std::path::Path;

// TODO: building tests should happen just before testing, not before build
// Consider using `ctor` crate to create tests when `cargo test`

const REMOVED_TESTS: &[&str] = &[
    // "/code/book/markdown/dir/file.md",
];

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build
    // script. println!("cargo:rerun-if-changed=/code/book/markdown/"
    // );

    let root = "/code/book/markdown/";
    if ! Path::new(root).exists() {
        let msg = format!("The root folder {} does not exist (yet). Run `mdbook build`", root);
        println!("cargo:warning={}", msg);
        return;
    }

    let paths = WalkDir::new(root).into_iter()
        // convert paths to Strings
        .map(|p| p.unwrap().path().to_str().unwrap().to_string())
        // only compile markdown files
        .filter(|p| p.ends_with(".md"))
        .filter(|p| !REMOVED_TESTS.contains(&p.as_ref()))
        .collect::<Vec<_>>();

    skeptic::generate_doc_tests(&paths[..]);

    println!("Tested:");
    paths.into_iter().for_each(|p| println!("{}", p));
}
