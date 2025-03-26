use anyhow::Result;
use clap::Parser;
use regex::Regex;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::{fs::File, sync::LazyLock};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(help = "Path to directory to process")]
    directory: PathBuf,
}

fn main() -> Result<()> {
    // Install a global tracing subscriber that listens for events and filters based on the value of the RUST_LOG environment variable.
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    walk_directory(args.directory.as_path())?;
    Ok(())
}

fn walk_directory(dir: &Path) -> Result<()> {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".md") {
            let path = entry.path();
            replace_in_file(path)?;
        }
    }
    Ok(())
}

fn replace_in_file(filepath: &Path) -> Result<()> {
    let mut file = File::open(filepath)?;
    let size = file.metadata()?.len() as usize;
    let mut buffer = String::with_capacity(size);
    file.read_to_string(&mut buffer)?;
    drop(file); // Close the file early

    if REGEX.is_match(&buffer) {
        let temp_filepath = filepath.with_extension(".tmp");
        let mut temp_file = File::create(&temp_filepath)?;
        buffer = replace(&buffer);
        temp_file.write_all(buffer.as_bytes())?;
        // Renames a file or directory to a new name, replacing the original file if to already exists
        std::fs::rename(&temp_filepath, filepath)?;
    }
    Ok(())
}

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{\s*!\s*(docs|github|lib\.rs|crates\.io|web|crate)\s*:*\s*([^}]+)\}\}").unwrap()
});

// {{!crate xyz}}
// {{!docs xyz}}
// {{!github xyz}}
// {{!lib.rs xyz}}
// {{!crates.io xyz}}
// {{!web xyz}}
fn replace(text: &str) -> String {
    let mut res = text.to_string();
    res.reserve(150);
    for (matching, typ, crates) in REGEX.captures_iter(&text).map(|caps| {
        (
            caps.get(0).map_or("", |m| m.as_str()), // Guaranteed to return a non-None value.
            caps.get(1).map_or("", |m| m.as_str()),
            caps.get(2).map_or("", |m| m.as_str()),
        )
    }) {
        let suffix = match typ {
            "crate" => "", // FIXME
            "docs" => "",
            "github" => "-github",
            "lib.rs" => "-lib.rs",
            "crates.io" => "-crates.io",
            "web" => "-website",
            _ => unreachable!(),
        };
        let replacement = crates.split_whitespace().map(|crate_name| { format!(
                "[![{crate_name}][c-{crate_name}{suffix}-badge]][c-{crate_name}{suffix}]{{{{hi:{crate_name}}}}}"
            ) }).collect::<Vec<_>>().join("");
        println!("{} {} {} {}", matching, typ, crates,replacement);
        res = res.replace(matching, &replacement);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_crate() {
        let text = "{{!crate clap regex}}";
        let expected = "[![clap][c-clap-badge]][c-clap]{{hi:clap}}[![regex][c-regex-badge]][c-regex]{{hi:regex}}";
        assert_eq!(replace(text), expected);
    }

    #[test]
    fn test_replace_docs() {
        let text = "{{!docs clap regex}}";
        let expected = "[![clap][c-clap-badge]][c-clap]{{hi:clap}}[![regex][c-regex-badge]][c-regex]{{hi:regex}}";
        assert_eq!(replace(text), expected);
    }

    #[test]
    fn test_replace_github() {
        let text = "{{!github clap regex}}";
        let expected = "[![clap][c-clap-github-badge]][c-clap-github]{{hi:clap}}[![regex][c-regex-github-badge]][c-regex-github]{{hi:regex}}";
        assert_eq!(replace(text), expected);
    }

    #[test]
    fn test_replace_lib_rs() {
        let text = "{{!lib.rs clap regex}}";
        let expected = "[![clap][c-clap-lib.rs-badge]][c-clap-lib.rs]{{hi:clap}}[![regex][c-regex-lib.rs-badge]][c-regex-lib.rs]{{hi:regex}}";
        assert_eq!(replace(text), expected);
    }

    #[test]
    fn test_replace_crates_io() {
        let text = "{{!crates.io clap regex}}";
        let expected = "[![clap][c-clap-crates.io-badge]][c-clap-crates.io]{{hi:clap}}[![regex][c-regex-crates.io-badge]][c-regex-crates.io]{{hi:regex}}";
        assert_eq!(replace(text), expected);
    }

    #[test]
    fn test_replace_web() {
        let text = "{{!web clap regex}}";
        let expected = "[![clap][c-clap-website-badge]][c-clap-website]{{hi:clap}}[![regex][c-regex-website-badge]][c-regex-website]{{hi:regex}}";
        assert_eq!(replace(text), expected);
    }

    #[test]
    fn test_replace_with_extra_spaces() {
        let text = "{{   !   crate   clap   regex   }}";
        let expected = "[![clap][c-clap-badge]][c-clap]{{hi:clap}}[![regex][c-regex-badge]][c-regex]{{hi:regex}}";
        assert_eq!(replace(text), expected);
    }

    #[test]
    fn test_replace_multiple_types() {
        let text = "{{!crate clap}} {{!docs regex}} {{!github walkdir}}";
        let expected = "[![clap][c-clap-badge]][c-clap]{{hi:clap}} [![regex][c-regex-badge]][c-regex]{{hi:regex}} [![walkdir][c-walkdir-github-badge]][c-walkdir-github]{{hi:walkdir}}";
        assert_eq!(replace(text), expected);
    }

    #[test]
    fn test_replace_no_match() {
        let text = "This is a test string with no matches.";
        let expected = "This is a test string with no matches.";
        assert_eq!(replace(text), expected);
    }

    #[test]
    fn test_replace_empty_input() {
        let text = "";
        let expected = "";
        assert_eq!(replace(text), expected);
    }
    #[test]
    fn test_replace_single_crate() {
        let text = "{{!crate clap}}";
        let expected = "[![clap][c-clap-badge]][c-clap]{{hi:clap}}";
        assert_eq!(replace(text), expected);
    }
    #[test]
    fn test_replace_single_docs() {
        let text = "{{!docs clap}}";
        let expected = "[![clap][c-clap-badge]][c-clap]{{hi:clap}}";
        assert_eq!(replace(text), expected);
    }
    #[test]
    fn test_replace_single_github() {
        let text = "{{!github clap}}";
        let expected = "[![clap][c-clap-github-badge]][c-clap-github]{{hi:clap}}";
        assert_eq!(replace(text), expected);
    }
    #[test]
    fn test_replace_single_lib_rs() {
        let text = "{{!lib.rs clap}}";
        let expected = "[![clap][c-clap-lib.rs-badge]][c-clap-lib.rs]{{hi:clap}}";
        assert_eq!(replace(text), expected);
    }
    #[test]
    fn test_replace_single_crates_io() {
        let text = "{{!crates.io clap}}";
        let expected = "[![clap][c-clap-crates.io-badge]][c-clap-crates.io]{{hi:clap}}";
        assert_eq!(replace(text), expected);
    }
    #[test]
    fn test_replace_single_web() {
        let text = "{{!web clap}}";
        let expected = "[![clap][c-clap-website-badge]][c-clap-website]{{hi:clap}}";
        assert_eq!(replace(text), expected);
    }

    #[test]
    fn test_replace_in_file_no_match() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("test.md");
        let mut file = File::create(&file_path)?;
        writeln!(file, "This is a test file.")?;
        drop(file);

        replace_in_file(&file_path)?;

        let contents = std::fs::read_to_string(&file_path)?;
        assert_eq!(contents, "This is a test file.\n");
        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_replace_in_file_with_match() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path = dir.path().join("test.md");
        let mut file = File::create(&file_path)?;
        writeln!(file, "This is a test file with {{{{!crate clap}}}}.")?;
        drop(file);

        replace_in_file(&file_path)?;

        let contents = std::fs::read_to_string(&file_path)?;
        assert_eq!(
            contents,
            "This is a test file with [![clap][c-clap-badge]][c-clap]{{hi:clap}}.\n"
        );
        dir.close()?;
        Ok(())
    }

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
            "This is test1 with [![clap][c-clap-badge]][c-clap]{{hi:clap}}.\n"
        );

        let contents2 = std::fs::read_to_string(&file_path2)?;
        assert_eq!(contents2, "This is test2 with no match.\n");

        assert!(file_path3.exists());
        dir.close()?;
        Ok(())
    }
}
