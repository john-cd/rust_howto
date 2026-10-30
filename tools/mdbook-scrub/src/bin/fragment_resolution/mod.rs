//! Resolves local Markdown fragments for the mdBook linkcheck adapter.

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::sync::OnceLock;

use mdbook_markdown::MarkdownOptions;
use mdbook_markdown::new_cmark_parser;
use mdbook_markdown::pulldown_cmark::Event;
use mdbook_markdown::pulldown_cmark::Tag;
use mdbook_markdown::pulldown_cmark::TagEnd;
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::errors::Result;
use regex::Regex;
use serde_json::Value;

/// A local Markdown link whose target chapter does not define its fragment.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct InvalidFragment {
    /// The source chapter containing the link.
    pub(super) source: PathBuf,
    /// The chapter addressed by the link.
    pub(super) target: PathBuf,
    /// The decoded fragment identifier.
    pub(super) fragment: String,
}

/// Markdown chapter data needed to resolve local fragment links.
struct ChapterFragments {
    /// The path used to resolve links in this chapter.
    path: PathBuf,
    /// Alternate source path used by mdBook for files such as `README.md`.
    source_path: Option<PathBuf>,
    /// Heading and HTML identifiers emitted by the HTML renderer.
    identifiers: HashSet<String>,
    /// Local and external Markdown links found in the chapter.
    links: Vec<String>,
}

/// Finds local links to Markdown chapters whose fragments do not exist.
pub(super) fn find_invalid_fragments(context: &Value) -> Result<Vec<InvalidFragment>> {
    let source_root = source_root(context)?;
    let items = context
        .pointer("/book/items")
        .or_else(|| context.pointer("/book/sections"))
        .and_then(Value::as_array)
        .ok_or_else(|| Error::msg("RenderContext is missing its book items"))?;

    let mut chapters = Vec::new();
    collect_chapters(items, &source_root, &mut chapters);

    let mut identifiers_by_path = HashMap::new();
    for (index, chapter) in chapters.iter().enumerate() {
        insert_chapter_path_aliases(&chapter.path, index, &mut identifiers_by_path);
        if let Some(source_path) = &chapter.source_path {
            insert_chapter_path_aliases(source_path, index, &mut identifiers_by_path);
        }
    }

    let mut invalid = Vec::new();
    for chapter in &chapters {
        for destination in &chapter.links {
            let Some((target, fragment)) = resolve_target(&chapter.path, destination, &source_root)
            else {
                continue;
            };
            let Some(index) = identifiers_by_path.get(&path_key(&target)) else {
                continue;
            };
            if !chapters[*index].identifiers.contains(&fragment) {
                invalid.push(InvalidFragment {
                    source: chapter.path.clone(),
                    target: chapters[*index].path.clone(),
                    fragment,
                });
            }
        }
    }
    Ok(invalid)
}

/// Registers Markdown, rendered HTML, and directory aliases for a chapter.
fn insert_chapter_path_aliases(
    path: &Path,
    index: usize,
    identifiers_by_path: &mut HashMap<String, usize>,
) {
    identifiers_by_path.insert(path_key(path), index);

    let mut html_path = path.to_path_buf();
    html_path.set_extension("html");
    identifiers_by_path.insert(path_key(&html_path), index);

    if matches!(
        path.file_stem().and_then(|stem| stem.to_str()),
        Some("index" | "README")
    ) && let Some(parent) = path.parent()
    {
        identifiers_by_path.insert(path_key(parent), index);
        identifiers_by_path.insert(path_key(&parent.join("index.html")), index);
    }
}

/// Returns the book's absolute Markdown source directory.
fn source_root(context: &Value) -> Result<PathBuf> {
    let root = context
        .get("root")
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_default();
    let source = context
        .pointer("/config/book/src")
        .and_then(Value::as_str)
        .unwrap_or("src");
    Ok(normalize_path(&root.join(source)))
}

/// Collects chapter paths, identifiers, links, and nested chapters.
fn collect_chapters(items: &[Value], source_root: &Path, chapters: &mut Vec<ChapterFragments>) {
    for item in items {
        let Some(chapter) = item.get("Chapter") else {
            continue;
        };
        let summary_path = chapter.get("path").and_then(Value::as_str);
        let source_path = chapter
            .get("source_path")
            .and_then(Value::as_str)
            .or(summary_path);
        if let Some(path) = summary_path.or(source_path) {
            let content = chapter
                .get("content")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let path = normalize_path(&source_root.join(path));
            let source_path = source_path
                .filter(|source_path| Some(*source_path) != summary_path)
                .map(|source_path| normalize_path(&source_root.join(source_path)));
            chapters.push(ChapterFragments {
                path,
                source_path,
                identifiers: identifiers(content),
                links: links(content),
            });
        }
        if let Some(sub_items) = chapter.get("sub_items").and_then(Value::as_array) {
            collect_chapters(sub_items, source_root, chapters);
        }
    }
}

/// Extracts heading and explicit HTML identifiers from rendered Markdown.
fn identifiers(content: &str) -> HashSet<String> {
    let parser = new_cmark_parser(content, &MarkdownOptions::default());
    let mut ids = HashSet::new();
    let mut generated_ids = HashSet::new();
    let mut heading: Option<(Option<String>, String)> = None;
    for event in parser {
        match event {
            Event::Start(Tag::Heading { id, .. }) => {
                heading = Some((
                    id.map(mdbook_markdown::pulldown_cmark::CowStr::into_string),
                    String::new(),
                ));
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((explicit_id, text)) = heading.take() {
                    let id =
                        explicit_id.unwrap_or_else(|| unique_id(&slug(&text), &mut generated_ids));
                    ids.insert(id);
                }
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some((_, heading_text)) = &mut heading {
                    heading_text.push_str(&text);
                }
            }
            Event::SoftBreak => {
                if let Some((_, heading_text)) = &mut heading {
                    heading_text.push('\n');
                }
            }
            Event::HardBreak => {}
            Event::Html(html) | Event::InlineHtml(html) => {
                ids.extend(html_identifiers(&html));
            }
            _ => {}
        }
    }
    ids
}

/// Extracts exact `id` and anchor `name` attributes from HTML start tags.
fn html_identifiers(html: &str) -> Vec<String> {
    let mut identifiers = Vec::new();
    for tag in html_tag_regex().find_iter(html).map(|tag| tag.as_str()) {
        if tag.starts_with("<!--") {
            continue;
        }
        identifiers.extend(tag_identifiers(tag));
    }
    identifiers
}

/// Reads `id` and `name` values while skipping quoted attribute contents.
fn tag_identifiers(tag: &str) -> Vec<String> {
    let bytes = tag.as_bytes();
    let mut identifiers = Vec::new();
    let mut index = 1;
    while index < bytes.len() && !bytes[index].is_ascii_whitespace() {
        index += 1;
    }

    while index < bytes.len() {
        while index < bytes.len() && (bytes[index].is_ascii_whitespace() || bytes[index] == b'/') {
            index += 1;
        }
        let name_start = index;
        while index < bytes.len()
            && !bytes[index].is_ascii_whitespace()
            && !matches!(bytes[index], b'=' | b'>' | b'/')
        {
            index += 1;
        }
        if name_start == index {
            index += 1;
            continue;
        }
        let is_identifier = bytes[name_start..index].eq_ignore_ascii_case(b"id")
            || bytes[name_start..index].eq_ignore_ascii_case(b"name");
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() || bytes[index] != b'=' {
            continue;
        }
        index += 1;
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        let value = if index < bytes.len() && matches!(bytes[index], b'"' | b'\'') {
            let quote = bytes[index];
            index += 1;
            let value_start = index;
            while index < bytes.len() && bytes[index] != quote {
                index += 1;
            }
            let value = String::from_utf8_lossy(&bytes[value_start..index]).into_owned();
            index = index.saturating_add(1);
            value
        } else {
            let value_start = index;
            while index < bytes.len() && !bytes[index].is_ascii_whitespace() && bytes[index] != b'>'
            {
                index += 1;
            }
            String::from_utf8_lossy(&bytes[value_start..index]).into_owned()
        };

        if is_identifier {
            identifiers.push(value);
        }
    }
    identifiers
}

/// Returns an expression matching HTML comments and start tags.
fn html_tag_regex() -> &'static Regex {
    static HTML_TAG: OnceLock<Regex> = OnceLock::new();
    HTML_TAG.get_or_init(|| {
        Regex::new("(?is)<!--.*?-->|<[a-z][^>]*>").expect("the fixed HTML tag expression is valid")
    })
}

/// Extracts Markdown link destinations, excluding code blocks and link definitions.
fn links(content: &str) -> Vec<String> {
    let mut destinations = Vec::new();
    for event in new_cmark_parser(content, &MarkdownOptions::default()) {
        match event {
            Event::Start(Tag::Link { dest_url, .. }) => destinations.push(dest_url.into_string()),
            Event::Html(html) | Event::InlineHtml(html) => {
                for capture in html_href_regex().captures_iter(&html) {
                    if let Some(destination) = capture
                        .get(1)
                        .or_else(|| capture.get(2))
                        .or_else(|| capture.get(3))
                    {
                        destinations.push(destination.as_str().to_owned());
                    }
                }
            }
            _ => {}
        }
    }
    destinations
}

/// Returns the compiled expression used to read HTML `href` attributes.
fn html_href_regex() -> &'static Regex {
    static HTML_HREF: OnceLock<Regex> = OnceLock::new();
    HTML_HREF.get_or_init(|| {
        Regex::new(r#"(?is)\bhref\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s>]+))"#)
            .expect("the fixed HTML hyperlink expression is valid")
    })
}

/// Resolves a Markdown link into a known local chapter and decoded fragment.
fn resolve_target(
    source: &Path,
    destination: &str,
    source_root: &Path,
) -> Option<(PathBuf, String)> {
    let (path, fragment) = destination.split_once('#')?;
    if fragment.is_empty() || is_external_url(path) {
        return None;
    }
    let path = path.split_once('?').map_or(path, |(path, _)| path);
    let decoded_path = percent_decode(path);
    let target = if decoded_path.is_empty() {
        source.to_path_buf()
    } else if decoded_path.starts_with('/') || decoded_path.starts_with('\\') {
        source_root.join(decoded_path.trim_start_matches(['/', '\\']))
    } else {
        source.parent()?.join(decoded_path)
    };
    Some((normalize_path(&target), percent_decode(fragment)))
}

/// Identifies absolute URLs and protocol-relative URLs.
fn is_external_url(path: &str) -> bool {
    path.starts_with("//")
        || path
            .split_once(':')
            .is_some_and(|(scheme, _)| !scheme.contains('/') && !scheme.contains('\\'))
}

/// Decodes valid percent escapes while preserving plus signs and malformed escapes.
fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && let (Some(high), Some(low)) = (
                bytes.get(index + 1).and_then(|byte| hex_value(*byte)),
                bytes.get(index + 2).and_then(|byte| hex_value(*byte)),
            )
        {
            decoded.push(high * 16 + low);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

/// Converts an ASCII hexadecimal byte to its numeric value.
fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

/// Generates the mdBook HTML renderer's heading identifier from heading text.
fn slug(text: &str) -> String {
    text.trim()
        .to_lowercase()
        .chars()
        .filter_map(|character| {
            if character.is_alphanumeric() || character == '_' || character == '-' {
                Some(character)
            } else if character.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}

/// Adds the numeric suffix used by mdBook when a generated heading ID repeats.
fn unique_id(id: &str, used: &mut HashSet<String>) -> String {
    if used.insert(id.to_owned()) {
        return id.to_owned();
    }
    let mut suffix = 1_u32;
    loop {
        let candidate = format!("{id}-{suffix}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        suffix += 1;
    }
}

/// Normalizes path components without accessing the filesystem.
fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir if normalized.file_name().is_some() => {
                normalized.pop();
            }
            Component::ParentDir => normalized.push(component.as_os_str()),
            _ => normalized.push(component.as_os_str()),
        }
    }
    normalized
}

/// Creates a platform-aware key for comparing normalized chapter paths.
fn path_key(path: &Path) -> String {
    let path = path.to_string_lossy().replace('\\', "/");
    if cfg!(windows) {
        path.to_lowercase()
    } else {
        path
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    /// Verifies generated, explicit, duplicate, and HTML identifiers are recognized.
    #[test]
    fn extracts_mdbook_heading_and_html_identifiers() {
        let ids = identifiers(
            "# First heading\n\n## First heading\n\n### Explicit {#caseSensitive}\n\n<a data-id=\"not-an-id\" title=\"id='quoted-id'\" id=\"raw-anchor\"></a>\n<!-- <a id=\"comment-id\"></a> -->\n<a name='legacy-anchor'></a>\n",
        );
        assert!(ids.contains("first-heading"));
        assert!(ids.contains("first-heading-1"));
        assert!(ids.contains("caseSensitive"));
        assert!(ids.contains("raw-anchor"));
        assert!(ids.contains("legacy-anchor"));
        assert!(!ids.contains("not-an-id"));
        assert!(!ids.contains("comment-id"));
    }

    /// Verifies heading IDs against output rendered by mdBook 0.5.4.
    #[test]
    fn heading_ids_match_mdbook_html_output() {
        let ids = identifiers(
            "# Shared {#shared}\n\n# Shared\n\n# Shared\n\n# Crème &amp; Rust\n\n# Crème & Rust\n\n# Crème and Rust {#given-id}\n\n# Crème &amp; Rust\n\n## Escaped \\*heading\\*\n",
        );
        for id in [
            "shared",
            "shared-1",
            "crème--rust",
            "crème--rust-1",
            "given-id",
            "crème--rust-2",
            "escaped-heading",
        ] {
            assert!(
                ids.contains(id),
                "mdBook's rendered heading id `{id}` was missing"
            );
        }
    }

    /// Verifies rendered `.html` and directory links resolve to index chapters.
    #[test]
    fn resolves_rendered_html_and_directory_index_aliases() {
        let context = json!({
            "root": "/book",
            "config": {"book": {"src": "src"}},
            "book": {"items": [
                {"Chapter": {
                    "path": "links.md",
                    "content": "[html](guide/index.html#start) [directory](guide/#start) [missing](guide/#absent)"
                }},
                {"Chapter": {
                    "path": "guide/index.md",
                    "content": "# Start"
                }}
            ]}
        });

        let invalid = find_invalid_fragments(&context).expect("book structure should be valid");
        assert_eq!(invalid.len(), 1);
        assert_eq!(invalid[0].fragment, "absent");
        assert!(invalid[0].target.ends_with("guide/index.md"));
    }

    /// Verifies local links resolve while unknown, encoded, and external fragments are handled.
    #[test]
    fn finds_only_missing_local_markdown_fragments() {
        let context = json!({
            "root": "/book",
            "config": {"book": {"src": "src"}},
            "book": {"items": [
                {"Chapter": {
                    "path": "one.md",
                    "content": "# One\n\n[ok](#one) [encoded](two%20page.md#hello%2Dworld) [missing](two%20page.md#absent) [external](https://example.com/#remote) <a href=\"two%20page.md#no-such-id\">HTML link</a>"
                }},
                {"Chapter": {
                    "path": "two page.md",
                    "content": "# Hello world"
                }}
            ]}
        });
        let invalid = find_invalid_fragments(&context).expect("book structure should be valid");
        assert_eq!(invalid.len(), 2);
        assert!(invalid.iter().any(|item| item.fragment == "absent"));
        assert!(invalid.iter().any(|item| item.fragment == "no-such-id"));
        assert!(
            invalid
                .iter()
                .all(|item| item.target.ends_with("two page.md"))
        );
    }

    /// Verifies URL decoding preserves `+` and safely handles malformed escapes.
    #[test]
    fn percent_decode_preserves_plus_and_invalid_sequences() {
        assert_eq!(percent_decode("a+b%20c%Q1"), "a+b c%Q1");
    }
}
