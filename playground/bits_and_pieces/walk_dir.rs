//! Parses the directory argument and expands markers in its Markdown files.
//!
//! TODO Reviewed ??-??-2026

// [dependencies]
// anyhow = "1.0.95"
// clap = { version = "4.5.31", features = ["derive"] }
// regex = "1.11.1"
// tempfile = "3.20.0"
// tracing = "0.1.41"
// tracing-subscriber = {version = "0.3.20", features = ["env-filter"] }
// walkdir = "2.5.0"

use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(version)]
pub(crate) struct Args {
    #[clap(help = "Path to directory to process")]
    pub directory: PathBuf,
}

/// Walks through a directory and processes all markdown files.
///
/// # Arguments
///
/// * `dir` - The directory to walk through.
///
/// # Errors
/// Returns an error if traversal, reading, or atomic replacement fails.
pub fn walk_directory(dir: &Path) -> Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let f_name = entry.file_name().to_string_lossy();
        if entry.file_type().is_file() && f_name.ends_with(".md") {
            let path = entry.path();
            let contents = fs::read_to_string(path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            let updated = replace_crate_markers(&contents);
            if updated != contents {
                let parent = path
                    .parent()
                    .context("Markdown file must have a parent directory")?;
                let mut temporary = tempfile::NamedTempFile::new_in(parent)?;
                temporary.write_all(updated.as_bytes())?;
                temporary
                    .as_file()
                    .set_permissions(fs::metadata(path)?.permissions())?;
                temporary
                    .persist(path)
                    .with_context(|| format!("Failed to replace {}", path.display()))?;
            }
        }
    }
    Ok(())
}

/// Expands complete crate markers and preserves malformed or unfinished markers.
fn replace_crate_markers(contents: &str) -> String {
    const PREFIX: &str = "{{!crate ";

    let mut output = String::with_capacity(contents.len());
    let mut remaining = contents;
    while let Some(start) = remaining.find(PREFIX) {
        output.push_str(&remaining[..start]);
        let marker = &remaining[start + PREFIX.len()..];
        let Some(end) = marker.find("}}") else {
            output.push_str(&remaining[start..]);
            return output;
        };
        let crate_name = marker[..end].trim();
        if crate_name.is_empty() || crate_name.chars().any(char::is_whitespace) {
            output.push_str(&remaining[start..start + PREFIX.len() + end + 2]);
        } else {
            output.push_str(&format!(
                "[![{crate_name}][c~{crate_name}~docs~badge]][c~{crate_name}~docs]{{{{hi:{crate_name}}}}}"
            ));
        }
        remaining = &marker[end + 2..];
    }
    output.push_str(remaining);
    output
}

/// Parses the directory argument and expands markers in its Markdown files.
fn main() -> Result<()> {
    // Install a global tracing subscriber that listens for events and filters based on the value of the RUST_LOG environment variable.
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    walk_directory(args.directory.as_path())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use anyhow::Result;

    use super::*;

    /// Markdown files are rewritten while other files and Markdown-named directories survive.
    #[test]
    fn test_walk_directory() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path1 = dir.path().join("test1.md");
        let file_path2 = dir.path().join("test2.md");
        let file_path3 = dir.path().join("test3.txt");
        fs::create_dir(dir.path().join("directory.md"))?;

        let mut file1 = File::create(&file_path1)?;
        writeln!(file1, "This is test1 with {{{{!crate clap}}}}.")?;
        drop(file1);

        let mut file2 = File::create(&file_path2)?;
        writeln!(file2, "This is test2 with no match.")?;
        drop(file2);

        File::create(&file_path3)?;

        walk_directory(dir.path())?;

        let contents1 = std::fs::read_to_string(&file_path1)?;
        assert_eq!(
            contents1,
            "This is test1 with [![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}}.\n"
        );

        let contents2 = std::fs::read_to_string(&file_path2)?;
        assert_eq!(contents2, "This is test2 with no match.\n");

        assert!(file_path3.exists());
        dir.close()?;
        Ok(())
    }

    /// Multiple markers expand without consuming malformed trailing syntax.
    #[test]
    fn replaces_multiple_crate_markers_and_leaves_malformed_markers() {
        assert_eq!(
            replace_crate_markers("{{!crate clap}} and {{!crate serde}} {{!crate }} {{!crate"),
            "[![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}} and [![serde][c~serde~docs~badge]][c~serde~docs]{{hi:serde}} {{!crate }} {{!crate"
        );
    }
}
