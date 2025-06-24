use anyhow::Result;

use super::*;

#[test]
fn test_replace_crate() {
    let text = "{{!crate clap regex}}";
    let expected =
        "[![clap][c-clap-badge]][c-clap]{{hi:clap}}[![regex][c-regex-badge]][c-regex]{{hi:regex}}";
    assert_eq!(replace(text), expected);
}

#[test]
fn test_replace_docs() {
    let text = "{{!docs clap regex}}";
    let expected =
        "[![clap][c-clap-badge]][c-clap]{{hi:clap}}[![regex][c-regex-badge]][c-regex]{{hi:regex}}";
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
    let expected =
        "[![clap][c-clap-badge]][c-clap]{{hi:clap}}[![regex][c-regex-badge]][c-regex]{{hi:regex}}";
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
fn test_replace_no_crate_name() {
    let text = "{{!crate }}";
    let expected = "{{!crate }}";
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
