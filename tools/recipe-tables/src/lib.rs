//! Synchronizes recipe table fragments from local references and Markdown headings.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::Context as _;
use anyhow::Result;
use percent_encoding::percent_decode_str;
use regex::Regex;
use tempfile::NamedTempFile;
use walkdir::WalkDir;

/// Matches local recipe reference definitions.
static REFERENCE_DEFINITION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?m)^\s*\[ex~(?P<label>(?:\\.|[^\]])+)\]:\s*(?:<(?P<bracket>[^>]+)>|(?P<bare>\S+))(?:[ \t]+.*)?$",
    )
    .expect("the recipe reference regex is valid")
});

/// Matches headings that define stable recipe anchors.
static HEADING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^ {0,3}(#{1,6})[ \t]+(.+?)[ \t]+\{#([^}]+)\}[ \t]*$")
        .expect("the recipe heading regex is valid")
});

/// Matches Markdown code-fence openers and closers.
static FENCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("^ {0,3}(`{3,}|~{3,})").expect("the Markdown fence regex is valid")
});

/// Matches mdBook hidden content that terminates a recipe table.
static HIDDEN_DIV: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)<div\s+class\s*=\s*(?:"hidden"|'hidden')"#)
        .expect("the hidden div regex is valid")
});

/// Matches include directives when checking whether a wrapper owns a fragment.
static INCLUDE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{\s*#include\s+[^}]*\}\}").expect("the include regex is valid")
});

/// Matches recipe badge links in a heading's content.
static BADGE_LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[!\[[^\]]*\]\[[^\]]+\]\]\[(?P<label>[^\]]+)\]")
        .expect("the recipe badge regex is valid")
});

/// Matches recipe links already present in the first table column.
static RECIPE_LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[(?P<title>(?:\\.|[^\]])*)\]\[ex~(?P<label>(?:\\.|[^\]])+)\]")
        .expect("the recipe table link regex is valid")
});

/// Matches one Markdown table separator cell.
static SEPARATOR_CELL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^:?-{3,}:?$").expect("the table separator regex is valid"));

/// One local recipe reference and its decoded heading anchor.
#[derive(Clone, Debug, Eq, PartialEq)]
struct RecipeReference {
    label: String,
    anchor: String,
}

/// A recipe heading title and its source location.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Heading {
    title: String,
}

/// Returns a line without its CRLF, LF, or CR ending.
fn line_body(line: &str) -> &str {
    line.strip_suffix("\r\n")
        .or_else(|| line.strip_suffix('\n'))
        .or_else(|| line.strip_suffix('\r'))
        .unwrap_or(line)
}

/// Returns a line's exact newline sequence.
fn line_ending(line: &str) -> &str {
    if line.ends_with("\r\n") {
        "\r\n"
    } else if line.ends_with('\n') {
        "\n"
    } else if line.ends_with('\r') {
        "\r"
    } else {
        ""
    }
}

/// Splits text into lines while retaining each original line ending.
fn split_lines_keep_ends(source: &str) -> Vec<String> {
    let bytes = source.as_bytes();
    let mut lines = Vec::new();
    let mut start = 0;
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'\n' => {
                index += 1;
                lines.push(
                    source
                        .get(start..index)
                        .expect("line endings occur at UTF-8 boundaries")
                        .to_owned(),
                );
                start = index;
            }
            b'\r' => {
                index += 1;
                if bytes.get(index) == Some(&b'\n') {
                    index += 1;
                }
                lines.push(
                    source
                        .get(start..index)
                        .expect("line endings occur at UTF-8 boundaries")
                        .to_owned(),
                );
                start = index;
            }
            _ => index += 1,
        }
    }
    if start < source.len() {
        lines.push(
            source
                .get(start..)
                .expect("line endings occur at UTF-8 boundaries")
                .to_owned(),
        );
    }
    lines
}

/// Selects the first line ending in a file, defaulting to LF for a new file.
fn preferred_ending(lines: &[String]) -> String {
    lines
        .iter()
        .map(|line| line_ending(line))
        .find(|ending| !ending.is_empty())
        .unwrap_or("\n")
        .to_owned()
}

/// Unescapes `CommonMark` punctuation, collapses whitespace, and folds label case.
fn normalize_label(label: &str) -> String {
    let mut unescaped = String::new();
    let mut characters = label.chars().peekable();
    while let Some(character) = characters.next() {
        if character == '\\' && characters.peek().is_some_and(char::is_ascii_punctuation) {
            continue;
        }
        unescaped.push(character);
    }
    unescaped
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Escapes pipes that would otherwise create Markdown table cells.
fn escape_table_pipes(text: &str) -> String {
    let mut result = String::new();
    let mut backslashes = 0;
    for character in text.chars() {
        if character == '|' && backslashes & 1 == 0 {
            result.push('\\');
        }
        result.push(character);
        if character == '\\' {
            backslashes += 1;
        } else {
            backslashes = 0;
        }
    }
    result
}

/// Splits a Markdown table row into cells, respecting escaped pipes.
fn split_table_cells(line: &str) -> Option<Vec<String>> {
    let body = line_body(line).trim_start_matches('\u{feff}');
    let mut separators = Vec::new();
    let mut backslashes = 0;
    for (index, character) in body.char_indices() {
        if character == '|' && backslashes & 1 == 0 {
            separators.push(index);
        }
        if character == '\\' {
            backslashes += 1;
        } else {
            backslashes = 0;
        }
    }
    if separators.is_empty() {
        return None;
    }
    let mut cells = Vec::with_capacity(separators.len() + 1);
    let mut start = 0;
    for separator in separators {
        cells.push(
            body.get(start..separator)
                .expect("pipe separators occur at UTF-8 boundaries")
                .trim()
                .to_owned(),
        );
        start = separator + 1;
    }
    cells.push(
        body.get(start..)
            .expect("pipe separators occur at UTF-8 boundaries")
            .trim()
            .to_owned(),
    );
    if cells.first().is_some_and(String::is_empty) {
        cells.remove(0);
    }
    if cells.last().is_some_and(String::is_empty) {
        cells.pop();
    }
    Some(cells)
}

/// Reports whether a row contains only Markdown table separator cells.
fn is_separator_row(line: &str) -> bool {
    split_table_cells(line).is_some_and(|cells| {
        !cells.is_empty()
            && cells
                .iter()
                .all(|cell| SEPARATOR_CELL.is_match(&cell.replace(' ', "")))
    })
}

/// Finds the first table whose first column is named `Recipe`.
fn find_recipe_header(lines: &[String]) -> Option<(usize, Vec<String>)> {
    lines.iter().enumerate().find_map(|(index, line)| {
        let cells = split_table_cells(line)?;
        cells
            .first()
            .is_some_and(|first| {
                first
                    .trim_start_matches('\u{feff}')
                    .eq_ignore_ascii_case("recipe")
            })
            .then_some((index, cells))
    })
}

/// Reports whether a reference points to an external URL or host.
fn is_external_reference(destination: &str) -> bool {
    if destination.starts_with("//") {
        return true;
    }
    let before_path = destination
        .split(['/', '?', '#'])
        .next()
        .unwrap_or_default();
    before_path.split_once(':').is_some_and(|(scheme, _)| {
        !scheme.is_empty()
            && scheme
                .chars()
                .next()
                .is_some_and(|character| character.is_ascii_alphabetic())
            && scheme.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '+' | '.' | '-')
            })
    })
}

/// Splits a local reference target into its URL path and fragment.
fn local_reference_parts(destination: &str) -> Option<(&str, &str)> {
    if is_external_reference(destination) {
        return None;
    }
    let (before_fragment, fragment) = destination
        .split_once('#')
        .map_or((destination, ""), |(path, fragment)| (path, fragment));
    let path = before_fragment
        .split_once('?')
        .map_or(before_fragment, |(path, _query)| path);
    Some((path, fragment))
}

/// Decodes a URL path or anchor using replacement for invalid UTF-8.
fn decode_url_component(value: &str) -> String {
    percent_decode_str(value).decode_utf8_lossy().into_owned()
}

/// Reads recipe references in a sibling reference file that resolve to the chapter.
fn parse_references(refs_path: &Path, chapter_path: &Path) -> Result<Vec<RecipeReference>> {
    let source = fs::read_to_string(refs_path)
        .with_context(|| format!("could not read {}", refs_path.display()))?;
    let chapter_path = chapter_path
        .canonicalize()
        .with_context(|| format!("could not resolve {}", chapter_path.display()))?;
    let mut references = Vec::new();
    let mut seen = BTreeSet::new();
    for line in split_lines_keep_ends(&source) {
        let Some(captures) = REFERENCE_DEFINITION.captures(line_body(&line)) else {
            continue;
        };
        let label = captures
            .name("label")
            .expect("a matched definition has a label")
            .as_str();
        let destination = captures
            .name("bracket")
            .or_else(|| captures.name("bare"))
            .expect("a matched definition has a destination")
            .as_str();
        let Some((target, anchor)) = local_reference_parts(destination) else {
            continue;
        };
        let target = decode_url_component(target).replace('\\', "/");
        let target_path = if target.is_empty() {
            refs_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .to_path_buf()
        } else {
            refs_path
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .join(target)
        };
        if target_path.canonicalize().ok().as_deref() != Some(chapter_path.as_path()) {
            continue;
        }
        let normalized = normalize_label(label);
        if seen.insert(normalized) {
            references.push(RecipeReference {
                label: label.to_owned(),
                anchor: decode_url_component(anchor),
            });
        }
    }
    Ok(references)
}

/// Finds stable anchored headings and their content blocks, excluding fenced code.
fn heading_blocks(
    chapter_text: &str,
) -> (BTreeMap<String, Heading>, BTreeMap<String, Vec<String>>) {
    let lines = split_lines_keep_ends(chapter_text);
    let mut headings = Vec::<(usize, String, String)>::new();
    let mut fence: Option<(char, usize)> = None;
    for (index, line) in lines.iter().enumerate() {
        if let Some(marker) = FENCE
            .captures(line_body(line))
            .and_then(|capture| capture.get(1))
        {
            let marker = marker.as_str();
            let character = marker.chars().next().expect("fence marker is non-empty");
            if let Some((open_character, open_len)) = fence {
                if character == open_character && marker.len() >= open_len {
                    fence = None;
                }
            } else {
                fence = Some((character, marker.len()));
            }
            continue;
        }
        if fence.is_some() {
            continue;
        }
        if let Some(captures) = HEADING.captures(line_body(line)) {
            let title = captures
                .get(2)
                .expect("heading has a title")
                .as_str()
                .trim();
            let anchor = captures
                .get(3)
                .expect("heading has an anchor")
                .as_str()
                .trim();
            headings.push((index, title.to_owned(), anchor.to_owned()));
        }
    }

    let mut titles = BTreeMap::new();
    let mut blocks = BTreeMap::new();
    for (position, (start, title, anchor)) in headings.iter().enumerate() {
        let end = headings
            .get(position + 1)
            .map_or(lines.len(), |heading| heading.0);
        titles.insert(
            anchor.clone(),
            Heading {
                title: title.clone(),
            },
        );
        blocks.insert(anchor.clone(), lines[*start..end].to_vec());
    }
    (titles, blocks)
}

/// Collects unique crate or category badges from a heading block.
fn badges(block: &[String], categories: bool) -> String {
    let mut result = Vec::new();
    let mut seen = BTreeSet::new();
    for line in block {
        for capture in BADGE_LINK.captures_iter(line_body(line)) {
            let label = capture
                .name("label")
                .expect("a badge link has a label")
                .as_str();
            let include = if categories {
                label.starts_with("cat~")
            } else {
                label.starts_with("c~")
                    && ["~docs", "~website", "~repo"]
                        .iter()
                        .any(|suffix| label.ends_with(suffix))
            };
            if include && seen.insert(label.to_lowercase()) {
                result.push(
                    capture
                        .get(0)
                        .expect("badge match exists")
                        .as_str()
                        .to_owned(),
                );
            }
        }
    }
    result.join(" ")
}

/// Returns the first line after the recipe table's rows.
fn table_end(lines: &[String], header_index: usize) -> usize {
    let mut index = header_index + 1;
    if lines.get(index).is_some_and(|line| is_separator_row(line)) {
        index += 1;
    }
    while let Some(line) = lines.get(index) {
        let body = line_body(line);
        if body.trim().is_empty() || HIDDEN_DIV.is_match(body) || split_table_cells(line).is_none()
        {
            break;
        }
        index += 1;
    }
    index
}

/// Collects normalized recipe labels already present in the table.
fn existing_labels(lines: &[String], header_index: usize, end_index: usize) -> BTreeSet<String> {
    let mut labels = BTreeSet::new();
    for line in &lines[header_index + 1..end_index] {
        if is_separator_row(line) {
            continue;
        }
        let Some(cells) = split_table_cells(line) else {
            continue;
        };
        for capture in RECIPE_LINK.captures_iter(&cells[0]) {
            labels.insert(normalize_label(
                capture
                    .name("label")
                    .expect("recipe link has a label")
                    .as_str(),
            ));
        }
    }
    labels
}

/// Creates one Markdown row using the table's declared columns.
fn recipe_row(
    reference: &RecipeReference,
    heading: Option<&Heading>,
    columns: &[String],
    blocks: &BTreeMap<String, Vec<String>>,
) -> String {
    let fallback = reference
        .label
        .rsplit('~')
        .next()
        .unwrap_or(&reference.label);
    let title = heading.map_or_else(
        || fallback.replace('-', " "),
        |heading| heading.title.clone(),
    );
    let mut cells = vec![format!("[{title}][ex~{}]", reference.label)];
    cells.resize(columns.len(), String::new());
    if let Some(block) = blocks.get(&reference.anchor) {
        for (index, column) in columns.iter().enumerate().skip(1) {
            let normalized = column.to_lowercase();
            if normalized.contains("crate") {
                cells[index] = badges(block, false);
            } else if normalized.contains("categor") {
                cells[index] = badges(block, true);
            }
        }
    }
    format!(
        "| {} |",
        cells
            .iter()
            .map(|cell| escape_table_pipes(cell))
            .collect::<Vec<_>>()
            .join(" | ")
    )
}

/// Updates current titles without changing surrounding row content or line endings.
fn update_existing_titles(
    lines: &mut [String],
    header_index: usize,
    end_index: usize,
    references: &BTreeMap<String, RecipeReference>,
    titles: &BTreeMap<String, Heading>,
) -> bool {
    let mut changed = false;
    for line in lines.iter_mut().take(end_index).skip(header_index + 1) {
        if is_separator_row(line) {
            continue;
        }
        let body = line_body(line);
        let Some(capture) = RECIPE_LINK.captures(body) else {
            continue;
        };
        let label = capture
            .name("label")
            .expect("recipe link has a label")
            .as_str();
        let Some(reference) = references.get(&normalize_label(label)) else {
            continue;
        };
        let Some(title) = titles.get(&reference.anchor) else {
            continue;
        };
        let range = capture.get(0).expect("recipe link match exists");
        let replacement = escape_table_pipes(&format!("[{}][ex~{label}]", title.title));
        let mut updated = String::with_capacity(body.len() + replacement.len());
        updated.push_str(
            body.get(..range.start())
                .expect("the recipe link begins at a character boundary"),
        );
        updated.push_str(&replacement);
        updated.push_str(
            body.get(range.end()..)
                .expect("the recipe link ends at a character boundary"),
        );
        if updated != body {
            let ending = line_ending(line).to_owned();
            *line = updated + &ending;
            changed = true;
        }
    }
    changed
}

/// Checks whether a chapter explicitly includes its own recipe-table fragment.
fn wrapper_includes(chapter_path: &Path, include_path: &Path) -> Result<bool> {
    if !chapter_path.exists() {
        return Ok(false);
    }
    let source = fs::read_to_string(chapter_path)
        .with_context(|| format!("could not read {}", chapter_path.display()))?;
    let name = include_path
        .file_name()
        .and_then(|name| name.to_str())
        .context("recipe fragment file name is not valid UTF-8")?;
    Ok(INCLUDE.find_iter(&source).any(|directive| {
        directive
            .as_str()
            .strip_suffix("}}")
            .is_some_and(|body| body.trim_end().ends_with(name))
    }))
}

/// Atomically replaces a recipe fragment, keeping its existing filesystem permissions.
fn write_atomically(path: &Path, source: &str) -> Result<()> {
    let permissions = match fs::metadata(path) {
        Ok(metadata) => Some(metadata.permissions()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => {
            return Err(error).with_context(|| format!("could not inspect {}", path.display()));
        }
    };
    let parent = path
        .parent()
        .context("recipe fragment path has no parent directory")?;
    let mut temporary = NamedTempFile::new_in(parent)
        .with_context(|| format!("could not create temporary file beside {}", path.display()))?;
    temporary.write_all(source.as_bytes()).with_context(|| {
        format!(
            "could not write temporary recipe fragment for {}",
            path.display()
        )
    })?;
    if let Some(permissions) = permissions {
        temporary
            .as_file()
            .set_permissions(permissions)
            .with_context(|| format!("could not preserve permissions on {}", path.display()))?;
    }
    temporary.as_file().sync_all().with_context(|| {
        format!(
            "could not flush temporary recipe fragment for {}",
            path.display()
        )
    })?;
    temporary
        .persist(path)
        .map_err(|error| error.error)
        .with_context(|| format!("could not atomically replace {}", path.display()))?;
    Ok(())
}

/// Updates one chapter's recipe table include when its local references need it.
///
/// # Arguments
///
/// * `chapter_path` - Markdown chapter with a sibling `refs.incl.md` file.
///
/// # Errors
///
/// Returns an error if a required source file cannot be read or an include cannot be replaced.
pub fn update_chapter(chapter_path: &Path) -> Result<bool> {
    let parent = chapter_path.parent().unwrap_or_else(|| Path::new("."));
    let refs_path = parent.join("refs.incl.md");
    if !refs_path.is_file() {
        return Ok(false);
    }
    let references = parse_references(&refs_path, chapter_path)?;
    if references.is_empty() {
        return Ok(false);
    }

    let include_path = chapter_path.with_extension("incl.md");
    let mut is_new_include = !include_path.exists();
    if is_new_include && !wrapper_includes(chapter_path, &include_path)? {
        eprintln!(
            "Skipping {}: wrapper does not include this fragment",
            include_path.display()
        );
        return Ok(false);
    }

    let old_source = if is_new_include {
        String::new()
    } else {
        fs::read_to_string(&include_path)
            .with_context(|| format!("could not read {}", include_path.display()))?
    };
    let mut lines = split_lines_keep_ends(&old_source);
    let mut ending = preferred_ending(&lines);
    if is_new_include || !lines.iter().any(|line| !line_body(line).trim().is_empty()) {
        lines = vec![
            format!("| Recipe | Crates | Categories |{ending}"),
            format!("|--------|--------|------------|{ending}"),
        ];
        is_new_include = true;
    }

    let Some((header_index, columns)) = find_recipe_header(&lines) else {
        eprintln!(
            "Skipping {}: no recipe table header found",
            include_path.display()
        );
        return Ok(false);
    };
    let end_index = table_end(&lines, header_index);
    let chapter_text = fs::read_to_string(chapter_path)
        .with_context(|| format!("could not read {}", chapter_path.display()))?;
    let (titles, blocks) = heading_blocks(&chapter_text);
    let reference_map = references
        .iter()
        .map(|reference| (normalize_label(&reference.label), reference.clone()))
        .collect::<BTreeMap<_, _>>();

    let mut changed =
        update_existing_titles(&mut lines, header_index, end_index, &reference_map, &titles);
    let mut present = existing_labels(&lines, header_index, end_index);
    let mut new_rows = Vec::new();
    for reference in &references {
        if !present.insert(normalize_label(&reference.label)) {
            continue;
        }
        new_rows.push(recipe_row(
            reference,
            titles.get(&reference.anchor),
            &columns,
            &blocks,
        ));
    }

    if !new_rows.is_empty() {
        let insertion_index = table_end(&lines, header_index);
        ending = preferred_ending(&lines);
        let at_end = insertion_index == lines.len();
        let had_final_newline = lines
            .last()
            .is_some_and(|line| !line_ending(line).is_empty());
        if insertion_index > 0 && line_ending(&lines[insertion_index - 1]).is_empty() {
            lines[insertion_index - 1].push_str(&ending);
        }
        let mut inserted = new_rows
            .into_iter()
            .map(|row| row + &ending)
            .collect::<Vec<_>>();
        if at_end
            && !had_final_newline
            && let Some(last) = inserted.last_mut()
        {
            *last = line_body(last).to_owned();
        }
        lines.splice(insertion_index..insertion_index, inserted);
        changed = true;
    }

    if changed || is_new_include {
        let source = lines.concat();
        write_atomically(&include_path, &source)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Returns whether a Markdown source file is a chapter rather than an include or index.
fn is_chapter(path: &Path) -> bool {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_lowercase();
    !name.ends_with(".incl.md")
        && !name.ends_with("index.md")
        && !name.ends_with("refs.md")
        && name != "toreview.md"
}

/// Updates all recipe table fragments under an mdBook source root.
///
/// Both `src/` and a legacy optional `drafts/` directory are scanned in stable path order.
///
/// # Arguments
///
/// * `root` - mdBook root containing `src/` and optionally `drafts/`.
///
/// # Errors
///
/// Returns an error if a directory or chapter cannot be read or an include cannot be replaced.
pub fn update_book(root: &Path) -> Result<Vec<PathBuf>> {
    let mut changed = Vec::new();
    for content_dir in [root.join("src"), root.join("drafts")] {
        if !content_dir.is_dir() {
            continue;
        }
        let mut chapters = Vec::new();
        for entry in WalkDir::new(&content_dir).follow_links(false) {
            let entry =
                entry.with_context(|| format!("could not walk {}", content_dir.display()))?;
            if entry.path().is_file()
                && entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension.to_string_lossy().eq_ignore_ascii_case("md"))
                && is_chapter(entry.path())
            {
                chapters.push(entry.into_path());
            }
        }
        chapters.sort();
        for chapter in chapters {
            if update_chapter(&chapter)? {
                changed.push(chapter.with_extension("incl.md"));
            }
        }
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates chapter, references, and include files in a temporary book tree.
    fn fixture(
        directory: &Path,
        chapter: &str,
        references: &str,
        include: &str,
    ) -> Result<PathBuf> {
        let source = directory.join("src/chapter");
        fs::create_dir_all(&source)?;
        let chapter_path = source.join(chapter);
        fs::write(&chapter_path, chapter_content_placeholder())?;
        fs::write(source.join("refs.incl.md"), references)?;
        if !include.is_empty() {
            fs::write(chapter_path.with_extension("incl.md"), include)?;
        }
        Ok(chapter_path)
    }

    /// Supplies placeholder chapter content before a test replaces it with exact content.
    fn chapter_content_placeholder() -> &'static str {
        ""
    }

    /// Synchronizes special characters in a title without breaking recipe links or reruns.
    #[test]
    fn existing_titles_escape_table_pipes_and_remain_idempotent() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let chapter = fixture(
            directory.path(),
            "table.md",
            "[ex~existing+(v1)?]: table.md#title\n",
            "| Recipe | Crates | Categories |\n|--------|--------|------------|\n| [Stale title][ex~existing+(v1)?] | crate | category |\n",
        )?;
        fs::write(
            &chapter,
            r"## C:\tmp & API | v1 + (x.y)? {#title}".to_owned() + "\n",
        )?;
        assert!(update_chapter(&chapter)?);
        let include = chapter.with_extension("incl.md");
        let generated = fs::read_to_string(&include)?;
        assert!(generated.contains(r"[C:\tmp & API \| v1 + (x.y)?][ex~existing+(v1)?]"));
        assert!(!update_chapter(&chapter)?);
        assert_eq!(generated, fs::read_to_string(include)?);
        Ok(())
    }

    /// Adds unique crate and category badges before hidden content and is idempotent.
    #[test]
    fn three_columns_add_badges_and_preserve_following_content() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let chapter = fixture(
            directory.path(),
            "tables.md",
            "[ex~existing]: tables.md#existing\n[ex~new]: tables.md#new\n",
            "| Recipe | Crates | Categories |\n|--------|--------|------------|\n| [Old title][ex~existing] |  |  |\n<div class=\"hidden\">\nprivate refs\n</div>\n",
        )?;
        fs::write(
            &chapter,
            "## Existing {#existing}\n[![serde][c~serde~docs~badge]][c~serde~docs] [![cat~data][cat~data~badge]][cat~data]\n## New title & API | usage {#new}\n[![serde][c~serde~docs~badge]][c~serde~docs] [![cat~data][cat~data~badge]][cat~data]\n",
        )?;
        assert!(update_chapter(&chapter)?);
        let include = chapter.with_extension("incl.md");
        let generated = fs::read_to_string(&include)?;
        let expected = r"| [New title & API \| usage][ex~new] | [![serde][c~serde~docs~badge]][c~serde~docs] | [![cat~data][cat~data~badge]][cat~data] |";
        assert!(generated.contains(expected));
        assert!(generated.contains("| [Existing][ex~existing] |  |  |"));
        assert!(generated.find(expected) < generated.find("<div class=\"hidden\">"));
        assert!(generated.contains("private refs"));
        assert!(!update_chapter(&chapter)?);
        assert_eq!(generated, fs::read_to_string(include)?);
        Ok(())
    }

    /// Uses declared columns and matches escaped punctuation in reference labels.
    #[test]
    fn two_columns_add_category_badges_and_decode_paths_with_spaces() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let source = directory.path().join("book root with spaces/src/category");
        fs::create_dir_all(&source)?;
        let chapter = source.join("chapter with spaces.md");
        fs::write(
            source.join("refs.incl.md"),
            "[ex~regex+name?(v1)]: <chapter with spaces.md#anchor>\n",
        )?;
        fs::write(
            &chapter,
            "{{#include chapter with spaces.incl.md}}\n## Oil & gas | API {#anchor}\n[![cat~network][cat~network~badge]][cat~network]\n",
        )?;
        let include = chapter.with_extension("incl.md");
        fs::write(
            &include,
            "| Recipe | Categories |\n|--------|------------|\n",
        )?;
        assert!(update_chapter(&chapter)?);
        let generated = fs::read_to_string(include)?;
        assert!(generated.contains(r"[Oil & gas \| API][ex~regex+name?(v1)]"));
        assert!(generated.contains("[![cat~network][cat~network~badge]][cat~network]"));
        assert!(!generated.contains("categories | [![cat"));
        Ok(())
    }

    /// Adds a one-column fallback row when its reference anchor has no heading.
    #[test]
    fn one_column_missing_anchor_uses_reference_suffix() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let chapter = fixture(
            directory.path(),
            "one.md",
            "[ex~lost-anchor]: one.md#missing\n",
            "| Recipe |\n|--------|\n",
        )?;
        fs::write(&chapter, "{{#include one.incl.md}}\n")?;
        assert!(update_chapter(&chapter)?);
        assert!(
            fs::read_to_string(chapter.with_extension("incl.md"))?
                .contains("| [lost anchor][ex~lost-anchor] |")
        );
        Ok(())
    }

    /// Creates a missing include only when its chapter wrapper already owns that include.
    #[test]
    fn creates_missing_include_only_for_matching_wrapper() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let source = directory.path().join("src/chapter");
        fs::create_dir_all(&source)?;
        let chapter = source.join("missing with spaces.md");
        fs::write(
            source.join("refs.incl.md"),
            "[ex~anchor]: <missing with spaces.md#anchor>\n",
        )?;
        fs::write(
            &chapter,
            "{{#include missing with spaces.incl.md}}\n## Title {#anchor}\n",
        )?;
        let include = chapter.with_extension("incl.md");
        assert!(update_chapter(&chapter)?);
        let generated = fs::read_to_string(&include)?;
        assert!(generated.contains("| Recipe | Crates | Categories |"));
        assert!(generated.contains("| [Title][ex~anchor] |  |  |"));

        fs::remove_file(&include)?;
        fs::write(&chapter, "## Title {#anchor}\n")?;
        assert!(!update_chapter(&chapter)?);
        assert!(!include.exists());
        Ok(())
    }

    /// Retains CRLF endings and unrelated Markdown while inserting a recipe row.
    #[test]
    fn preserves_crlf_line_endings_and_unrelated_markdown() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let chapter = fixture(
            directory.path(),
            "line-endings.md",
            "[ex~one]: line-endings.md#one\r\n",
            "| Recipe | Crates |\r\n|--------|--------|\r\n| Existing content | unchanged |\r\n\r\nAfter table\r\n",
        )?;
        fs::write(&chapter, "## One {#one}\r\n")?;
        assert!(update_chapter(&chapter)?);
        let generated = fs::read(chapter.with_extension("incl.md"))?;
        assert!(generated.windows(2).all(|window| window != b"\n\n"));
        assert!(generated.windows(2).any(|window| window == b"\r\n"));
        let generated = String::from_utf8(generated)?;
        assert_eq!(
            generated.matches('\n').count(),
            generated.matches("\r\n").count()
        );
        assert!(generated.contains("After table\r\n"));
        Ok(())
    }

    /// Keeps the existing Unix mode when the fragment is atomically replaced.
    #[cfg(unix)]
    #[test]
    fn atomic_replacement_preserves_unix_permissions() -> Result<()> {
        use std::os::unix::fs::PermissionsExt as _;

        let directory = tempfile::tempdir()?;
        let chapter = fixture(
            directory.path(),
            "permissions.md",
            "[ex~one]: permissions.md#one\n",
            "| Recipe |\n|--------|\n",
        )?;
        fs::write(&chapter, "## One {#one}\n")?;
        let include = chapter.with_extension("incl.md");
        fs::set_permissions(&include, fs::Permissions::from_mode(0o640))?;

        assert!(update_chapter(&chapter)?);
        assert_eq!(fs::metadata(include)?.permissions().mode() & 0o777, 0o640);
        Ok(())
    }

    /// Ignores external definitions and duplicate normalized labels.
    #[test]
    fn references_skip_external_and_duplicate_labels() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let chapter = fixture(
            directory.path(),
            "definitions.md",
            "[ex~one]: https://example.com/chapter.md#one\n[ex~ One ]: definitions.md#one\n[ex~one]: definitions.md#two\n",
            "| Recipe |\n|--------|\n",
        )?;
        fs::write(&chapter, "## One {#one}\n## Two {#two}\n")?;
        assert!(update_chapter(&chapter)?);
        let generated = fs::read_to_string(chapter.with_extension("incl.md"))?;
        assert!(generated.contains("[One][ex~ One ]"));
        assert!(!generated.contains("[Two][ex~one]"));
        Ok(())
    }

    /// Scans published and legacy draft chapters while ignoring indexes and includes.
    #[test]
    fn updates_src_and_optional_drafts_in_stable_chapter_scope() -> Result<()> {
        let directory = tempfile::tempdir()?;
        for content_dir in ["src", "drafts"] {
            let chapter_dir = directory.path().join(content_dir).join("chapter");
            fs::create_dir_all(&chapter_dir)?;
            fs::write(
                chapter_dir.join("refs.incl.md"),
                "[ex~one]: example.md#one\n",
            )?;
            fs::write(
                chapter_dir.join("example.md"),
                "{{#include example.incl.md}}\n## One {#one}\n",
            )?;
            fs::write(
                chapter_dir.join("example.incl.md"),
                "| Recipe |\n|--------|\n",
            )?;
            fs::write(chapter_dir.join("index.md"), "Not a recipe chapter\n")?;
        }

        let changed = update_book(directory.path())?;
        assert_eq!(changed.len(), 2);
        assert!(changed.iter().all(|path| path.ends_with("example.incl.md")));
        Ok(())
    }
}
