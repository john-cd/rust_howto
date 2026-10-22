// TODO Reviewed ??-??-2026
// [dependencies]
// anyhow = "1.0.95"
// clap = { version = "4.5.31", features = ["derive"] }
// regex = "1.11.1"
// tempfile = "3.20.0"
// tracing = "0.1.41"
// tracing-subscriber = {version = "0.3.20", features = ["env-filter"] }
// walkdir = "2.5.0"

use anyhow::Result;
use clap::Parser;
use std::path::Path;
use std::path::PathBuf;
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
pub fn walk_directory(dir: &Path) -> Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".md") {
            replace_in_file(entry.path())?;
        }
    }
    Ok(())
}

fn replace_in_file(path: &Path) -> Result<()> {
    let original = std::fs::read_to_string(path)?;
    let updated = replace_crate_placeholders(&original);
    if updated != original {
        std::fs::write(path, updated)?;
    }
    Ok(())
}

fn replace_crate_placeholders(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut remaining = input;
    while let Some(start) = remaining.find("{{!crate ") {
        let after_start = &remaining[start + "{{!crate ".len()..];
        let Some(end) = after_start.find("}}") else {
            break;
        };
        let crate_name = after_start[..end].trim();
        if crate_name.is_empty() || crate_name.chars().any(char::is_whitespace) {
            output.push_str(&remaining[..start + "{{!crate ".len() + end + 2]);
            remaining = &after_start[end + 2..];
            continue;
        }
        output.push_str(&remaining[..start]);
        output.push_str(&format!(
            "[![{crate_name}][c~{crate_name}~docs~badge]][c~{crate_name}~docs]{{{{hi:{crate_name}}}}}"
        ));
        remaining = &after_start[end + 2..];
    }
    output.push_str(remaining);
    output
}

fn main() -> Result<()> {
    // Install a global tracing subscriber that listens for events and filters based on the value of the RUST_LOG environment variable.
    tracing_subscriber::fmt::init();

    let args = cli::Args::parse();
    walk::walk_directory(args.directory.as_path())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_walk_directory() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path1 = dir.path().join("test1.md");
        let file_path2 = dir.path().join("test2.md");
        let file_path3 = dir.path().join("test3.txt");

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

}
