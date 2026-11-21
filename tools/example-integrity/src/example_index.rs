//! Generates the example index and its derived reference definitions.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;
use regex::Regex;
use tempfile::NamedTempFile;
use walkdir::WalkDir;

/// Matches the book's linked chapter entries in `SUMMARY.md`.
static SUMMARY_ENTRY: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^([[:blank:]]*)-? ?\[([^\]]*)\]\(([^()]+\.md)\)[[:blank:]]*$")
        .expect("the summary-entry regex is valid")
});

/// Matches example reference definitions copied into the generated reference file.
static EXAMPLE_REFDEF: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^(\[ex~.+?\]):\s*(\S+)\s*$").expect("the example-reference regex is valid")
});

/// Matches any reference definition already present in the generated reference file.
static REFDEF: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^(\[[^\]]+\]):\s*(\S+)\s*$").expect("the reference-definition regex is valid")
});

/// Matches an example row in a recipe table.
static EXAMPLE_TABLE_ROW: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^\|.*\]\[ex~").expect("the example-table regex is valid")
});

/// Matches content that disqualifies a table from being included as an index table.
static NON_EXAMPLE_TABLE_CONTENT: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^#{1,6}\s|\{\{#(rustdoc_)?include[^}]*\.rs([: }])")
        .expect("the non-example-table regex is valid")
});

/// Matches Rust code fences and source includes used to identify example chapters.
static RUST_EXAMPLE: std::sync::LazyLock<Regex> = std::sync::LazyLock::new(|| {
    Regex::new(r"^\s*(?:`{3,}|~{3,})rust\b|\{\{#(rustdoc_)?include[^}]*\.rs([: }])")
        .expect("the Rust-example regex is valid")
});

/// Generates the Markdown page that lists published chapters containing examples.
///
/// The review marker and trailing hidden block are retained from the existing index.
///
/// # Arguments
///
/// * `book_root` - Book directory containing `src/SUMMARY.md` and `src/indices`.
///
/// # Errors
///
/// Returns an error if the summary or index cannot be read or the index cannot be replaced.
pub fn generate_example_index(book_root: &Path) -> Result<bool> {
    let (target, content) = build_example_index(book_root)?;
    let current = fs::read_to_string(&target)
        .with_context(|| format!("could not read example index {}", target.display()))?;
    if current == content {
        return Ok(false);
    }
    write_atomic(&target, content.as_bytes())?;
    Ok(true)
}

/// Checks whether the example index matches the current summary and chapter content.
///
/// # Arguments
///
/// * `book_root` - Book directory containing `src/SUMMARY.md` and `src/indices`.
///
/// # Errors
///
/// Returns an error if the summary, index, or chapter content cannot be read.
pub fn check_example_index(book_root: &Path) -> Result<Vec<String>> {
    let (target, expected) = build_example_index(book_root)?;
    let current = fs::read_to_string(&target)
        .with_context(|| format!("could not read example index {}", target.display()))?;
    if current == expected {
        Ok(Vec::new())
    } else {
        Ok(vec![format!(
            "src/indices/examples_index.md is out of date; run the example-index generate command"
        )])
    }
}

/// Generates `src/indices/refs.incl.md` from chapter-local example definitions.
///
/// Existing hand-written reference definitions are retained and sorted with derived entries.
///
/// # Arguments
///
/// * `book_root` - Book directory containing `src/indices/refs.incl.md`.
///
/// # Errors
///
/// Returns an error if a source reference file cannot be read or the destination cannot be
/// replaced.
pub fn generate_example_refdefs(book_root: &Path) -> Result<bool> {
    let (target, content, _) = build_example_refdefs(book_root)?;
    let current = fs::read_to_string(&target)
        .with_context(|| format!("could not read example references {}", target.display()))?;
    if current == content {
        return Ok(false);
    }
    write_atomic(&target, content.as_bytes())?;
    Ok(true)
}

/// Checks derived example references without changing the destination file.
///
/// # Arguments
///
/// * `book_root` - Book directory containing `src/indices/refs.incl.md`.
///
/// # Errors
///
/// Returns an error if a source reference file cannot be read.
pub fn check_example_refdefs(book_root: &Path) -> Result<Vec<String>> {
    let (_, _, findings) = build_example_refdefs(book_root)?;
    Ok(findings)
}

/// Builds the example index contents from the summary and chapter source files.
fn build_example_index(book_root: &Path) -> Result<(PathBuf, String)> {
    let book_root = book_root
        .canonicalize()
        .with_context(|| format!("could not resolve book root {}", book_root.display()))?;
    let src = book_root.join("src");
    let summary = src.join("SUMMARY.md");
    let target = src.join("indices/examples_index.md");
    let summary_text = fs::read_to_string(&summary)
        .with_context(|| format!("could not read {}", summary.display()))?;
    let current = fs::read_to_string(&target)
        .with_context(|| format!("could not read example index {}", target.display()))?;

    let mut output = String::new();
    if current
        .lines()
        .next()
        .is_some_and(|line| line.starts_with("<!-- TODO Reviewed "))
    {
        output.push_str(current.lines().next().unwrap_or_default());
        output.push_str("\n\n");
    }
    output.push_str("# Index of Examples\n\n");

    let mut section = String::new();
    let mut previous_section = String::new();
    for line in summary_text.lines() {
        let Some(captures) = SUMMARY_ENTRY.captures(line) else {
            continue;
        };
        let indent = captures.get(1).map_or("", |value| value.as_str());
        let title = captures.get(2).map_or("", |value| value.as_str());
        let chapter = captures.get(3).map_or("", |value| value.as_str());
        let chapter_path = src.join(chapter);
        if !chapter_path.is_file() || chapter.contains("/_") {
            continue;
        }
        if indent.is_empty() {
            title.clone_into(&mut section);
        }

        let table = chapter
            .strip_suffix(".md")
            .map(|stem| format!("{stem}.incl.md"));
        let table_path = table.as_ref().map(|name| src.join(name));
        let table_text = table_path
            .as_ref()
            .filter(|path| path.is_file())
            .map(|path| {
                fs::read_to_string(path)
                    .with_context(|| format!("could not read {}", path.display()))
            })
            .transpose()?;
        let has_table = table_text.as_deref().is_some_and(is_example_table);
        let mut source_to_scan = fs::read_to_string(&chapter_path)
            .with_context(|| format!("could not read {}", chapter_path.display()))?;
        if let Some(table_text) = &table_text {
            source_to_scan.push('\n');
            source_to_scan.push_str(table_text);
        }
        if !has_table && !contains_rust_example(&source_to_scan) {
            continue;
        }

        if section != previous_section {
            output.push_str("## ");
            output.push_str(&section);
            output.push_str("\n\n");
            previous_section.clone_from(&section);
        }
        output.push_str("### [");
        output.push_str(title);
        output.push_str("](../");
        output.push_str(chapter);
        output.push_str(")\n\n");
        if has_table && let Some(table) = table {
            output.push_str("{{#include ../");
            output.push_str(&table);
            output.push_str("}}\n\n");
        }
    }

    output.push_str("{{#include refs.incl.md}}\n{{#include ../refs/link-refs.md}}\n\n");
    output.push_str(&hidden_index_block(&current));
    Ok((target, output))
}

/// Determines whether a chapter table contains only example-reference rows.
fn is_example_table(source: &str) -> bool {
    let has_example_row = source.lines().any(|line| EXAMPLE_TABLE_ROW.is_match(line));
    let contains_heading_or_rust_include = source
        .lines()
        .any(|line| NON_EXAMPLE_TABLE_CONTENT.is_match(line));
    has_example_row && !contains_heading_or_rust_include
}

/// Detects Rust code fences or source includes in Markdown text.
fn contains_rust_example(source: &str) -> bool {
    source.lines().any(|line| RUST_EXAMPLE.is_match(line))
}

/// Copies every mdBook hidden div block from the existing index.
fn hidden_index_block(source: &str) -> String {
    let mut output = String::new();
    let mut in_block = false;
    for line in source.lines() {
        if line == "<div class=\"hidden\">" {
            in_block = true;
        }
        if in_block {
            output.push_str(line);
            output.push('\n');
            if line == "</div>" {
                in_block = false;
            }
        }
    }
    output
}

/// Builds the derived example reference contents and reports stale derived entries.
fn build_example_refdefs(book_root: &Path) -> Result<(PathBuf, String, Vec<String>)> {
    let book_root = book_root
        .canonicalize()
        .with_context(|| format!("could not resolve book root {}", book_root.display()))?;
    let src = book_root.join("src");
    let target = src.join("indices/refs.incl.md");
    let target_text = fs::read_to_string(&target)
        .with_context(|| format!("could not read {}", target.display()))?;
    let mut refs = BTreeMap::new();
    let mut header = Vec::new();
    for line in target_text.lines() {
        if let Some(captures) = REFDEF.captures(line) {
            refs.insert(
                captures
                    .get(1)
                    .map_or("", |value| value.as_str())
                    .to_owned(),
                captures
                    .get(2)
                    .map_or("", |value| value.as_str())
                    .to_owned(),
            );
        } else if !line.trim().is_empty() {
            header.push(line.to_owned());
        }
    }

    let derived = derived_example_refdefs(&src, &target)?;
    let mut findings = Vec::new();
    for (label, destination) in derived {
        if refs.get(&label) != Some(&destination) {
            findings.push(format!(
                "missing or outdated in src/indices/refs.incl.md: {label}"
            ));
            refs.insert(label, destination);
        }
    }

    let mut output = String::new();
    output.push_str(&header.join("\n"));
    output.push_str("\n\n");
    for (label, destination) in refs {
        output.push_str(&label);
        output.push_str(": ");
        output.push_str(&destination);
        output.push('\n');
    }
    Ok((target, output, findings))
}

/// Collects example reference definitions from chapter fragments in stable path order.
fn derived_example_refdefs(src: &Path, target: &Path) -> Result<BTreeMap<String, String>> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(src).follow_links(false) {
        let entry = entry.with_context(|| format!("could not walk {}", src.display()))?;
        if entry.path().is_file() && entry.file_name() == "refs.incl.md" {
            let path = entry.into_path();
            if path != target {
                paths.push(path);
            }
        }
    }
    paths.sort();

    let mut refs = BTreeMap::new();
    for path in paths {
        let source = fs::read_to_string(&path)
            .with_context(|| format!("could not read {}", path.display()))?;
        let relative_directory = path
            .parent()
            .and_then(|parent| parent.strip_prefix(src).ok())
            .map_or_default(path_to_slashes);
        for line in source.lines() {
            let Some(captures) = EXAMPLE_REFDEF.captures(line) else {
                continue;
            };
            let label = captures.get(1).map_or("", |value| value.as_str());
            let destination = captures.get(2).map_or("", |value| value.as_str());
            let destination = if has_url_scheme(destination) {
                destination.to_owned()
            } else {
                let (path, anchor) = destination
                    .split_once('#')
                    .map_or((destination, None), |(path, anchor)| (path, Some(anchor)));
                let joined = if path.starts_with('/') {
                    path.to_owned()
                } else if relative_directory.is_empty() {
                    normalize_posix_path(&format!("../{path}"))
                } else {
                    normalize_posix_path(&format!("../{relative_directory}/{path}"))
                };
                match anchor {
                    Some(anchor) => format!("{joined}#{anchor}"),
                    None => joined,
                }
            };
            refs.insert(label.to_owned(), destination);
        }
    }
    Ok(refs)
}

/// Returns whether a destination begins with a URI scheme.
fn has_url_scheme(destination: &str) -> bool {
    let Some((scheme, _)) = destination.split_once(':') else {
        return false;
    };
    let mut characters = scheme.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_lowercase())
        && characters.all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || matches!(character, '+' | '.' | '-')
        })
}

/// Normalizes a slash-separated relative path without consulting the host platform.
fn normalize_posix_path(path: &str) -> String {
    let absolute = path.starts_with('/');
    let mut components: Vec<&str> = Vec::new();
    for component in path.split('/') {
        match component {
            "" | "." => {}
            ".." if components.last().is_some_and(|previous| *previous != "..") => {
                components.pop();
            }
            ".." if !absolute => components.push(component),
            ".." => {}
            other => components.push(other),
        }
    }
    let joined = components.join("/");
    if absolute {
        format!("/{joined}")
    } else if joined.is_empty() {
        ".".to_owned()
    } else {
        joined
    }
}

/// Converts a platform path to slash-separated Markdown path text.
fn path_to_slashes(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

/// Writes generated text through a temporary sibling, preserving destination permissions.
fn write_atomic(path: &Path, contents: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let mut temporary = NamedTempFile::new_in(parent).with_context(|| {
        format!(
            "could not create a temporary file beside {}",
            path.display()
        )
    })?;
    temporary
        .write_all(contents)
        .with_context(|| format!("could not write generated content for {}", path.display()))?;
    temporary
        .as_file()
        .sync_all()
        .with_context(|| format!("could not sync generated content for {}", path.display()))?;
    if let Ok(metadata) = fs::metadata(path) {
        fs::set_permissions(temporary.path(), metadata.permissions())
            .with_context(|| format!("could not preserve permissions for {}", path.display()))?;
    }
    temporary
        .persist(path)
        .map(drop)
        .map_err(|error| error.error)
        .with_context(|| format!("could not replace {}", path.display()))
}
