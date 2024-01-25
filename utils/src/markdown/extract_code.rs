/// Extract Rust code examples from Markdown
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use regex::Regex;
use tracing::info;

/// Embedded Rust code extraction from Markdown
static EXTRACT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)```rust.*?\n(?<code>.*?)```").unwrap());

/// Extract code examples from all Markdown files within a directory
/// and write them to separate files.
pub fn extract_code_from_all_markdown_files_in(
    markdown_root: &str,
    code_dst_dir: &str,
) -> Result<()> {
    // Locate the Markdown files with the e.g. src/ directory
    let paths = crate::fs::find_markdown_files_in(Path::new(markdown_root))?;

    // Process each .md file
    for p in paths {
        info!("{p:?}");
        let buf = fs::read_to_string(p.as_path())?;
        let random_string =
            Alphanumeric.sample_string(&mut rand::thread_rng(), 5);

        // debug!("{:?}: length = {}", p, buf.len());
        for (number, (_, [code])) in EXTRACT_REGEX
            .captures_iter(&buf)
            .map(|c| c.extract())
            .enumerate()
        {
            // remove "# " at beginning of lines
            let code = Regex::new(r"(?m)^(?:#\s)(?<rest>.*)$")?
                .replace_all(code, "$rest");
            let code_filename = format!(
                "{}{}{}",
                p.file_stem()
                    .unwrap_or(random_string.as_ref())
                    .to_string_lossy(),
                if number == 0 {
                    String::new()
                } else {
                    number.to_string()
                },
                ".rs"
            );
            let code_path = Path::new(code_dst_dir).join(code_filename);
            info!(" {number}:\n {code_path:?}\n");
            File::create(code_path)?.write_all(code.as_bytes())?;
        }
    }
    Ok(())
}

/// Remove Rust code examples from Markdown
pub fn remove_code_from_all_markdown_files_in(
    markdown_root: &str,
) -> Result<()> {
    // Locate the Markdown files with the src directory
    let paths = crate::fs::find_markdown_files_in(Path::new(markdown_root))?;

    // Process each .md file
    for p in paths {
        info!("{p:?}");
        let buf = fs::read_to_string(p.as_path())?;
        let re =
            Regex::new(r"(?s)(?<first>```rust.*?\n)(?<code>.+?)(?<last>```)")?;
        if re.is_match(&buf) {
            let replacement = format!(
                "$first{{#include ../../../deps/examples/{}.rs}}\n$last",
                p.file_stem().unwrap().to_string_lossy()
            );
            let new_txt = re.replace_all(&buf, replacement);
            // debug!("{}", new_txt);
            File::create(p)?.write_all(new_txt.as_bytes())?;
        }
    }
    Ok(())
}
