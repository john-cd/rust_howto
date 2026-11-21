//! Checks for visible TODO placeholders in published Markdown and the license.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use anyhow::Context as _;
use anyhow::Result;
use regex::Regex;
use walkdir::WalkDir;

/// Matches a TODO placeholder at the end of an authored line.
static TODO_PLACEHOLDER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\bTODO\b(?:\s+[\w?!.:/-]+){0,3}\s*$").expect("the TODO placeholder regex is valid")
});

/// Matches a Markdown code fence opener or closer.
static FENCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s{0,3}(`{3,}|~{3,})").expect("the Markdown fence regex is valid")
});

/// Matches the repository-wide TODO review marker excluded from the scan.
static REVIEW_MARKER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*<!--\s*TODO Reviewed \?\?-\?\?-\d{4}\s*-->\s*$")
        .expect("the review-marker regex is valid")
});

/// Matches the start of an mdBook hidden div.
static HIDDEN_DIV: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)<div\s+class=["']hidden["']\s*>"#).expect("the hidden-div regex is valid")
});

/// Matches an mdBook hidden-div close tag.
static DIV_CLOSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)</div\s*>").expect("the closing-div regex is valid"));

/// One visible TODO placeholder with its repository-relative source location.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoFinding {
    /// Repository-relative Markdown or license path.
    pub path: String,

    /// One-based source line number.
    pub line: usize,

    /// Trimmed source line containing the placeholder.
    pub text: String,
}

/// Scans the license and published Markdown for visible TODO placeholders.
///
/// The scan excludes fenced code, mdBook hidden divs, and the standard TODO review marker.
///
/// # Arguments
///
/// * `repo_root` - Repository root containing `LICENSE` and `bk/src`.
///
/// # Errors
///
/// Returns an error if a source file cannot be enumerated, read, or represented below the root.
pub fn check_visible_todos(repo_root: &Path) -> Result<Vec<TodoFinding>> {
    let repo_root = repo_root
        .canonicalize()
        .with_context(|| format!("could not resolve repository root {}", repo_root.display()))?;
    let mut paths = vec![repo_root.join("LICENSE")];
    let markdown_root = repo_root.join("bk/src");
    if markdown_root.is_dir() {
        for entry in WalkDir::new(&markdown_root).follow_links(false) {
            let entry =
                entry.with_context(|| format!("could not walk {}", markdown_root.display()))?;
            if !entry.file_type().is_dir()
                && !entry.file_type().is_symlink()
                && entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
            {
                paths.push(entry.path().to_path_buf());
            }
        }
    }
    paths[1..].sort();

    let mut findings = Vec::new();
    for path in paths {
        let source = fs::read_to_string(&path)
            .with_context(|| format!("could not read {}", path.display()))?;
        let relative = path
            .strip_prefix(&repo_root)
            .with_context(|| format!("{} is outside the repository root", path.display()))?;
        let path = relative
            .components()
            .map(|component| component.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");
        findings.extend(
            visible_todo_lines(&source)
                .into_iter()
                .map(|(line, text)| TodoFinding {
                    path: path.clone(),
                    line,
                    text,
                }),
        );
    }
    Ok(findings)
}

/// Returns line-numbered TODOs that are visible to readers.
fn visible_todo_lines(source: &str) -> Vec<(usize, String)> {
    let mut in_fence = false;
    let mut fence_char = None;
    let mut in_hidden_div = false;
    let mut matches = Vec::new();

    for (index, line) in source.lines().enumerate() {
        if REVIEW_MARKER.is_match(line) {
            continue;
        }
        if let Some(fence) = FENCE.captures(line).and_then(|capture| capture.get(1)) {
            let marker = fence.as_str().as_bytes()[0];
            if !in_fence {
                in_fence = true;
                fence_char = Some(marker);
            } else if fence_char == Some(marker) && fence.as_str().len() >= 3 {
                in_fence = false;
            }
            continue;
        }
        if in_fence {
            continue;
        }

        if HIDDEN_DIV.is_match(line) {
            in_hidden_div = true;
        }
        if !in_hidden_div && TODO_PLACEHOLDER.is_match(line) {
            matches.push((index + 1, line.trim().to_owned()));
        }
        if DIV_CLOSE.is_match(line) {
            in_hidden_div = false;
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Excludes fenced and hidden TODOs while retaining the review marker convention.
    #[test]
    fn visible_todo_lines_honor_fences_hidden_divs_and_review_markers() {
        let source = "TODO\n<!-- TODO Reviewed ??-??-2026 -->\n```text\nTODO in a code fence\n```\n<div class=\"hidden\">\nTODO hidden\n</div>\nVisible TODO remove this\n";
        assert_eq!(
            visible_todo_lines(source),
            [
                (1, "TODO".to_owned()),
                (9, "Visible TODO remove this".to_owned())
            ]
        );
    }

    /// Reports authored file paths and one-based line numbers.
    #[test]
    fn reports_repository_relative_todo_locations() -> Result<()> {
        let directory = tempfile::tempdir()?;
        fs::create_dir_all(directory.path().join("bk/src/chapter"))?;
        fs::write(directory.path().join("LICENSE"), "License text\n")?;
        fs::write(
            directory.path().join("bk/src/chapter/page.md"),
            "# Page\nTODO remove after review\n",
        )?;
        let findings = check_visible_todos(directory.path())?;
        assert_eq!(
            findings,
            [TodoFinding {
                path: "bk/src/chapter/page.md".to_owned(),
                line: 2,
                text: "TODO remove after review".to_owned(),
            }]
        );
        Ok(())
    }
}
