//! Process crate badge directives.
//!- Internal crate page: {{!crate xyz}}
//!   - {{!crate xyz}}
//!   - {{!crate xyz }}
//!   - {{ ! crate xyz }}
//!   - {{!crate: x_y-z}}
//!   - {{!crate : x_y-z}}
//! - {{!docs xyz}}
//! - {{!github xyz}}
//! - {{!lib.rs xyz}}
//! - {{!crates.io xyz}}
//! - {{!web xyz}}

// - Internal crate page: {{crate xyz}}
// - Variations:
//   - {{ crate xyz}}
//   - {{crate xyz }}
//   - {{crate: xyz}}
//   - {{crate : xyz}}
//   - {{crate x_y-z}}
// - `docs.rs` link: {{docs xyz}}
// - Github link: {{github xyz}}
// - `lib.rs` link: {{lib.rs xyz}}
// - `crates.io` link: {{crates.io xyz}}
// - Website for the crate: {{web xyz}}

// TODO

//         // {{c: xyz }}
//         let re_string: String = r"\{\{c:\s*(\S+)\s*\}\}".into();
//         let re = Regex::new(&re_string).expect("Invalid regex");
//         //let replacement = "";
//         rr.push(RegexAndReplacement {
//             re,
//             replacement: None,
//         });

use std::borrow::Cow;
use std::sync::LazyLock;

use regex::Regex;

// TODO use common.rs
// use Regex::replace_all

static CRATE_BADGE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{\s*!\s*(docs|github|lib\.rs|crates\.io|web|crate)\s*:?\s+([^}]+)\s*\}\}")
        .unwrap()
});

/// Returns true if at least one crate badge is found in the string slice.
fn has_crate_badge(s: &str) -> bool {
    CRATE_BADGE_REGEX.is_match(s)
}

/// Replaces any crate badge directive found by the corresponding markdown, in a given string slice:
///
/// # Arguments
///
/// * `text` - The text to process.
///
/// # Returns the updated content as a `String`.
fn process_crate_badge_directives(text: &str) -> Cow<str> {
    let mut res = Cow::Borrowed(text);
    // Yields successive non-overlapping matches in `text`.
    for (whole_matching_string, badge_kind, crates) in
        CRATE_BADGE_REGEX.captures_iter(text).map(|caps| {
            (
                caps.get(0).map_or("", |m| m.as_str()), // Guaranteed to return a non-`None` value.
                caps.get(1).map_or("", |m| m.as_str()),
                caps.get(2).map_or("", |m| m.as_str()),
            )
        })
    {
        let (prefix, suffix) = match badge_kind {
            "crate" => ("p~", "~crate"), // TODO decide of the refdef format for internal crate pages.
            "docs" => ("c~", "~docs"),
            "github" => ("c~", "~github"),
            "lib.rs" => ("c~", "~lib.rs"),
            "crates.io" => ("c~", "~crates.io"),
            "web" => ("c~", "~website"),
            _ => unreachable!(),
        };
        let replacement = crates.split_whitespace().map(|crate_name| { format!(
                "[![{crate_name}][{prefix}{crate_name}{suffix}~badge]][{prefix}{crate_name}{suffix}]{{{{hi:{crate_name}}}}}"
            ) }).collect::<Vec<_>>().join("");
        tracing::debug!(
            "match: {whole_matching_string}, kind: {badge_kind}, crates: {crates}, replacement: {replacement}"
        );
        res = Cow::Owned(res.replace(whole_matching_string, &replacement));
    }
    res
}

/// Replaces the following directives by the corresponding markdown, in a given file:
///
/// {{!crate xyz}}
/// {{!docs xyz}}
/// {{!github xyz}}
/// {{!lib.rs xyz}}
/// {{!crates.io xyz}}
/// {{!web xyz}}
///
pub fn process_crate_badge_directives_in_file(filepath: &std::path::Path) -> anyhow::Result<()> {
    core_lib::process_file(filepath, has_crate_badge, process_crate_badge_directives)?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_replace_crate() {
        let text = "{{!crate clap regex}}";
        let expected = "[![clap][p~clap~crate~badge]][p~clap~crate]{{hi:clap}}[![regex][p~regex~crate~badge]][p~regex~crate]{{hi:regex}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_docs() {
        let text = "{{!docs clap regex}}";
        let expected = "[![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}}[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_github() {
        let text = "{{!github clap regex}}";
        let expected = "[![clap][c~clap~github~badge]][c~clap~github]{{hi:clap}}[![regex][c~regex~github~badge]][c~regex~github]{{hi:regex}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_lib_rs() {
        let text = "{{!lib.rs clap regex}}";
        let expected = "[![clap][c~clap~lib.rs~badge]][c~clap~lib.rs]{{hi:clap}}[![regex][c~regex~lib.rs~badge]][c~regex~lib.rs]{{hi:regex}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_crates_io() {
        let text = "{{!crates.io clap regex}}";
        let expected = "[![clap][c~clap~crates.io~badge]][c~clap~crates.io]{{hi:clap}}[![regex][c~regex~crates.io~badge]][c~regex~crates.io]{{hi:regex}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_web() {
        let text = "{{!web clap regex}}";
        let expected = "[![clap][c~clap~website~badge]][c~clap~website]{{hi:clap}}[![regex][c~regex~website~badge]][c~regex~website]{{hi:regex}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_with_extra_spaces() {
        let text = "{{   !   docs   clap   regex   }}";
        let expected = "[![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}}[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_multiple_types() {
        let text = "{{!docs : regex}} {{!github: walkdir}}";
        let expected = "[![regex][c~regex~docs~badge]][c~regex~docs]{{hi:regex}} [![walkdir][c~walkdir~github~badge]][c~walkdir~github]{{hi:walkdir}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_no_match() {
        let text = "This is a test string with no matches.";
        let expected = "This is a test string with no matches.";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_empty_input() {
        let text = "";
        let expected = "";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_no_crate_name() {
        let text = "{{!crate }}";
        let expected = "{{!crate }}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_single_crate() {
        let text = "{{!crate clap}}";
        let expected = "[![clap][p~clap~crate~badge]][p~clap~crate]{{hi:clap}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }
    #[test]
    fn test_replace_single_docs() {
        let text = "{{!docs clap}}";
        let expected = "[![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }
    #[test]
    fn test_replace_single_github() {
        let text = "{{!github clap}}";
        let expected = "[![clap][c~clap~github~badge]][c~clap~github]{{hi:clap}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }
    #[test]
    fn test_replace_single_lib_rs() {
        let text = "{{!lib.rs clap}}";
        let expected = "[![clap][c~clap~lib.rs~badge]][c~clap~lib.rs]{{hi:clap}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }
    #[test]
    fn test_replace_single_crates_io() {
        let text = "{{!crates.io clap}}";
        let expected = "[![clap][c~clap~crates.io~badge]][c~clap~crates.io]{{hi:clap}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }
    #[test]
    fn test_replace_single_web() {
        let text = "{{!web clap}}";
        let expected = "[![clap][c~clap~website~badge]][c~clap~website]{{hi:clap}}";
        assert_eq!(process_crate_badge_directives(text), expected);
    }

    #[test]
    fn test_replace_in_file_no_match() -> anyhow::Result<()> {
        use core_lib::TestCase;
        use core_lib::test_with;
        use core_lib::walk_directory_and_process_files;

        let test_cases = vec![
            // No match.
            TestCase {
                file_name_and_ext: "test1.md",
                original_contents: Some("No changes."),
                expected_final_contents: Some("No changes."),
            },
            // Single match.
            TestCase {
                file_name_and_ext: "test2.md",
                original_contents: Some("This is a test file: {{!docs clap}}."),
                expected_final_contents: Some(
                    "This is a test file: [![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}}.",
                ),
            },
            // Empty file.
            TestCase {
                file_name_and_ext: "test3.md",
                original_contents: None,
                expected_final_contents: None,
            },
        ];

        let scope = core_lib::Scope::default();

        test_with(&test_cases, |temp_dir_path| {
            walk_directory_and_process_files(
                temp_dir_path,
                &scope,
                process_crate_badge_directives_in_file,
            )?;
            Ok(())
        })?;

        Ok(())
    }
}
