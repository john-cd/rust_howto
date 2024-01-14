//! Generate tests from the Markdown
//! See https://crates.io/crates/skeptic

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use walkdir::WalkDir;

const REMOVED_TESTS: &[&str] = &[
    // "/code/book/markdown/dir/file.md",
];

#[allow(dead_code)]
fn write_log(out: &[u8], err: &[u8]) -> Result<()> {
    let mut buffer = BufWriter::new(File::create("build.log")?);
    buffer.write_all(out)?;
    buffer.write_all(err)?;
    buffer.flush()?;
    Ok(())
}

fn build_book(root_path: &Path) -> Result<()> {
    let output = Command::new("mdbook")
        .args(["build"])
        .current_dir(root_path)
        .output()?; // return if failed to execute process

    // write_log(&output.stdout, &output.stderr)?;

    let out_string =
        String::from_utf8(output.stdout)? + &String::from_utf8(output.stderr)?;

    if !output.status.success() {
        return Err(anyhow!(
            "Book building failed. Status: {}. Output: {}",
            output.status,
            out_string
        ));
    }

    Ok(())
}

fn main() -> Result<()> {
    // TODO
    // Tell Cargo that if the given file changes, to rerun this build
    // script. println!("cargo:rerun-if-changed=/code/book/markdown/"
    // );

    let root_path = std::fs::canonicalize("..")?;
    println!(
        "cargo:warning=Building the skeptic tests for the book under root {:?}",
        root_path
    );

    // Build the book to get the fully expanded Markdown
    let res = build_book(&root_path);
    if let Err(ref e) = res {
        println!("cargo:warning=Book building failed. Error: {}", e);
        return res;
    }

    let expanded_markdown_path = root_path.join("book/markdown/");
    if !Path::new(&expanded_markdown_path).exists() {
        let msg =
            format!("The folder {:?} does not exist.", expanded_markdown_path);
        println!("cargo:warning={}", msg);
        bail!("{}", msg);
    }

    let paths = WalkDir::new(expanded_markdown_path).into_iter()
        // convert paths to Strings
        .map(|p| p.unwrap().path().to_str().unwrap().to_string())
        // only compile markdown files
        .filter(|p| p.ends_with(".md"))
        .filter(|p| !REMOVED_TESTS.contains(&p.as_ref()))
        .collect::<Vec<_>>();

    skeptic::generate_doc_tests(&paths[..]);

    Ok(())
}
