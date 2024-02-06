//! Generate tests from the Markdown
//! See https://crates.io/crates/skeptic

use std::path::Path;

use anyhow::bail;
use anyhow::Result;
use walkdir::WalkDir;

const REMOVED_TESTS: &[&str] = &[
    // "/code/book/markdown/dir/file.md",
];

// NOTE: `skeptic` issues "cargo:rerun-if-changed={}" for all expanded
// markdown files it processes. A full code rebuild will happen every
// time `mdBook build` is run. Therefore don't build the book here
// every time, but rather check the expanded markdown sources exist.
// See the `justfile`.
fn main() -> Result<()> {
    // Disable build.rs when building documentation at docs.rs
    // https://docs.rs/about/builds
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    let root_path = std::fs::canonicalize("..")?;

    // Check for the existence of expanded Markdown files
    // (created by `mdbook build`).
    let expanded_markdown_path = root_path.join("book/markdown/");
    if !Path::new(&expanded_markdown_path).exists() {
        let msg =
            format!("The folder {:?} does not exist.", expanded_markdown_path);
        println!("cargo:warning=ERROR: {}", msg);
        bail!("{}", msg);
    }

    // Remove any leftover {{#include ../../deps/examples/*.rs}} from the
    // expanded Markdown (to avoid skeptic errors) and warn about
    // missing files.

    mdbook_utils::markdown::remove_includes_in_all_markdown_files_in(expanded_markdown_path.clone())?;

    // let msg = format!(
    //     "Some {{#include }} statements did not resolve! Are you missing files or code examples?"
    // );
    // println!("cargo:warning=WARN: {}", msg);

    // Get the paths of all expanded Markdown files
    let paths = WalkDir::new(expanded_markdown_path).into_iter()
        // convert DirEntry to String
        .map(|p| p.unwrap().path().to_string_lossy().into_owned())
        .filter(|p| p.ends_with(".md"))
        .filter(|p| !REMOVED_TESTS.contains(&p.as_ref()))
        .collect::<Vec<_>>();

    println!(
        "cargo:warning=Info: building the skeptic tests for the book under root {:?}",
        root_path
    );
    // `skeptic` must run from `build.rs` - it needs the OUT_DIR env
    // variable... thus this code cannot be moved to a separate
    // executable.
    skeptic::generate_doc_tests(&paths[..]);

    Ok(())
}
