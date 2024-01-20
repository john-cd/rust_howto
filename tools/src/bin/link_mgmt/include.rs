use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use regex::Regex;

static INSERT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\{\{#include (?<filepath>\S+?\.md)\}\}").unwrap()
});

// Replace {{#include complex/toml.md}} by the file contents
// Exclude {{#include refs/link-refs.md}} and similar
pub fn include_in_all_markdown_files_in(markdown_root: &str) -> Result<()> {
    // Locate the Markdown files with the src directory
    let paths = tools::find_markdown_paths(Path::new(markdown_root))?;

    // Process each .md file
    for p in paths {
        println!("Looking into {p:?}");
        let parent_dir = p.parent().unwrap().to_string_lossy();
        let buf = fs::read_to_string(p.as_path())?;
        if INSERT_REGEX.is_match(&buf) {
            let mut new_txt = buf.clone();
            for cap in INSERT_REGEX.captures_iter(&buf) {
                let rel_file_path = cap.name("filepath").unwrap().as_str();
                // print!("{rel_file_path:?}: ");
                if !rel_file_path.ends_with("refs.md") {
                    let path_file_to_insert =
                        Path::new(parent_dir.as_ref()).join(rel_file_path);
                    println!("Insert {path_file_to_insert:?}");
                    let contents_to_insert =
                        fs::read_to_string(path_file_to_insert)?;
                    //println!("\n{}", contents_to_insert);
                    //println!("{}", cap.get(0).unwrap().as_str());
                    new_txt = new_txt.replace(cap.get(0).unwrap().as_str(), &contents_to_insert);
                } else {
                    println!("Ignored");
                }
            }
            if new_txt != buf {
                //println!("{}",  new_txt);
                File::create(p)?.write_all(new_txt.as_bytes())?;
                break;
            }
        }
    }
    Ok(())
}
