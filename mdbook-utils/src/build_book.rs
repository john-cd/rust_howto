#![allow(unused)]
use std::path::Path;
use std::process::Command;

use anyhow::anyhow;
use anyhow::Result;

// Invoke `mdbook build`
//
// Usage example:
// println!("Info: building the book...");
// let res = build_book(&root_path);
// if let Err(ref e) = res {
//     println!("cargo:warning=ERROR: {}", e);
//     return res;
// }
pub(crate) fn build_book(root_path: &Path) -> Result<()> {
    let output = Command::new("mdbook")
        .args(["build"])
        .current_dir(root_path)
        .output()?; // return if failed to execute process

    // write_log(&output.stdout, &output.stderr)?;

    if !output.status.success() {
        return Err(anyhow!(
            "Book building failed. Status: {}. Output: {}\n{}",
            output.status,
            String::from_utf8(output.stdout)?,
            String::from_utf8(output.stderr)?
        ));
    }
    Ok(())
}

// // Tell Cargo to rerun the build.rs script, if the .md files
// change.
// See also: https://crates.io/crates/cargo-emit
// fn build_rs_helper() -> Result<()> {
//     let root_path = std::fs::canonicalize("..")?;
//     let original_markdown_dir_path = root_path.join("src/");
//     let original_markdown_paths =
// WalkDir::new(original_markdown_dir_path).into_iter()
//         .map(|p| p.unwrap().path().to_string_lossy().into_owned())
// // DirEntry to String         .filter(|p| p.ends_with(".md"))
//         .collect::<Vec<_>>();

//     for path in original_markdown_paths {
//         println!("cargo:rerun-if-changed={}", path);
//         // println!("cargo:warning=DEBUG:{}", path);
//     }

//     Ok(())
// }
