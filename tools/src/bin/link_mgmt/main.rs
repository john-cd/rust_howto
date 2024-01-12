#![allow(unused)]

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Error;
use anyhow::Result;
use url::ParseError;
use url::Url;

mod link;
mod rules;

fn parse_markdown_file(path: &Path) -> Result<()> {
    let contents = fs::read_to_string(path)?;

    Ok(())
}

// TODO
fn main() -> Result<()> {
    // Locate the Markdown files with the src directory
    let src = Path::new("/code/src/");
    let paths: Vec<PathBuf> = tools::find_paths(src)?;

    // Create temp directory
    let dest_dir = "/code/book/temp/";
    tools::create_dir(dest_dir)?;

    // Extract links from md files
    for p in paths {}

    Ok(())
}
