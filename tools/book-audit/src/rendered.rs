//! Rendered HTML link, completeness, and Playground checks.
//!
//! TODO Reviewed ??-??-2026.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use html5ever::tendril::StrTendril;
use html5ever::tokenizer::BufferQueue;
use html5ever::tokenizer::TagKind;
use html5ever::tokenizer::Token;
use html5ever::tokenizer::TokenSink;
use html5ever::tokenizer::TokenSinkResult;
use html5ever::tokenizer::Tokenizer;
use html5ever::tokenizer::TokenizerOpts;
use percent_encoding::percent_decode_str;
use regex::Regex;
use url::Url;
use walkdir::WalkDir;

/// Base URL used to resolve links authored for the published GitHub Pages book.
const BOOK_URL: &str = "https://book.invalid/rust_howto/";

/// Pages whose entry links are always checked in a rendered publication.
const REQUIRED_PAGES: &[&str] = &[
    "index.html",
    "introduction.html",
    "introduction/index.html",
    "categories/development-tools/benchmark_regressions.html",
    "categories/development-tools/profiling.html",
    "appendices/contributing/mdBook_pre-processors.html",
    "appendices/contributing/documentation_generation.html",
];

/// Finds unresolved authored reference labels outside code and scripts.
static UNRESOLVED_REFERENCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\]\[([a-z][a-z0-9_-]*~[^\]\s]+)\]")
        .expect("the unresolved-reference regex is valid")
});

/// Removes rendered code and scripts before inspecting prose for unresolved references.
static CODE_AND_SCRIPT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)<pre\b.*?</pre>|<script\b.*?</script>")
        .expect("the code and script regex is valid")
});

/// Finds the URL in an HTML meta-refresh directive.
static META_REFRESH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i);\s*url\s*=\s*(.+)").expect("the meta-refresh regex is valid")
});

/// Finds imported crate names in runnable Rust snippets.
static USE_IMPORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^\s*(?:pub\s+)?use\s+(?:::)?(\w+)\s*(?:::|\bas\b|;)")
        .expect("the use-import regex is valid")
});

/// Finds extern-crate declarations in runnable Rust snippets.
static EXTERN_IMPORT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^\s*(?:pub\s+)?extern\s+crate\s+(\w+)")
        .expect("the extern-crate regex is valid")
});

/// Finds local module declarations that can satisfy an imported crate name.
static LOCAL_MODULE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^\s*(?:pub\s+)?mod\s+(\w+)\s*[{;]").expect("the local-module regex is valid")
});

/// One runnable Playground block collected from an HTML page.
#[derive(Debug, Default)]
struct PlaygroundBlock {
    /// One-based rendered HTML line where the block begins.
    line: u64,

    /// Whether mdBook marked the block as `no_run`.
    no_run: bool,

    /// Whether the block contained an opening `code` tag.
    has_code: bool,

    /// Whether character tokens currently belong to the `code` element.
    in_code: bool,

    /// Decoded text contents of the code element.
    code: String,
}

/// Link, anchor, and runnable-block data extracted from one HTML document.
#[derive(Debug, Default)]
struct Page {
    /// Link and meta-refresh targets in document order.
    targets: Vec<HtmlTarget>,

    /// IDs and legacy named anchors in the page.
    anchors: BTreeSet<String>,

    /// Code blocks inside rendered Playground containers.
    playground: Vec<PlaygroundBlock>,
}

/// One rendered link or refresh target and its HTML source line.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct HtmlTarget {
    /// Link or redirect URL value.
    href: String,

    /// One-based line where the target element begins.
    line: u64,
}

/// HTML5 token sink collecting the elements used by the rendered checks.
#[derive(Debug, Default)]
struct PageSink {
    /// Link and redirect targets.
    targets: RefCell<Vec<HtmlTarget>>,

    /// IDs and legacy anchor names.
    anchors: RefCell<BTreeSet<String>>,

    /// Completed runnable blocks.
    playground: RefCell<Vec<PlaygroundBlock>>,

    /// Playground block currently being tokenized.
    active_playground: RefCell<Option<PlaygroundBlock>>,
}

impl TokenSink for PageSink {
    type Handle = ();

    fn process_token(&self, token: Token, line_number: u64) -> TokenSinkResult<Self::Handle> {
        match token {
            Token::TagToken(tag) if tag.kind == TagKind::StartTag => {
                let name = tag.name.as_ref();
                let attr = |wanted: &str| {
                    tag.attrs
                        .iter()
                        .find(|attribute| attribute.name.local.as_ref() == wanted)
                        .map(|attribute| attribute.value.to_string())
                };

                if let Some(id) = attr("id").filter(|value| !value.is_empty()) {
                    self.anchors.borrow_mut().insert(id);
                }
                if name == "a" {
                    if let Some(anchor) = attr("name").filter(|value| !value.is_empty()) {
                        self.anchors.borrow_mut().insert(anchor);
                    }
                    if let Some(href) = attr("href").filter(|value| !value.is_empty()) {
                        self.targets.borrow_mut().push(HtmlTarget {
                            href,
                            line: line_number,
                        });
                    }
                }
                if name == "meta"
                    && attr("http-equiv").is_some_and(|value| value.eq_ignore_ascii_case("refresh"))
                    && let Some(content) = attr("content")
                    && let Some(value) = META_REFRESH.captures(&content).and_then(|m| m.get(1))
                {
                    self.targets.borrow_mut().push(HtmlTarget {
                        href: value.as_str().trim_matches(['\'', '"']).to_owned(),
                        line: line_number,
                    });
                }
                if name == "pre" {
                    *self.active_playground.borrow_mut() =
                        (attr("class").as_deref() == Some("playground")).then(|| PlaygroundBlock {
                            line: line_number,
                            ..PlaygroundBlock::default()
                        });
                } else if name == "code"
                    && let Some(block) = self.active_playground.borrow_mut().as_mut()
                {
                    block.has_code = true;
                    block.in_code = true;
                    block.no_run = tag.attrs.iter().any(|attribute| {
                        attribute.name.local.as_ref().contains("no_run")
                            || attribute.value.contains("no_run")
                    });
                }
            }
            Token::TagToken(tag) if tag.kind == TagKind::EndTag => match tag.name.as_ref() {
                "code" => {
                    if let Some(block) = self.active_playground.borrow_mut().as_mut() {
                        block.in_code = false;
                    }
                }
                "pre" => {
                    if let Some(block) = self.active_playground.borrow_mut().take()
                        && block.has_code
                    {
                        self.playground.borrow_mut().push(block);
                    }
                }
                _ => {}
            },
            Token::CharacterTokens(text) => {
                if let Some(block) = self.active_playground.borrow_mut().as_mut()
                    && block.in_code
                {
                    block.code.push_str(&text);
                }
            }
            _ => {}
        }
        TokenSinkResult::Continue
    }
}

impl PageSink {
    /// Moves the collected document data into a stable page record.
    fn finish(self) -> Page {
        Page {
            targets: self.targets.into_inner(),
            anchors: self.anchors.into_inner(),
            playground: self.playground.into_inner(),
        }
    }
}

/// Parses HTML into links, anchors, and runnable Playground code blocks.
fn parse_page(html: &str) -> Page {
    let input = BufferQueue::default();
    input.push_back(StrTendril::from(html));
    let tokenizer = Tokenizer::new(PageSink::default(), TokenizerOpts::default());
    let _ = tokenizer.feed(&input);
    tokenizer.end();
    tokenizer.sink.finish()
}

/// Reads and parses a rendered HTML page.
fn read_page(path: &Path) -> Result<Page> {
    let html =
        fs::read_to_string(path).with_context(|| format!("could not read {}", path.display()))?;
    Ok(parse_page(&html))
}

/// Checks required rendered pages and authored local links.
///
/// # Arguments
///
/// * `html_root` - Directory containing the generated mdBook HTML.
/// * `all_chapters` - Whether to discover and check every rendered chapter and fragment.
///
/// # Errors
///
/// Returns an error if the HTML root cannot be read or a rendered page is invalid UTF-8.
pub fn check_rendered_links(html_root: &Path, all_chapters: bool) -> Result<Vec<String>> {
    let html_root = canonical_html_root(html_root)?;
    let mut sources = REQUIRED_PAGES
        .iter()
        .map(|page| (*page).to_owned())
        .collect::<BTreeSet<_>>();
    let mut failures = Vec::new();
    if all_chapters {
        match rendered_chapter_paths(&html_root) {
            Ok(chapters) => sources.extend(chapters),
            Err(error) => failures.push(error),
        }
        for path in html_files(&html_root)? {
            let relative = slash_path(path.strip_prefix(&html_root).expect("walked below root"));
            if relative != "word_index.html" {
                sources.insert(relative);
            }
        }
    }
    failures.extend(check_chapters(
        &html_root,
        &sources.into_iter().collect::<Vec<_>>(),
        all_chapters,
    )?);
    Ok(failures)
}

/// Checks local fragments linked from the rendered word index.
///
/// # Arguments
///
/// * `html_root` - Directory containing the generated mdBook HTML.
///
/// # Errors
///
/// Returns an error if the HTML root or a required page cannot be read.
pub fn check_word_index(html_root: &Path) -> Result<Vec<String>> {
    let html_root = canonical_html_root(html_root)?;
    let index_path = html_root.join("word_index.html");
    if !index_path.is_file() {
        return Ok(vec!["missing required page: word_index.html".to_owned()]);
    }
    if index_path.metadata()?.len() == 0 {
        return Ok(vec!["empty required page: word_index.html".to_owned()]);
    }
    let page = read_page(&index_path)?;
    let source_url = Url::parse(BOOK_URL)?.join("word_index.html")?;
    let mut chapter_anchors = BTreeMap::<PathBuf, BTreeSet<String>>::new();
    let mut failures = Vec::new();
    let mut links = page.targets;
    links.sort();
    links.dedup_by(|left, right| left.href == right.href);
    for link in links {
        let href = &link.href;
        if is_absolute_or_authority(href) {
            continue;
        }
        let Ok(target) = source_url.join(href) else {
            continue;
        };
        let Some(fragment) = target.fragment().filter(|fragment| !fragment.is_empty()) else {
            continue;
        };
        let Some(chapter) = local_book_path(&html_root, &target) else {
            failures.push(format!(
                "word_index.html:{}: {href} leaves the book",
                link.line
            ));
            continue;
        };
        let anchors = if let Some(anchors) = chapter_anchors.get(&chapter) {
            anchors
        } else {
            let anchors = if chapter.is_file() {
                read_page(&chapter)?.anchors
            } else {
                BTreeSet::new()
            };
            chapter_anchors.entry(chapter.clone()).or_insert(anchors)
        };
        let fragment = decode_component(fragment);
        if !anchors.contains(&fragment) {
            failures.push(format!(
                "word_index.html:{}: missing anchor {href}",
                link.line
            ));
        }
    }
    Ok(failures)
}

/// Checks imports in runnable Playground blocks against the pinned crate list.
///
/// # Arguments
///
/// * `html_root` - Directory containing rendered book pages.
/// * `book_root` - Book source root containing the example lockfile and crate list.
///
/// # Errors
///
/// Returns an error if required book files or rendered pages cannot be read.
pub fn check_playground(html_root: &Path, book_root: &Path) -> Result<Vec<String>> {
    let html_root = canonical_html_root(html_root)?;
    let lock_path = book_root.join("crates/Cargo.lock");
    let lock: toml::Value = toml::from_str(
        &fs::read_to_string(&lock_path)
            .with_context(|| format!("could not read {}", lock_path.display()))?,
    )
    .with_context(|| format!("could not parse {}", lock_path.display()))?;
    let dependencies = lock
        .get("package")
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|package| package.get("name").and_then(toml::Value::as_str))
        .map(|name| name.replace('-', "_"))
        .collect::<BTreeSet<_>>();
    let available_path = book_root.join("master/playground-crates.txt");
    let available = fs::read_to_string(&available_path)
        .with_context(|| format!("could not read {}", available_path.display()))?
        .split_whitespace()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();

    let index_path = html_root.join("index.html");
    if !index_path.is_file() {
        return Ok(vec![format!("{}: missing book index", html_root.display())]);
    }
    if index_path.metadata()?.len() == 0 {
        return Ok(vec![format!("{}: empty book index", html_root.display())]);
    }
    let mut failures = Vec::new();
    for path in html_files(&html_root)? {
        let page = read_page(&path)?;
        let relative = slash_path(path.strip_prefix(&html_root).expect("walked below root"));
        for (number, block) in page.playground.iter().enumerate() {
            if block.no_run {
                continue;
            }
            let missing = unavailable_playground_imports(&block.code, &dependencies, &available);
            if !missing.is_empty() {
                failures.push(format!(
                    "{relative}:{}: runnable block {} imports {}",
                    block.line,
                    number + 1,
                    missing.into_iter().collect::<Vec<_>>().join(", ")
                ));
            }
        }
    }
    Ok(failures)
}

/// Checks a staged preview's chapter links using its Summary and deferred-page policy.
///
/// # Arguments
///
/// * `html_root` - Directory containing the staged rendered pages.
/// * `summary_path` - Markdown Summary that defines the preview publication scope.
/// * `include_deferred` - Whether to check the normally deferred aerospace and cryptography pages.
///
/// # Errors
///
/// Returns an error if the Summary or rendered pages cannot be read.
pub fn check_preview_links(
    html_root: &Path,
    summary_path: &Path,
    include_deferred: bool,
) -> Result<(Vec<String>, usize)> {
    let html_root = canonical_html_root(html_root)?;
    let summary = fs::read_to_string(summary_path)
        .with_context(|| format!("could not read {}", summary_path.display()))?;
    let summary_links =
        Regex::new(r"(?m)^\s*-\s+\[.*?\]\(([^)]+\.md)\)").expect("the Summary link regex is valid");
    let sources = summary_links
        .captures_iter(&summary)
        .filter_map(|capture| capture.get(1))
        .map(|target| {
            let decoded = decode_component(target.as_str());
            let mut path = PathBuf::from(decoded);
            path.set_extension("html");
            slash_path(&path)
        })
        .collect::<BTreeSet<_>>();
    if sources.is_empty() {
        bail!("the staged Summary lists no chapters");
    }
    let mut required = vec!["index.html".to_owned()];
    required.extend(sources.iter().cloned());
    let mut missing = Vec::new();
    for source in &required {
        let path = html_root.join(source);
        let valid_output = path
            .metadata()
            .is_ok_and(|metadata| metadata.is_file() && metadata.len() > 0);
        if !valid_output {
            missing.push(source.clone());
        }
    }
    if !missing.is_empty() {
        bail!("preview output missing or empty: {}", missing.join(", "));
    }

    let sources = sources
        .into_iter()
        .filter(|source| {
            include_deferred
                || !source.starts_with("categories/aerospace")
                    && !source.starts_with("categories/cryptography_cryptocurrencies/")
        })
        .collect::<Vec<_>>();
    let checked = sources.len();
    let failures = check_chapters(&html_root, &sources, true)?;
    Ok((failures, checked))
}

/// Returns imports that name known Cargo dependencies unavailable in the Playground.
fn unavailable_playground_imports(
    code: &str,
    dependencies: &BTreeSet<String>,
    available: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut imports = USE_IMPORT
        .captures_iter(code)
        .chain(EXTERN_IMPORT.captures_iter(code))
        .filter_map(|capture| capture.get(1))
        .map(|name| name.as_str().to_owned())
        .collect::<BTreeSet<_>>();
    let modules = LOCAL_MODULE
        .captures_iter(code)
        .filter_map(|capture| capture.get(1))
        .map(|name| name.as_str().to_owned())
        .collect::<BTreeSet<_>>();
    imports.retain(|name| {
        dependencies.contains(name) && !available.contains(name) && !modules.contains(name)
    });
    imports
}

/// Checks every requested page for required content, unresolved references, and local targets.
fn check_chapters(
    html_root: &Path,
    sources: &[String],
    check_fragments: bool,
) -> Result<Vec<String>> {
    let mut failures = Vec::new();
    let mut pages = BTreeMap::<PathBuf, Page>::new();
    for source in sources {
        let path = html_root.join(source);
        if !path.is_file() {
            failures.push(format!("{source}: missing required page"));
            continue;
        }
        if path.metadata()?.len() == 0 {
            failures.push(format!("{source}: empty required page"));
            continue;
        }
        let html = fs::read_to_string(&path)
            .with_context(|| format!("could not read {}", path.display()))?;
        let prose = CODE_AND_SCRIPT.replace_all(&html, "");
        let unresolved = UNRESOLVED_REFERENCE
            .captures_iter(&prose)
            .filter_map(|capture| capture.get(1))
            .map(|label| label.as_str().to_owned())
            .collect::<BTreeSet<_>>();
        for label in unresolved {
            failures.push(format!("{source}: unresolved reference {label}"));
        }
        if !pages.contains_key(&path) {
            pages.insert(path.clone(), parse_page(&html));
        }
        let mut targets = pages
            .get(&path)
            .map_or_else(Vec::new, |page| page.targets.clone());
        targets.sort();
        targets.dedup_by(|left, right| left.href == right.href);
        let source_url = Url::parse(BOOK_URL)?.join(source)?;
        for link in targets {
            let href = &link.href;
            let Ok(target) = source_url.join(href) else {
                continue;
            };
            if !is_local_book(&target) {
                continue;
            }
            let Some(candidate) = local_book_path(html_root, &target) else {
                failures.push(format!("{source}:{}: {href} leaves the book", link.line));
                continue;
            };
            let candidate = if candidate.is_dir() {
                candidate.join("index.html")
            } else {
                candidate
            };
            if !candidate.is_file() {
                failures.push(format!("{source}:{}: missing target {href}", link.line));
            } else if check_fragments
                && let Some(fragment) = target.fragment().filter(|fragment| !fragment.is_empty())
                && candidate
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("html"))
            {
                let candidate = candidate.canonicalize().with_context(|| {
                    format!("could not resolve rendered page {}", candidate.display())
                })?;
                if !candidate.starts_with(html_root) {
                    failures.push(format!("{source}:{}: {href} leaves the book", link.line));
                    continue;
                }
                if !pages.contains_key(&candidate) {
                    pages.insert(candidate.clone(), read_page(&candidate)?);
                }
                let fragment = decode_component(fragment);
                if !pages
                    .get(&candidate)
                    .is_some_and(|page| page.anchors.contains(&fragment))
                {
                    failures.push(format!("{source}:{}: missing anchor {href}", link.line));
                }
            }
        }
    }
    Ok(failures)
}

/// Discovers the rendered chapter paths linked from `toc.html`.
fn rendered_chapter_paths(html_root: &Path) -> std::result::Result<BTreeSet<String>, String> {
    let toc_path = html_root.join("toc.html");
    if !toc_path.is_file() {
        return Err("toc.html: missing rendered chapter inventory".to_owned());
    }
    let toc = fs::read_to_string(&toc_path)
        .map_err(|error| format!("toc.html: could not read rendered chapter inventory: {error}"))?;
    let page = parse_page(&toc);
    let base = Url::parse(BOOK_URL)
        .expect("the configured book URL is valid")
        .join("toc.html")
        .expect("the toc URL is valid");
    let chapters = page
        .targets
        .iter()
        .filter(|link| !is_absolute_or_authority(&link.href))
        .filter_map(|link| base.join(&link.href).ok())
        .filter(|target| {
            Path::new(target.path())
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("html"))
        })
        .filter_map(|target| local_book_path(html_root, &target))
        .filter_map(|path| path.strip_prefix(html_root).ok().map(slash_path))
        .collect::<BTreeSet<_>>();
    if chapters.is_empty() {
        return Err("toc.html: empty rendered chapter inventory".to_owned());
    }
    Ok(chapters)
}

/// Returns the sorted HTML files below the rendered book root.
fn html_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry.with_context(|| format!("could not walk {}", root.display()))?;
        if !entry.file_type().is_dir()
            && !entry.file_type().is_symlink()
            && entry
                .path()
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("html"))
        {
            paths.push(entry.path().to_path_buf());
        }
    }
    paths.sort();
    Ok(paths)
}

/// Verifies that the rendered book root exists and is a directory.
fn canonical_html_root(root: &Path) -> Result<PathBuf> {
    if !root.is_dir() {
        bail!("HTML directory does not exist: {}", root.display());
    }
    root.canonicalize()
        .with_context(|| format!("could not resolve HTML directory {}", root.display()))
}

/// Reports whether a target is absolute or has an authority component.
fn is_absolute_or_authority(href: &str) -> bool {
    href.starts_with("//") || Url::parse(href).is_ok()
}

/// Reports whether a resolved target points to the configured publication host.
fn is_local_book(target: &Url) -> bool {
    target.scheme() == "https"
        && target.host_str() == Some("book.invalid")
        && target.port().is_none()
        && target.username().is_empty()
        && target.password().is_none()
}

/// Resolves a URL beneath the book path to a safe local file path.
fn local_book_path(root: &Path, target: &Url) -> Option<PathBuf> {
    let relative = target.path().strip_prefix("/rust_howto/")?;
    let decoded = decode_component(relative);
    let mut path = root.to_path_buf();
    for component in Path::new(&decoded).components() {
        match component {
            Component::Normal(part) => path.push(part),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }
    path.starts_with(root).then_some(path)
}

/// Decodes one URL path or fragment component using replacement for invalid UTF-8.
fn decode_component(value: &str) -> String {
    percent_decode_str(value).decode_utf8_lossy().into_owned()
}

/// Converts platform paths to the stable slash-separated report representation.
fn slash_path(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Collects IDs, decoded links, redirects, and Playground source from HTML.
    #[test]
    fn html_tokenizer_decodes_attributes_and_collects_page_metadata() {
        let page = parse_page(
            "<a id='a&amp;b' href='chapter%20one.html#x&amp;y'></a><a name='legacy'></a><meta http-equiv='refresh' content=\"0; URL='next.html'\"><pre class=playground><code class='language-rust'>&lt;use&gt; serde::x;</code></pre>",
        );
        assert!(page.anchors.contains("a&b"));
        assert!(page.anchors.contains("legacy"));
        assert_eq!(
            page.targets
                .iter()
                .map(|target| target.href.as_str())
                .collect::<Vec<_>>(),
            ["chapter%20one.html#x&y", "next.html"]
        );
        assert_eq!(page.targets[0].line, 1);
        assert_eq!(page.playground.len(), 1);
        assert_eq!(page.playground[0].code, "<use> serde::x;");
        assert_eq!(page.playground[0].line, 1);
    }

    /// Missing required pages and an empty rendered chapter remain failures in inventory mode.
    #[test]
    fn all_chapters_keeps_required_pages_and_reports_empty_inventory_pages() -> Result<()> {
        let directory = tempfile::tempdir()?;
        for source in REQUIRED_PAGES {
            let path = directory.path().join(source);
            fs::create_dir_all(path.parent().context("required page has a parent")?)?;
            fs::write(path, "<html></html>\n")?;
        }
        fs::write(
            directory.path().join("toc.html"),
            "<a href='extra.html'>extra</a>",
        )?;
        fs::write(directory.path().join("extra.html"), "")?;
        let failures = check_rendered_links(directory.path(), true)?;
        assert_eq!(failures, ["extra.html: empty required page"]);
        Ok(())
    }

    /// Resolves percent-encoded targets and checks decoded fragment IDs.
    #[test]
    fn all_chapters_checks_encoded_local_fragments() -> Result<()> {
        let directory = tempfile::tempdir()?;
        fs::write(
            directory.path().join("toc.html"),
            "<a href='chapter%20one.html'>chapter</a>",
        )?;
        fs::write(
            directory.path().join("chapter one.html"),
            "<div id='a&amp;b'></div>",
        )?;
        fs::write(
            directory.path().join("index.html"),
            "<a href='chapter%20one.html#a%26b'>good</a><a href='chapter%20one.html#missing'>bad</a>",
        )?;
        let base = Url::parse(BOOK_URL)?.join("index.html")?;
        let target = base.join("chapter%20one.html#a%26b")?;
        assert!(is_local_book(&target));
        assert_eq!(
            local_book_path(directory.path(), &target),
            Some(directory.path().join("chapter one.html"))
        );
        let html_root = directory.path().canonicalize()?;
        let failures = check_chapters(&html_root, &["index.html".to_owned()], true)?;
        assert_eq!(
            failures,
            ["index.html:1: missing anchor chapter%20one.html#missing"]
        );
        Ok(())
    }

    /// Requires a rendered word-index page before checking its links.
    #[test]
    fn word_index_requires_its_rendered_page() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let failures = check_word_index(directory.path())?;
        assert_eq!(failures, ["missing required page: word_index.html"]);
        Ok(())
    }

    /// Finds unavailable external crates but preserves local modules and pinned imports.
    #[test]
    fn playground_imports_respect_available_crates_and_local_modules() {
        let dependencies = ["serde".to_owned(), "postage".to_owned()]
            .into_iter()
            .collect();
        let available = std::iter::once("serde".to_owned()).collect();
        assert_eq!(
            unavailable_playground_imports(
                "use serde::Deserialize;\nmod postage;\nuse postage::Message;",
                &dependencies,
                &available,
            ),
            BTreeSet::new()
        );
        assert_eq!(
            unavailable_playground_imports(
                "extern crate postage as channels;",
                &dependencies,
                &available
            ),
            std::iter::once("postage".to_owned()).collect()
        );
    }

    /// Honors the default deferred preview categories and their opt-in override.
    #[test]
    fn preview_filter_excludes_deferred_pages_by_default() -> Result<()> {
        let directory = tempfile::tempdir()?;
        fs::create_dir_all(directory.path().join("categories/aerospace"))?;
        fs::write(directory.path().join("index.html"), "<html></html>")?;
        fs::write(
            directory.path().join("categories/aerospace/index.html"),
            "<a href='missing.html'>missing</a>",
        )?;
        let summary = directory.path().join("SUMMARY.md");
        fs::write(&summary, "- [Deferred](categories/aerospace/index.md)\n")?;
        let (failures, checked) = check_preview_links(directory.path(), &summary, false)?;
        assert!(failures.is_empty());
        assert_eq!(checked, 0);
        let (failures, checked) = check_preview_links(directory.path(), &summary, true)?;
        assert_eq!(checked, 1);
        assert_eq!(
            failures,
            ["categories/aerospace/index.html:1: missing target missing.html"]
        );
        Ok(())
    }
}
