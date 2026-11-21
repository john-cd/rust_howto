//! Validates the generated mdBook search-index files.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::path::Path;

use anyhow::Context as _;
use serde_json::Value;

/// Summary of the generated search-index validation.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct SearchIndexReport {
    /// Number of generated mdBook index files found.
    pub index_count: usize,

    /// Missing required terms, leaked content, and malformed index diagnostics.
    pub findings: Vec<String>,
}

/// Checks index JSON validity, required visible terms, and the draft exclusion policy.
///
/// # Arguments
///
/// * `book_root` - Book source root whose generated output is under `book/html`.
///
/// # Errors
///
/// Returns an error if the generated index directory cannot be enumerated or read.
pub fn check_search_index(book_root: &Path) -> anyhow::Result<SearchIndexReport> {
    let html_root = book_root.join("book/html");
    let entries = match fs::read_dir(&html_root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(SearchIndexReport {
                index_count: 0,
                findings: vec![format!(
                    "No mdBook search index found under {}",
                    html_root.display()
                )],
            });
        }
        Err(error) => {
            return Err(anyhow::anyhow!(
                "could not list search indexes under {}: {error}",
                html_root.display()
            ));
        }
    };
    let mut indexes = Vec::new();
    for entry in entries {
        let entry = entry.with_context(|| format!("could not list {}", html_root.display()))?;
        let path = entry.path();
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| {
                name.starts_with("searchindex-")
                    && Path::new(name)
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("js"))
            })
        {
            indexes.push(path);
        }
    }
    indexes.sort();
    let mut report = SearchIndexReport {
        index_count: indexes.len(),
        findings: Vec::new(),
    };
    if indexes.is_empty() {
        report.findings.push(format!(
            "No mdBook search index found under {}",
            html_root.display()
        ));
        return Ok(report);
    }

    let mut search_index = String::new();
    let mut document_urls = Vec::new();
    for path in &indexes {
        let text = fs::read_to_string(path)
            .map_err(|error| anyhow::anyhow!("could not read {}: {error}", path.display()))?;
        search_index.push_str(&text);
        match parse_index_json(&text) {
            Ok(value) => match value.get("doc_urls").and_then(Value::as_array) {
                Some(urls) => {
                    for (index, url) in urls.iter().enumerate() {
                        if let Some(url) = url.as_str() {
                            document_urls.push(url.to_owned());
                        } else {
                            report.findings.push(format!(
                                "{}: doc_urls entry {index} is not a string",
                                display_name(path)
                            ));
                        }
                    }
                }
                None => report.findings.push(format!(
                    "{}: search index does not contain a document URL list",
                    display_name(path)
                )),
            },
            Err(error) => report.findings.push(format!(
                "{}: invalid search-index JSON: {error}",
                display_name(path)
            )),
        }
    }

    let missing_terms = ["serde", "tokio"]
        .into_iter()
        .filter(|term| !search_index.contains(term))
        .collect::<Vec<_>>();
    if !missing_terms.is_empty() {
        report.findings.push(format!(
            "Search index is missing visible terms: {}",
            missing_terms.join(", ")
        ));
    }
    if search_index.contains("expecting") {
        report
            .findings
            .push("Hidden content leaked into the search index: expecting".to_owned());
    }
    let has_excluded_url = document_urls.iter().any(|url| {
        let path = url.split('#').next().unwrap_or(url);
        path.starts_with("drafts/") || path.starts_with("later/")
    });
    if has_excluded_url {
        report
            .findings
            .push("Draft or staging chapters leaked into the search index".to_owned());
    }
    Ok(report)
}

/// Extracts a JSON index from mdBook's `JSON.parse('...')` wrapper or a raw JSON object.
fn parse_index_json(source: &str) -> std::result::Result<Value, String> {
    let json = if let Some(start) = source.find("JSON.parse('") {
        const PREFIX: &str = "JSON.parse('";
        let mut decoded = String::new();
        let mut escaped = false;
        let mut closed = false;
        let payload_start = start + PREFIX.len();
        let payload = source
            .get(payload_start..)
            .ok_or_else(|| "JSON.parse marker is not on a character boundary".to_owned())?;
        for character in payload.chars() {
            if escaped {
                match character {
                    '\\' | '\'' => decoded.push(character),
                    other => {
                        decoded.push('\\');
                        decoded.push(other);
                    }
                }
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '\'' {
                closed = true;
                break;
            } else {
                decoded.push(character);
            }
        }
        if escaped || !closed {
            return Err("unterminated JSON.parse string".to_owned());
        }
        decoded
    } else {
        let start = source
            .find('{')
            .ok_or_else(|| "JSON object was not found".to_owned())?;
        source
            .get(start..)
            .ok_or_else(|| "JSON object does not begin on a character boundary".to_owned())?
            .to_owned()
    };
    serde_json::from_str(&json).map_err(|error| error.to_string())
}

/// Returns a stable file label for diagnostics.
fn display_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map_or_else(|| path.display().to_string(), str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks one JSON.parse-wrapped mdBook shard and detects staging URLs.
    #[test]
    fn validates_wrapped_index_json_and_rejects_private_urls() -> anyhow::Result<()> {
        let directory = tempfile::tempdir()?;
        let index_dir = directory.path().join("book/html");
        fs::create_dir_all(&index_dir)?;
        fs::write(
            index_dir.join("searchindex-1.js"),
            "window.search = Object.assign(window.search, JSON.parse('{\"doc_urls\":[\"chapter.html\",\"later/preview.html\"],\"body\":\"serde tokio\"}'));",
        )?;

        let report = check_search_index(directory.path())?;
        assert_eq!(report.index_count, 1);
        assert!(
            report
                .findings
                .contains(&"Draft or staging chapters leaked into the search index".to_owned())
        );
        assert!(
            !report
                .findings
                .iter()
                .any(|finding| finding.contains("invalid"))
        );
        Ok(())
    }

    /// Rejects malformed JSON instead of accepting only a regex-matched URL list.
    #[test]
    fn rejects_malformed_search_index_json() -> anyhow::Result<()> {
        let directory = tempfile::tempdir()?;
        let index_dir = directory.path().join("book/html");
        fs::create_dir_all(&index_dir)?;
        fs::write(
            index_dir.join("searchindex-1.js"),
            "window.search = Object.assign(window.search, JSON.parse('{\"doc_urls\":[broken],\"body\":\"serde tokio\"}'));",
        )?;

        let report = check_search_index(directory.path())?;
        assert!(
            report
                .findings
                .iter()
                .any(|finding| finding.contains("invalid search-index JSON"))
        );
        Ok(())
    }

    /// Reports a missing output directory as a missing index, like the legacy check.
    #[test]
    fn missing_output_directory_reports_no_search_index() -> anyhow::Result<()> {
        let directory = tempfile::tempdir()?;
        let report = check_search_index(directory.path())?;
        assert_eq!(report.index_count, 0);
        assert!(report.findings[0].contains("No mdBook search index found"));
        Ok(())
    }
}
