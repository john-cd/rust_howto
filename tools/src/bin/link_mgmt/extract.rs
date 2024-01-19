
use std::fs::File;
use std::{io::Write, fs};
use std::path::Path;

use anyhow::Result;
use rand::distributions::{Alphanumeric, DistString};
use once_cell::sync::Lazy;
use regex::Regex;

static EXTRACT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?s)```rust.*?\n(?<code>.*?)```").unwrap()
});

pub fn extract_code_from_all_markdown_files_in(markdown_root: &str) -> Result<()> {
    // Locate the Markdown files with the src directory
    let paths = tools::find_markdown_paths(Path::new(markdown_root))?;

    // Process each .md file
    for p in paths {
        println!("{p:?}");
        let buf = fs::read_to_string(p.as_path())?;
        let random_string = Alphanumeric.sample_string(&mut rand::thread_rng(), 5);

        // debug: println!("{:?}: length = {}", p, buf.len());
        for (number, (_, [code])) in EXTRACT_REGEX
                    .captures_iter(&buf)
                    .map(|c| c.extract())
                    .enumerate()
            {

                let code_filename = format!("{:?}_{}{}", p.file_stem().unwrap_or(random_string.as_ref()), number, ".rs");
                let code_path = Path::new("/code/deps/examples/temp/").join(code_filename);
                println!("{p:?}  {number}:\n  {code_path:?}\n");
                File::create(code_path)?.write_all(code.as_bytes())?;
            }
            // let result = re.replace("Springsteen, Bruce", "$first $last");
    }
    Ok(())
}
