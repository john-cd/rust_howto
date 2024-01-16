use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

pub fn read_to_string(path: OsString) -> String {
    let mut file = File::open(&path).expect("file to exist for reading");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("file to be readable");
    buf
}

fn parse_markdown_file(path: &Path) -> Result<()> {
    let contents = fs::read_to_string(path)?;

    Ok(())
}

fn walk_src_folder() -> Result<()> {
    // Locate the Markdown files with the src directory
    let src = Path::new("/code/src/");
    let paths: Vec<PathBuf> = tools::find_paths(src)?;

    // Create temp directory
    let dest_dir = "/code/book/temp/";
    tools::create_dir(dest_dir)?;

    // Extract links from md files
    // for p in paths {}

    Ok(())
}
