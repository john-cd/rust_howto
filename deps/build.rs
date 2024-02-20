//! Generate tests from the Markdown
//! See https://crates.io/crates/skeptic

use std::path::Path;

use anyhow::bail;
// use anyhow::Context;
use anyhow::Result;
use walkdir::WalkDir;

const REMOVED_TESTS: &[&str] = &[
    // "/code/src/dir/file.md",
];

// NOTE: `skeptic` issues "cargo:rerun-if-changed={}" for all
// markdown files it processes. A full code rebuild will happen every
// time the markdown sources change.
fn main() -> Result<()> {
    // Disable build.rs when building documentation at docs.rs
    // https://docs.rs/about/builds
    if std::env::var("DOCS_RS").is_ok() {
        return Ok(());
    }

    let root_path = std::fs::canonicalize("..")?;

    // Check for the existence of Markdown files.
    let markdown_path = root_path.join("src/");
    if !Path::new(&markdown_path).exists() {
        let msg = format!("The folder {:?} does not exist.", markdown_path);
        println!("cargo:warning=ERROR: {}", msg);
        bail!("{}", msg);
    }

    //// We are only testing code that are in the Markdown,
    //// not tests / examples that are in separate .rs files.
    // Remove any {{#include ../../deps/**/*.rs}} from the Markdown
    // (and replace by a hard-coded string to avoid Skeptic errors).
    // let contents_to_insert = "fn main() {}";
    // let modified_files =
    //     mdbook_utils::markdown::remove_includes_in_all_markdown_files_in(
    //         markdown_path.clone(),
    //         contents_to_insert,
    //     )
    //     .context("[build.rs] Failed to remove {{#include ...}}
    // statements.")?;

    // if !modified_files.is_empty() {
    //     modified_files.iter().for_each(|f| {
    //         println!(
    //             "cargo:warning=INFO: ignored {{#include ...}} in {}.",
    //             f.display()
    //         );
    //     });
    // }

    // Get the paths of all Markdown files
    let paths = WalkDir::new(markdown_path).into_iter()
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
