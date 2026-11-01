//! Repository-wide chapter, include, and Rust target integrity checks.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::sync::LazyLock;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use quote::ToTokens as _;
use regex::Regex;
use serde::Deserialize;
use syn::Attribute;
use syn::Item;
use syn::ItemMod;
use walkdir::WalkDir;

use crate::model::CargoTargetRecord;
use crate::model::ChapterRecord;
use crate::model::Diagnostic;
use crate::model::ExampleSourceRecord;
use crate::model::ExceptionRecord;
use crate::model::FragmentRecord;
use crate::model::IncludeRecord;
use crate::model::ModuleRecord;
use crate::model::Report;
use crate::model::SnippetRecord;

const BOOK_ROOTS: &[(&str, &str)] = &[
    ("bk/src", "published"),
    ("bk/drafts", "draft"),
    ("later/src", "future"),
];
const EXAMPLE_ROOTS: &[&str] = &["bk/crates", "later/crates"];
const ENTRY_POINTS: &[&str] = &["main.rs", "lib.rs", "mod.rs", "build.rs"];

static INCLUDE_DIRECTIVE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{#(?P<kind>include|rustdoc_include)\s+(?P<target>[^}\s]+)\s*\}\}")
        .expect("the include directive regex is valid")
});
static CUSTOM_EXAMPLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{#example\s+(?P<name>[^}\s]+)\s*\}\}")
        .expect("the custom example regex is valid")
});
static SUMMARY_LINK: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\]\((?P<path>[^)\s#]+\.md)(?:#[^)]*)?\)").expect("the Summary link regex is valid")
});
static RUST_FENCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\s*(`{3,}|~{3,})\s*(?P<language>[^\s`]*)")
        .expect("the Markdown fence regex is valid")
});

#[derive(Debug, Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoPackage>,
}

#[derive(Debug, Deserialize)]
struct CargoPackage {
    name: String,
    manifest_path: PathBuf,
    targets: Vec<CargoTarget>,
}

#[derive(Debug, Deserialize)]
struct CargoTarget {
    name: String,
    kind: Vec<String>,
    src_path: PathBuf,
    #[serde(default)]
    test: bool,
    #[serde(default, rename = "required-features")]
    required_features: Vec<String>,
}

#[derive(Clone, Debug)]
struct TargetIndex {
    record: CargoTargetRecord,
    tested: bool,
}

#[derive(Clone, Debug)]
struct ModuleVisitContext {
    source_path: String,
    target_identity: String,
    module_path: String,
    module_dir: PathBuf,
    conditions: Vec<String>,
}

#[derive(Debug)]
struct ScanState {
    repo_root: PathBuf,
    report: Report,
    targets_by_source: HashMap<String, Vec<TargetIndex>>,
    modules_by_source: HashMap<String, Vec<(String, Vec<String>)>>,
    directly_included: BTreeSet<String>,
    fragment_includers: BTreeMap<String, BTreeSet<String>>,
    chapter_has_compiled_example: HashSet<String>,
    chapter_has_tested_example: HashSet<String>,
    exceptions: Vec<ExceptionRecord>,
    example_paths: Vec<String>,
}

#[derive(Debug)]
struct IncludeOptions<'a> {
    chapter_path: &'a str,
    source_path: &'a str,
    line: usize,
    visible: bool,
    inside_fence: bool,
}

/// Builds a deterministic inventory and emits path-specific integrity diagnostics.
///
/// # Errors
///
/// Returns an error when repository files cannot be read or Cargo metadata cannot be loaded.
pub fn build_report(repo_root: &Path) -> Result<Report> {
    let repo_root = repo_root
        .canonicalize()
        .with_context(|| format!("could not resolve repository root {}", repo_root.display()))?;
    let exceptions = load_exceptions(&repo_root)?;
    let mut state = ScanState {
        repo_root,
        report: Report::default(),
        targets_by_source: HashMap::new(),
        modules_by_source: HashMap::new(),
        directly_included: BTreeSet::new(),
        fragment_includers: BTreeMap::new(),
        chapter_has_compiled_example: HashSet::new(),
        chapter_has_tested_example: HashSet::new(),
        exceptions,
        example_paths: Vec::new(),
    };
    for exception in state.exceptions.clone() {
        if exception.reason == "No reason supplied" {
            state.diagnostic(
                "error",
                "exception-no-reason",
                &exception.path,
                None,
                "exception entries must have a human-readable reason",
            );
        }
    }

    state.load_cargo_targets()?;
    state.scan_book_pages()?;
    state.finish_inventory()?;
    state.report.diagnostics.sort_by(|left, right| {
        (&left.path, left.line, &left.code, &left.message).cmp(&(
            &right.path,
            right.line,
            &right.code,
            &right.message,
        ))
    });
    state
        .report
        .chapters
        .sort_by(|left, right| left.path.cmp(&right.path));
    state
        .report
        .fragments
        .sort_by(|left, right| left.path.cmp(&right.path));
    state.report.includes.sort_by(|left, right| {
        (&left.source_path, left.line, &left.kind, &left.target_path).cmp(&(
            &right.source_path,
            right.line,
            &right.kind,
            &right.target_path,
        ))
    });
    state
        .report
        .cargo_targets
        .sort_by(|left, right| left.identity.cmp(&right.identity));
    state.report.modules.sort_by(|left, right| {
        (&left.target_identity, &left.module_path, &left.source_path).cmp(&(
            &right.target_identity,
            &right.module_path,
            &right.source_path,
        ))
    });
    state
        .report
        .standalone_snippets
        .sort_by(|left, right| (&left.path, left.line).cmp(&(&right.path, right.line)));
    state
        .report
        .example_sources
        .sort_by(|left, right| left.path.cmp(&right.path));
    state
        .report
        .exceptions
        .sort_by(|left, right| left.path.cmp(&right.path));
    Ok(state.report)
}

impl ScanState {
    /// Loads testable Cargo examples and their reachable Rust module graphs.
    fn load_cargo_targets(&mut self) -> Result<()> {
        for workspace in find_workspace_manifests(&self.repo_root)? {
            let output = Command::new("cargo")
                .args([
                    "metadata",
                    "--no-deps",
                    "--format-version",
                    "1",
                    "--manifest-path",
                ])
                .arg(&workspace)
                .current_dir(&self.repo_root)
                .output()
                .with_context(|| {
                    format!("could not run cargo metadata for {}", workspace.display())
                })?;
            if !output.status.success() {
                bail!(
                    "cargo metadata failed for {}: {}",
                    workspace.display(),
                    String::from_utf8_lossy(&output.stderr).trim()
                );
            }
            let metadata: CargoMetadata =
                serde_json::from_slice(&output.stdout).with_context(|| {
                    format!("could not parse cargo metadata for {}", workspace.display())
                })?;
            for package in metadata.packages {
                let manifest_path = self.relative_path(&package.manifest_path)?;
                for target in package.targets {
                    let source_path = self.relative_path(&target.src_path)?;
                    let identity =
                        format!("{manifest_path}#{}:{}", target.kind.join("+"), target.name);
                    let record = CargoTargetRecord {
                        identity: identity.clone(),
                        package: package.name.clone(),
                        name: target.name,
                        kinds: target.kind,
                        source_path: source_path.clone(),
                        required_features: target.required_features,
                        test: target.test,
                    };
                    self.report.cargo_targets.push(record.clone());
                    let indexed_target = TargetIndex {
                        tested: record.test,
                        record,
                    };
                    self.targets_by_source
                        .entry(source_path.clone())
                        .or_default()
                        .push(indexed_target.clone());
                    let target_conditions = indexed_target
                        .record
                        .required_features
                        .iter()
                        .map(|feature| format!("required-feature = {feature}"))
                        .collect::<Vec<_>>();
                    let crate_root_dir = self
                        .repo_root
                        .join(&source_path)
                        .parent()
                        .unwrap_or(&self.repo_root)
                        .to_path_buf();
                    self.visit_module_file(
                        ModuleVisitContext {
                            source_path,
                            target_identity: identity,
                            module_path: "crate".to_owned(),
                            module_dir: crate_root_dir,
                            conditions: target_conditions,
                        },
                        &mut HashSet::new(),
                    )?;
                }
            }
        }
        for targets in self.targets_by_source.values_mut() {
            targets.sort_by(|left, right| left.record.identity.cmp(&right.record.identity));
        }
        for crate_root in EXAMPLE_ROOTS {
            let root = self.repo_root.join(crate_root);
            if !root.is_dir() {
                continue;
            }
            for entry in WalkDir::new(&root).follow_links(false) {
                let entry = entry.with_context(|| format!("could not walk {}", root.display()))?;
                if !entry.file_type().is_dir()
                    && !entry.file_type().is_symlink()
                    && entry
                        .path()
                        .extension()
                        .is_some_and(|extension| extension == "rs")
                    && entry
                        .path()
                        .components()
                        .any(|part| part.as_os_str() == "examples")
                {
                    let source_path = self.relative_path(entry.path())?;
                    self.example_paths.push(source_path);
                }
            }
        }
        self.example_paths.sort();
        self.example_paths.dedup();
        Ok(())
    }

    /// Visits the modules reachable from a Cargo target, retaining their conditions.
    fn visit_module_file(
        &mut self,
        context: ModuleVisitContext,
        visited: &mut HashSet<(String, String)>,
    ) -> Result<()> {
        if !visited.insert((context.source_path.clone(), context.module_path.clone())) {
            return Ok(());
        }
        let full_path = self.repo_root.join(&context.source_path);
        let source = match fs::read_to_string(&full_path) {
            Ok(source) => source,
            Err(error) => {
                self.diagnostic(
                    "error",
                    "rust-module-unreadable",
                    &context.source_path,
                    None,
                    format!(
                        "could not read module for `{}`: {error}",
                        context.target_identity
                    ),
                );
                return Ok(());
            }
        };
        let parsed = match syn::parse_file(&source) {
            Ok(parsed) => parsed,
            Err(error) => {
                self.diagnostic(
                    "error",
                    "rust-module-parse-error",
                    &context.source_path,
                    None,
                    format!(
                        "could not parse module for `{}`: {error}",
                        context.target_identity
                    ),
                );
                return Ok(());
            }
        };
        self.report.modules.push(ModuleRecord {
            target_identity: context.target_identity.clone(),
            module_path: context.module_path.clone(),
            source_path: context.source_path.clone(),
            conditions: context.conditions.clone(),
        });
        self.modules_by_source
            .entry(context.source_path.clone())
            .or_default()
            .push((context.target_identity.clone(), context.conditions.clone()));
        self.visit_module_items(&parsed.items, context, visited)
    }

    /// Traverses inline and file-backed Rust modules from one parsed source file.
    fn visit_module_items(
        &mut self,
        items: &[Item],
        context: ModuleVisitContext,
        visited: &mut HashSet<(String, String)>,
    ) -> Result<()> {
        for item in items {
            let Item::Mod(module) = item else {
                continue;
            };
            let mut module_conditions = context.conditions.clone();
            module_conditions.extend(attribute_conditions(&module.attrs));
            module_conditions.sort();
            module_conditions.dedup();
            let child_module = format!("{}::{}", context.module_path, module.ident);
            if let Some((_, nested_items)) = &module.content {
                let nested_dir = context.module_dir.join(module.ident.to_string());
                self.visit_module_items(
                    nested_items,
                    ModuleVisitContext {
                        source_path: context.source_path.clone(),
                        target_identity: context.target_identity.clone(),
                        module_path: child_module,
                        module_dir: nested_dir,
                        conditions: module_conditions,
                    },
                    visited,
                )?;
                continue;
            }
            let Some(path) = Self::resolve_module_path(module, &context.module_dir) else {
                self.diagnostic(
                    "error",
                    "rust-module-missing",
                    &context.source_path,
                    None,
                    format!(
                        "module `{child_module}` declared by `{}` has no source file",
                        context.target_identity
                    ),
                );
                continue;
            };
            let child_source = match self.relative_path(&path) {
                Ok(path) => path,
                Err(error) => {
                    self.diagnostic(
                        "error",
                        "rust-module-outside-repository",
                        &context.source_path,
                        None,
                        format!("module `{child_module}` escapes the repository: {error}"),
                    );
                    continue;
                }
            };
            self.visit_module_file(
                ModuleVisitContext {
                    source_path: child_source,
                    target_identity: context.target_identity.clone(),
                    module_path: child_module,
                    module_dir: module_directory(&path),
                    conditions: module_conditions,
                },
                visited,
            )?;
        }
        Ok(())
    }

    /// Resolves a file-backed Rust module using `#[path]` or Rust's module layout.
    fn resolve_module_path(module: &ItemMod, module_dir: &Path) -> Option<PathBuf> {
        if let Some(path) = path_attribute(&module.attrs) {
            let candidate = module_dir.join(path);
            return candidate.is_file().then_some(candidate);
        }
        let name = module.ident.to_string();
        let flat = module_dir.join(format!("{name}.rs"));
        if flat.is_file() {
            return Some(flat);
        }
        let nested = module_dir.join(&name).join("mod.rs");
        nested.is_file().then_some(nested)
    }

    /// Scans book pages, recursively following Markdown and Rust include directives.
    fn scan_book_pages(&mut self) -> Result<()> {
        let mut pages = Vec::new();
        let mut listed_pages = HashSet::new();
        for (book_root, edition) in BOOK_ROOTS {
            let summary_relative_path = format!("{book_root}/SUMMARY.md");
            let summary_path = self.repo_root.join(&summary_relative_path);
            if summary_path.is_file() {
                let summary = fs::read_to_string(&summary_path)
                    .with_context(|| format!("could not read {}", summary_path.display()))?;
                for (line_index, line) in summary.lines().enumerate() {
                    for capture in SUMMARY_LINK.captures_iter(line) {
                        let link = capture
                            .name("path")
                            .expect("the path capture exists")
                            .as_str();
                        if link.contains("://") || link.starts_with("//") {
                            continue;
                        }
                        let line_number = line_index + 1;
                        match self.resolve_relative(&summary_relative_path, link) {
                            Some(path) if self.repo_root.join(&path).is_file() => {
                                listed_pages.insert(path);
                            }
                            Some(_) => self.diagnostic(
                                "error",
                                "summary-target-missing",
                                &summary_relative_path,
                                Some(line_number),
                                format!("Summary link `{link}` does not point to a file"),
                            ),
                            None => self.diagnostic(
                                "error",
                                "summary-link-outside-repository",
                                &summary_relative_path,
                                Some(line_number),
                                format!("Summary link `{link}` resolves outside the repository"),
                            ),
                        }
                    }
                }
            } else {
                self.diagnostic(
                    "error",
                    "summary-missing",
                    &summary_relative_path,
                    None,
                    "book root has no SUMMARY.md",
                );
            }
            for path in markdown_files(&self.repo_root.join(book_root))? {
                let relative = self.relative_path(&path)?;
                if is_chapter_candidate(&relative)
                    || (listed_pages.contains(&relative) && is_hidden_chapter(&relative))
                {
                    pages.push((relative, (*edition).to_owned()));
                }
            }
        }
        pages.sort_by(|left, right| left.0.cmp(&right.0));
        for (page_path, edition) in pages {
            let is_listed = listed_pages.contains(&page_path);
            let visible = self.scan_markdown(&page_path, &page_path, &mut HashSet::new())?;
            let has_example = self.chapter_has_compiled_example.contains(&page_path);
            let has_tested_example = self.chapter_has_tested_example.contains(&page_path);
            let disposition = self
                .exception_for(&page_path)
                .map(|exception| exception.reason.clone());
            let hidden = !visible || is_hidden_chapter(&page_path);
            self.report.chapters.push(ChapterRecord {
                path: page_path.clone(),
                edition,
                listed: is_listed,
                hidden,
                has_example,
                has_tested_example,
                standalone_rust_fences: self
                    .report
                    .standalone_snippets
                    .iter()
                    .filter(|snippet| snippet.chapter_path == page_path)
                    .count(),
                disposition: disposition.clone(),
            });
            if !is_listed && disposition.is_none() {
                self.diagnostic(
                    "error",
                    "chapter-unlisted",
                    &page_path,
                    None,
                    "chapter file is missing from its book SUMMARY.md",
                );
            }
            if is_listed && !has_example && disposition.is_none() {
                let has_inline_rust = self
                    .report
                    .standalone_snippets
                    .iter()
                    .any(|snippet| snippet.chapter_path == page_path);
                self.diagnostic(
                    if has_inline_rust { "warning" } else { "error" },
                    if has_inline_rust {
                        "chapter-inline-snippets-only"
                    } else {
                        "chapter-without-example"
                    },
                    &page_path,
                    None,
                    if has_inline_rust {
                        "listed chapter has inline Rust snippets but no Cargo-backed example include"
                    } else {
                        "listed chapter has no visible Rust source include backed by a Cargo target"
                    },
                );
            }
            if is_listed && has_example && !has_tested_example && disposition.is_none() {
                self.diagnostic(
                    "warning",
                    "chapter-example-not-test-target",
                    &page_path,
                    None,
                    "chapter source is compiled by Cargo, but no included source belongs to a test target",
                );
            }
        }
        Ok(())
    }

    /// Scans one Markdown source and the visible content of its nested includes.
    fn scan_markdown(
        &mut self,
        source_path: &str,
        chapter_path: &str,
        active_fragments: &mut HashSet<String>,
    ) -> Result<bool> {
        let full_path = self.repo_root.join(source_path);
        let text = fs::read_to_string(&full_path)
            .with_context(|| format!("could not read Markdown file {}", full_path.display()))?;
        let lines = visible_lines(&text);
        let mut has_visible_content = false;
        let mut fence: Option<(char, usize, usize, String, bool, bool)> = None;
        for (line_number, line, visible) in lines {
            if visible && !line.trim().is_empty() {
                has_visible_content = true;
            }
            let is_fence_line = RUST_FENCE.captures(&line);
            let inside_fence = fence.is_some();
            if let Some(capture) = is_fence_line {
                let marker = capture.get(1).expect("fence capture exists").as_str();
                let marker_char = marker.chars().next().unwrap_or('`');
                if let Some((
                    open_char,
                    open_len,
                    open_line,
                    language,
                    rust_fence,
                    has_source_include,
                )) = fence.take()
                {
                    if open_char == marker_char && marker.len() >= open_len {
                        if rust_fence && !has_source_include {
                            self.report.standalone_snippets.push(SnippetRecord {
                                path: source_path.to_owned(),
                                line: open_line,
                                language,
                                chapter_path: chapter_path.to_owned(),
                                visible,
                            });
                        }
                    } else {
                        fence = Some((
                            open_char,
                            open_len,
                            open_line,
                            language,
                            rust_fence,
                            has_source_include,
                        ));
                    }
                } else {
                    let language = capture
                        .name("language")
                        .map_or_else(String::new, |language| {
                            language.as_str().to_ascii_lowercase()
                        });
                    let rust_fence = matches!(
                        language.split(',').next().unwrap_or_default(),
                        "rust" | "rs"
                    );
                    fence = Some((
                        marker_char,
                        marker.len(),
                        line_number,
                        language,
                        rust_fence,
                        false,
                    ));
                }
            }
            let mut directives = Vec::new();
            for capture in INCLUDE_DIRECTIVE.captures_iter(&line) {
                let kind = capture.name("kind").expect("kind capture exists").as_str();
                let target = capture
                    .name("target")
                    .expect("target capture exists")
                    .as_str();
                directives.push((kind.to_owned(), target.to_owned()));
            }
            for capture in CUSTOM_EXAMPLE.captures_iter(&line) {
                let name = capture.name("name").expect("name capture exists").as_str();
                directives.push(("example".to_owned(), name.to_owned()));
            }
            for (kind, target) in directives {
                if inside_fence
                    && matches!(kind.as_str(), "include" | "rustdoc_include")
                    && Path::new(split_selection(&target).0)
                        .extension()
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("rs"))
                    && let Some((_, _, _, _, _, has_source_include)) = &mut fence
                {
                    *has_source_include = true;
                }
                self.scan_include(
                    &target,
                    &kind,
                    IncludeOptions {
                        chapter_path,
                        source_path,
                        line: line_number,
                        visible,
                        inside_fence,
                    },
                    active_fragments,
                )?;
            }
        }
        if let Some((_, _, open_line, language, rust_fence, has_source_include)) = fence
            && rust_fence
            && !has_source_include
        {
            self.report.standalone_snippets.push(SnippetRecord {
                path: source_path.to_owned(),
                line: open_line,
                language,
                chapter_path: chapter_path.to_owned(),
                visible: true,
            });
        }
        Ok(has_visible_content)
    }

    /// Records one include, checks its target, and follows Markdown fragments.
    fn scan_include(
        &mut self,
        target: &str,
        kind: &str,
        options: IncludeOptions<'_>,
        active_fragments: &mut HashSet<String>,
    ) -> Result<()> {
        if kind == "example" {
            let matching_targets = self
                .report
                .cargo_targets
                .iter()
                .filter(|cargo_target| {
                    cargo_target.name == target
                        || cargo_target.source_path.ends_with(&format!("/{target}.rs"))
                        || cargo_target
                            .source_path
                            .ends_with(&format!("/{target}/main.rs"))
                })
                .map(|cargo_target| cargo_target.identity.clone())
                .collect::<Vec<_>>();
            let tested = matching_targets.iter().any(|identity| {
                self.report
                    .cargo_targets
                    .iter()
                    .any(|cargo_target| cargo_target.identity == *identity && cargo_target.test)
            });
            if options.visible && tested {
                self.chapter_has_compiled_example
                    .insert(options.chapter_path.to_owned());
                self.chapter_has_tested_example
                    .insert(options.chapter_path.to_owned());
            } else if options.visible && !matching_targets.is_empty() {
                self.chapter_has_compiled_example
                    .insert(options.chapter_path.to_owned());
            }
            if options.visible && matching_targets.is_empty() {
                self.diagnostic(
                    "error",
                    "custom-example-unresolved",
                    options.source_path,
                    Some(options.line),
                    format!("custom example `{target}` does not match a Cargo target"),
                );
            }
            self.report.includes.push(IncludeRecord {
                source_path: options.source_path.to_owned(),
                chapter_path: options.chapter_path.to_owned(),
                line: options.line,
                kind: kind.to_owned(),
                target_path: None,
                selection: None,
                custom_name: Some(target.to_owned()),
                visible: options.visible,
                resolved: !matching_targets.is_empty(),
                cargo_targets: matching_targets,
                conditions: Vec::new(),
            });
            return Ok(());
        }

        let (file_target, selection) = split_selection(target);
        let Some(resolved_path) = self.resolve_relative(options.source_path, file_target) else {
            self.report.includes.push(IncludeRecord {
                source_path: options.source_path.to_owned(),
                chapter_path: options.chapter_path.to_owned(),
                line: options.line,
                kind: kind.to_owned(),
                target_path: None,
                selection: selection.map(str::to_owned),
                custom_name: None,
                visible: options.visible,
                resolved: false,
                cargo_targets: Vec::new(),
                conditions: Vec::new(),
            });
            if options.visible {
                self.diagnostic(
                    "error",
                    "include-outside-repository",
                    options.source_path,
                    Some(options.line),
                    format!("include `{target}` resolves outside the repository"),
                );
            }
            return Ok(());
        };
        let exists = self.repo_root.join(&resolved_path).is_file();
        let cargo_targets = self
            .targets_by_source
            .get(&resolved_path)
            .into_iter()
            .flatten()
            .map(|target| target.record.identity.clone())
            .chain(
                self.modules_by_source
                    .get(&resolved_path)
                    .into_iter()
                    .flatten()
                    .map(|(identity, _)| identity.clone()),
            )
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let conditions = self
            .modules_by_source
            .get(&resolved_path)
            .into_iter()
            .flatten()
            .flat_map(|(_, conditions)| conditions.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        self.report.includes.push(IncludeRecord {
            source_path: options.source_path.to_owned(),
            chapter_path: options.chapter_path.to_owned(),
            line: options.line,
            kind: kind.to_owned(),
            target_path: Some(resolved_path.clone()),
            selection: selection.map(str::to_owned),
            custom_name: None,
            visible: options.visible,
            resolved: exists,
            cargo_targets: cargo_targets.clone(),
            conditions,
        });
        if !exists {
            if options.visible {
                self.diagnostic(
                    "error",
                    "include-missing",
                    options.source_path,
                    Some(options.line),
                    format!("include target `{target}` does not exist"),
                );
            }
            return Ok(());
        }
        if Path::new(file_target)
            .extension()
            .is_some_and(|extension| extension == "rs")
        {
            if selection.is_some_and(|selection| {
                !selection_exists(&self.repo_root.join(&resolved_path), selection)
            }) && options.visible
            {
                self.diagnostic(
                    "error",
                    "include-anchor-missing",
                    options.source_path,
                    Some(options.line),
                    format!("Rust include `{target}` selects an anchor that is not defined"),
                );
            }
            if options.visible {
                self.directly_included.insert(resolved_path.clone());
                if !options.inside_fence {
                    self.diagnostic(
                        "error",
                        "include-unfenced",
                        options.source_path,
                        Some(options.line),
                        format!("Rust include `{target}` must be inside a fenced code block"),
                    );
                }
                let has_tested_target = self
                    .targets_by_source
                    .get(&resolved_path)
                    .into_iter()
                    .flatten()
                    .any(|target| target.tested)
                    || self
                        .modules_by_source
                        .get(&resolved_path)
                        .into_iter()
                        .flatten()
                        .any(|(identity, _)| {
                            self.report
                                .cargo_targets
                                .iter()
                                .any(|target| target.identity == *identity && target.test)
                        });
                if has_tested_target {
                    self.chapter_has_compiled_example
                        .insert(options.chapter_path.to_owned());
                    self.chapter_has_tested_example
                        .insert(options.chapter_path.to_owned());
                } else {
                    let has_compiled_target = !cargo_targets.is_empty();
                    if has_compiled_target {
                        self.chapter_has_compiled_example
                            .insert(options.chapter_path.to_owned());
                    }
                    let target_is_example = cargo_targets.iter().any(|identity| {
                        self.report.cargo_targets.iter().any(|target| {
                            target.identity == *identity
                                && target.kinds.iter().any(|kind| kind == "example")
                        })
                    });
                    let source_is_example =
                        resolved_path.contains("/examples/") || target_is_example;
                    let has_exception = self.exception_for(&resolved_path).is_some();
                    self.diagnostic(
                        if has_compiled_target || !source_is_example || has_exception {
                            "warning"
                        } else {
                            "error"
                        },
                        "included-example-untested",
                        options.source_path,
                        Some(options.line),
                        format!("Rust include `{target}` is not compiled by a tested Cargo target"),
                    );
                }
            }
        } else if Path::new(file_target)
            .extension()
            .is_some_and(|extension| extension == "md")
        {
            self.fragment_includers
                .entry(resolved_path.clone())
                .or_default()
                .insert(options.source_path.to_owned());
            if options.visible {
                if active_fragments.insert(resolved_path.clone()) {
                    self.scan_markdown(&resolved_path, options.chapter_path, active_fragments)?;
                    active_fragments.remove(&resolved_path);
                } else {
                    self.diagnostic(
                        "error",
                        "include-cycle",
                        options.source_path,
                        Some(options.line),
                        format!("Markdown include cycle reaches `{target}`"),
                    );
                }
            }
        }
        Ok(())
    }

    /// Completes example source roles, fragment records, and unreferenced-source diagnostics.
    fn finish_inventory(&mut self) -> Result<()> {
        for (fragment, included_by) in &self.fragment_includers {
            self.report.fragments.push(FragmentRecord {
                path: fragment.clone(),
                included_by: included_by.iter().cloned().collect(),
            });
        }
        for source_path in self.example_paths.clone() {
            let exception = self.exception_for(&source_path).cloned();
            let targets = self
                .modules_by_source
                .get(&source_path)
                .into_iter()
                .flatten()
                .map(|(identity, _)| identity.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let conditions = self
                .modules_by_source
                .get(&source_path)
                .into_iter()
                .flatten()
                .flat_map(|(_, conditions)| conditions.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            let anchored = has_any_anchor(&self.repo_root.join(&source_path));
            let directly_included = self.directly_included.contains(&source_path);
            let module_reachable = !targets.is_empty();
            let role = if exception.is_some() {
                "reasoned-exclusion"
            } else if directly_included {
                "direct-include"
            } else if module_reachable {
                "helper-module"
            } else if is_generated_source(&source_path) {
                "generated-source"
            } else {
                "orphan"
            };
            let disposition = exception.as_ref().map(|entry| entry.reason.clone());
            self.report.example_sources.push(ExampleSourceRecord {
                path: source_path.clone(),
                anchored,
                role: role.to_owned(),
                cargo_targets: targets.clone(),
                conditions,
                disposition,
            });
            if role == "orphan"
                && !is_generated_source(&source_path)
                && !ENTRY_POINTS.contains(
                    &Path::new(&source_path)
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or_default(),
                )
            {
                self.diagnostic(
                    "error",
                    "example-orphan",
                    &source_path,
                    None,
                    "Rust source under an examples directory is not included by a chapter or reachable from a Cargo target",
                );
            }
            if directly_included && !anchored {
                self.diagnostic(
                    "warning",
                    "example-anchor-missing",
                    &source_path,
                    None,
                    "directly included Rust source has no `example` anchor",
                );
            }
        }
        Ok(())
    }

    /// Converts a file path to a canonical repository-relative slash-separated path.
    fn relative_path(&self, path: &Path) -> Result<String> {
        let canonical = path
            .canonicalize()
            .with_context(|| format!("could not resolve {}", path.display()))?;
        canonical
            .strip_prefix(&self.repo_root)
            .map(path_to_slashes)
            .with_context(|| {
                format!(
                    "{} is outside {}",
                    canonical.display(),
                    self.repo_root.display()
                )
            })
    }

    /// Resolves an include path relative to its containing Markdown file.
    fn resolve_relative(&self, source_path: &str, target: &str) -> Option<String> {
        let candidate = self
            .repo_root
            .join(source_path)
            .parent()?
            .join(target.replace('/', std::path::MAIN_SEPARATOR_STR));
        let normalized = normalize_path(&candidate)?;
        if !normalized.starts_with(&self.repo_root) {
            return None;
        }
        let resolved = if normalized.exists() {
            normalized.canonicalize().ok()?
        } else {
            normalized
        };
        let relative = resolved.strip_prefix(&self.repo_root).ok()?;
        Some(path_to_slashes(relative))
    }

    /// Returns the most specific exception matching a repository-relative path.
    fn exception_for(&self, path: &str) -> Option<&ExceptionRecord> {
        self.exceptions
            .iter()
            .filter(|exception| {
                path == exception.path
                    || (exception.path.ends_with('/') && path.starts_with(&exception.path))
            })
            .max_by_key(|exception| exception.path.len())
    }

    /// Adds one diagnostic record with stable, explicit severity and code.
    fn diagnostic(
        &mut self,
        severity: &str,
        code: &str,
        path: &str,
        line: Option<usize>,
        message: impl Into<String>,
    ) {
        self.report.diagnostics.push(Diagnostic {
            severity: severity.to_owned(),
            code: code.to_owned(),
            path: path.to_owned(),
            line,
            message: message.into(),
        });
    }
}

/// Discovers Cargo workspace roots and standalone package roots under crate trees.
fn find_workspace_manifests(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let mut manifests = BTreeSet::new();
    let mut discovered = Vec::new();
    for crate_root in EXAMPLE_ROOTS {
        let root = repo_root.join(crate_root);
        if !root.is_dir() {
            continue;
        }
        for entry in WalkDir::new(&root).follow_links(false) {
            let entry = entry.with_context(|| format!("could not walk {}", root.display()))?;
            if entry.file_type().is_dir()
                || entry.file_type().is_symlink()
                || entry.file_name() != "Cargo.toml"
            {
                continue;
            }
            let value: toml::Value = toml::from_str(
                &fs::read_to_string(entry.path())
                    .with_context(|| format!("could not read {}", entry.path().display()))?,
            )
            .with_context(|| format!("could not parse {}", entry.path().display()))?;
            discovered.push((entry.path().to_path_buf(), value));
        }
    }
    let workspace_roots = discovered
        .iter()
        .filter(|(_, value)| value.get("workspace").is_some())
        .map(|(manifest, _)| {
            manifest
                .parent()
                .expect("manifest has a parent")
                .to_path_buf()
        })
        .collect::<Vec<_>>();
    let workspace_excludes = discovered
        .iter()
        .filter_map(|(manifest, value)| {
            let excludes = value
                .get("workspace")?
                .get("exclude")?
                .as_array()?
                .iter()
                .filter_map(toml::Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>();
            Some((manifest.parent()?.to_path_buf(), excludes))
        })
        .collect::<Vec<_>>();
    for (manifest, value) in discovered {
        if value.get("workspace").is_some() {
            manifests.insert(manifest);
            continue;
        }
        let Some(package) = value.get("package") else {
            continue;
        };
        if package.get("workspace").is_some() {
            continue;
        }
        let package_dir = manifest.parent().expect("manifest has a parent");
        let is_in_workspace = workspace_roots
            .iter()
            .any(|root| package_dir.starts_with(root));
        let is_excluded = workspace_excludes.iter().any(|(root, excludes)| {
            package_dir.strip_prefix(root).is_ok_and(|relative| {
                let relative = path_to_slashes(relative);
                excludes.iter().any(|exclude| {
                    relative == *exclude || relative.starts_with(&format!("{exclude}/"))
                })
            })
        });
        if !is_in_workspace || is_excluded {
            manifests.insert(manifest);
        }
    }
    Ok(manifests.into_iter().collect())
}

/// Reads and parses the path-specific exception list, keeping its adjacent explanation.
fn load_exceptions(repo_root: &Path) -> Result<Vec<ExceptionRecord>> {
    let path = repo_root.join("scripts/example_chapter_exceptions.txt");
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let text =
        fs::read_to_string(&path).with_context(|| format!("could not read {}", path.display()))?;
    let mut records = Vec::new();
    let mut reason_lines = Vec::new();
    let mut last_was_path = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(comment) = trimmed.strip_prefix('#') {
            let comment = comment.trim();
            if comment.starts_with("---") {
                reason_lines.clear();
            } else if !comment.is_empty() {
                if last_was_path {
                    reason_lines.clear();
                }
                reason_lines.push(comment.to_owned());
            }
            last_was_path = false;
        } else if !trimmed.is_empty() {
            let reason = reason_lines.join(" ");
            records.push(ExceptionRecord {
                path: trimmed.to_owned(),
                reason: if reason.is_empty() {
                    "No reason supplied".to_owned()
                } else {
                    reason
                },
            });
            last_was_path = true;
        }
    }
    Ok(records)
}

/// Lists Markdown files below a book source root in deterministic order.
fn markdown_files(root: &Path) -> Result<Vec<PathBuf>> {
    if !root.is_dir() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry.with_context(|| format!("could not walk {}", root.display()))?;
        if !entry.file_type().is_dir()
            && !entry.file_type().is_symlink()
            && entry
                .path()
                .extension()
                .is_some_and(|extension| extension == "md")
        {
            files.push(entry.path().to_path_buf());
        }
    }
    files.sort();
    Ok(files)
}

/// Determines whether a Markdown file is a chapter rather than a Summary or include fragment.
fn is_chapter_candidate(path: &str) -> bool {
    let page = Path::new(path);
    let name = page
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    name != "SUMMARY.md"
        && name != "README.md"
        && name != "index.md"
        && !name.ends_with(".incl.md")
        && !name.starts_with('_')
        && !path.contains("/refs/")
        && !path.ends_with("/refs.md")
}

/// Identifies chapters hidden by the repository's underscore filename prefix.
fn is_hidden_chapter(path: &str) -> bool {
    Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('_'))
}

/// Converts absolute or relative paths into normalized slash-separated repository paths.
fn path_to_slashes(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

/// Collapses `.` and `..` segments without allowing a path to escape its drive root.
fn normalize_path(path: &Path) -> Option<PathBuf> {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() {
                    return None;
                }
            }
            Component::Normal(part) => normalized.push(part),
        }
    }
    Some(normalized)
}

/// Converts Markdown lines hidden by the book preprocessor into invisible blank lines.
fn visible_lines(text: &str) -> Vec<(usize, String, bool)> {
    let mut hidden_div = false;
    let mut hidden_comment = false;
    let mut output = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let mut visible = !hidden_div && !hidden_comment;
        let mut remaining = line;
        while !remaining.is_empty() {
            if hidden_div {
                if let Some((_, rest)) = remaining.split_once("</div>") {
                    remaining = rest;
                    hidden_div = false;
                } else {
                    break;
                }
            } else if hidden_comment {
                if let Some((_, rest)) = remaining.split_once("-->") {
                    remaining = rest;
                    hidden_comment = false;
                } else {
                    break;
                }
            } else {
                let div = remaining.find("<div class=\"hidden\">");
                let comment = remaining.find("<!--");
                match (div, comment) {
                    (Some(div_at), Some(comment_at)) if comment_at < div_at => {
                        visible = false;
                        hidden_comment = true;
                        remaining = remaining.split_once("<!--").map_or("", |(_, rest)| rest);
                    }
                    (Some(_), _) => {
                        visible = false;
                        hidden_div = true;
                        remaining = remaining
                            .split_once("<div class=\"hidden\">")
                            .map_or("", |(_, rest)| rest);
                    }
                    (None, Some(_)) => {
                        visible = false;
                        remaining = remaining.split_once("<!--").map_or("", |(_, rest)| rest);
                        hidden_comment = true;
                    }
                    (None, None) => break,
                }
            }
        }
        output.push((index + 1, line.to_owned(), visible));
    }
    output
}

/// Splits an mdBook include target into its file path and optional anchor or line selection.
fn split_selection(target: &str) -> (&str, Option<&str>) {
    target
        .char_indices()
        .filter(|(_, character)| *character == ':')
        .find_map(|(index, _)| {
            let path = target.get(..index)?;
            let selection = target.get(index + 1..)?;
            Path::new(path)
                .extension()
                .is_some()
                .then_some((path, Some(selection)))
        })
        .unwrap_or((target, None))
}

/// Checks that a Rust source declares a matching mdBook anchor block.
fn anchor_exists(path: &Path, anchor: &str) -> bool {
    let Ok(source) = fs::read_to_string(path) else {
        return false;
    };
    let start = format!("ANCHOR: {anchor}");
    let end = format!("ANCHOR_END: {anchor}");
    let mut found_start = false;
    for line in source.lines() {
        found_start |= line.contains(&start);
        if found_start && line.contains(&end) {
            return true;
        }
    }
    false
}

/// Checks whether a Rust file contains at least one complete named anchor block.
fn has_any_anchor(path: &Path) -> bool {
    let Ok(source) = fs::read_to_string(path) else {
        return false;
    };
    let anchors = source
        .lines()
        .filter_map(|line| line.split_once("ANCHOR: ").map(|(_, anchor)| anchor.trim()))
        .collect::<HashSet<_>>();
    anchors
        .into_iter()
        .any(|anchor| anchor_exists(path, anchor))
}

/// Identifies files generated from schemas or code-generation templates.
fn is_generated_source(path: &str) -> bool {
    let name = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    name == "generated.rs" || name.ends_with("_generated.rs") || name.ends_with("_capnp.rs")
}

/// Checks a named anchor or an inclusive one-based line selection.
fn selection_exists(path: &Path, selection: &str) -> bool {
    let Ok(source) = fs::read_to_string(path) else {
        return false;
    };
    let line_count = source.lines().count();
    let parts = selection.split(':').collect::<Vec<_>>();
    let numeric_selection = parts.len() <= 2
        && parts
            .iter()
            .all(|part| part.is_empty() || part.parse::<usize>().is_ok());
    if numeric_selection {
        let range = match parts.as_slice() {
            [line] if !line.is_empty() => line.parse::<usize>().ok().map(|line| (line, line)),
            [start, end] if !start.is_empty() || !end.is_empty() => {
                let start = if start.is_empty() {
                    Some(1)
                } else {
                    start.parse::<usize>().ok()
                };
                let end = if end.is_empty() {
                    Some(line_count)
                } else {
                    end.parse::<usize>().ok()
                };
                start.zip(end)
            }
            _ => None,
        };
        if let Some((start, end)) = range {
            return start > 0 && end >= start && end <= line_count;
        }
    }
    anchor_exists(path, selection)
}

/// Returns the default child-module directory for a Rust source file.
fn module_directory(source_path: &Path) -> PathBuf {
    let parent = source_path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = source_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    if ENTRY_POINTS.contains(&file_name) {
        parent.to_path_buf()
    } else {
        parent.join(source_path.file_stem().unwrap_or_default())
    }
}

/// Extracts `cfg` and `cfg_attr` expressions carried by a Rust module declaration.
fn attribute_conditions(attributes: &[Attribute]) -> Vec<String> {
    attributes
        .iter()
        .filter(|attribute| {
            attribute.path().is_ident("cfg") || attribute.path().is_ident("cfg_attr")
        })
        .map(|attribute| attribute.meta.to_token_stream().to_string())
        .collect()
}

/// Reads a literal `#[path = "..."]` module override when present.
fn path_attribute(attributes: &[Attribute]) -> Option<String> {
    attributes.iter().find_map(|attribute| {
        if !attribute.path().is_ident("path") {
            return None;
        }
        let syn::Meta::NameValue(name_value) = &attribute.meta else {
            return None;
        };
        let syn::Expr::Lit(expression) = &name_value.value else {
            return None;
        };
        let syn::Lit::Str(path) = &expression.lit else {
            return None;
        };
        Some(path.value())
    })
}

#[cfg(test)]
mod tests {
    //! Tests for path identity, hidden content, and module conditions.

    use super::*;

    /// Hidden HTML and comments suppress directives without hiding subsequent lines.
    #[test]
    fn hidden_lines_are_not_visible() {
        let lines = visible_lines(
            "shown\n<div class=\"hidden\">\n{{#include missing.rs}}\n</div>\n<!-- hidden\nsecret\n-->\nvisible again\n",
        );
        assert!(lines[0].2);
        assert!(!lines[2].2);
        assert!(!lines[5].2);
        assert!(lines[7].2);
    }

    /// Include path normalization collapses nested relative components.
    #[test]
    fn relative_paths_keep_directory_identity() {
        let path = normalize_path(Path::new("repo/book/../crates/one/examples/demo.rs"))
            .expect("normalized path stays inside the root");
        assert_eq!(path_to_slashes(&path), "repo/crates/one/examples/demo.rs");
        assert!(normalize_path(Path::new("../outside.rs")).is_none());
    }

    /// A Windows drive prefix is dropped from the slash-separated form.
    #[cfg(windows)]
    #[test]
    fn drive_prefixes_are_dropped_from_slash_paths() {
        let path = normalize_path(Path::new("D:/repo/book/../crates/one/examples/demo.rs"))
            .expect("normalized path stays on its drive");
        assert_eq!(path_to_slashes(&path), "repo/crates/one/examples/demo.rs");
    }

    /// File-backed modules inherit target conditions and retain distinct paths.
    #[test]
    fn module_conditions_preserve_cfg_attributes() {
        let source = "#[cfg(target_os = \"none\")] mod embedded;";
        let file = syn::parse_file(source).expect("the fixture module parses");
        let Item::Mod(module) = &file.items[0] else {
            panic!("the fixture starts with a module");
        };
        assert_eq!(
            attribute_conditions(&module.attrs),
            ["cfg (target_os = \"none\")"]
        );
    }

    /// Anchor selection parsing preserves the full selector after the source path.
    #[test]
    fn source_include_splits_anchor() {
        assert_eq!(
            split_selection("examples/demo/main.rs:example"),
            ("examples/demo/main.rs", Some("example"))
        );
    }

    /// Named and numeric source selections are validated without conflating them.
    #[test]
    fn source_selections_distinguish_anchors_from_line_ranges() {
        let directory = tempfile::tempdir().expect("temporary fixture directory is created");
        let source_path = directory.path().join("source.rs");
        fs::write(
            &source_path,
            "// ANCHOR: example\nfn main() {}\n// ANCHOR_END: example\n",
        )
        .expect("fixture source is written");
        assert!(selection_exists(&source_path, "example"));
        assert!(selection_exists(&source_path, "1:3"));
        assert!(selection_exists(&source_path, "2:"));
        assert!(selection_exists(&source_path, ":2"));
        assert!(!selection_exists(&source_path, "2:4"));
        assert!(!selection_exists(&source_path, "missing"));
        let reversed_path = directory.path().join("reversed.rs");
        fs::write(
            &reversed_path,
            "// ANCHOR_END: reversed\nfn main() {}\n// ANCHOR: reversed\n",
        )
        .expect("reversed anchor fixture is written");
        assert!(!selection_exists(&reversed_path, "reversed"));
        assert_eq!(
            split_selection("examples/demo/main.rs:2:10"),
            ("examples/demo/main.rs", Some("2:10"))
        );
        assert_eq!(
            split_selection("examples/demo/main.rs::10"),
            ("examples/demo/main.rs", Some(":10"))
        );
    }

    /// Exception reasons apply to grouped paths and reset for the next explanation.
    #[test]
    fn exception_reasons_are_stable_across_path_groups() {
        let directory = tempfile::tempdir().expect("temporary fixture directory is created");
        let root = directory.path();
        write_fixture(
            root,
            "scripts/example_chapter_exceptions.txt",
            "# First reason, continued\n# with a second line.\nfirst.md\nsecond.md\n# Another reason.\nthird.md\n",
        );
        let exceptions = load_exceptions(root).expect("exception file parses");
        assert_eq!(
            exceptions[0].reason,
            "First reason, continued with a second line."
        );
        assert_eq!(exceptions[1].reason, exceptions[0].reason);
        assert_eq!(exceptions[2].reason, "Another reason.");
    }

    /// The fixture helper creates parent directories before writing UTF-8 test files.
    fn write_fixture(root: &Path, path: &str, contents: &str) {
        let path = root.join(path);
        fs::create_dir_all(path.parent().expect("fixture has a parent directory"))
            .expect("fixture parent directory is created");
        fs::write(path, contents).expect("fixture file is written");
    }

    /// The inventory uses canonical paths across nested includes and duplicate basenames.
    #[test]
    fn fixture_finds_integrity_gaps_and_tracks_cfg_modules() {
        let directory = tempfile::tempdir().expect("temporary repository is created");
        let root = directory.path();
        write_fixture(
            root,
            "bk/crates/Cargo.toml",
            "[workspace]\nresolver = \"3\"\nmembers = [\"one\", \"two\"]\n",
        );
        for (package, folder) in [("one", "first"), ("two", "second")] {
            write_fixture(
                root,
                &format!("bk/crates/{package}/Cargo.toml"),
                &format!(
                    "[package]\nname = \"{package}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[[example]]\nname = \"demo\"\npath = \"examples/{folder}/main.rs\"\ntest = true\n"
                ),
            );
            write_fixture(
                root,
                &format!("bk/crates/{package}/examples/{folder}/main.rs"),
                "// ANCHOR: example\n#[cfg(target_os = \"none\")]\nmod gated;\nfn main() {}\n// ANCHOR_END: example\n",
            );
            write_fixture(
                root,
                &format!("bk/crates/{package}/examples/{folder}/gated.rs"),
                "pub fn gated() {}\n",
            );
        }
        write_fixture(
            root,
            "bk/src/SUMMARY.md",
            "# Summary\n\n- [Chapter](chapter.md)\n- [Missing](missing.md)\n",
        );
        write_fixture(
            root,
            "bk/src/chapter.md",
            "# Chapter\n\n{{#include fragments/one.md}}\n\n{{#example demo}}\n\n{{#include ../crates/one/examples/first/main.rs:example}}\n\n```rust\n{{#include ../crates/one/examples/first/main.rs:missing}}\n{{#include ../crates/one/examples/first/missing.rs:example}}\n```\n\n```rust\nlet value = 1;\n```\n\n<div class=\"hidden\">\n{{#include missing-hidden.rs}}\n</div>\n",
        );
        write_fixture(
            root,
            "bk/src/fragments/one.md",
            "```rust\n{{#include ../../crates/one/examples/first/main.rs:example}}\n```\n",
        );
        write_fixture(root, "bk/src/unlisted.md", "# Unlisted\n\nPlain prose.\n");
        write_fixture(
            root,
            "bk/drafts/SUMMARY.md",
            "# Summary\n\n- [Hidden](_hidden.md)\n",
        );
        write_fixture(
            root,
            "bk/drafts/_hidden.md",
            "# Hidden\n\n```rust\n{{#include ../crates/one/examples/first/main.rs:example}}\n```\n",
        );
        write_fixture(root, "later/src/SUMMARY.md", "# Summary\n");
        write_fixture(
            root,
            "scripts/example_chapter_exceptions.txt",
            "# No fixture exceptions\n",
        );

        let report = build_report(root).expect("fixture report builds");
        let codes = report
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.as_str())
            .collect::<Vec<_>>();
        assert!(codes.contains(&"include-anchor-missing"));
        assert!(codes.contains(&"include-missing"));
        assert!(codes.contains(&"chapter-unlisted"));
        assert!(codes.contains(&"include-unfenced"));
        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "summary-target-missing"
                && diagnostic.path == "bk/src/SUMMARY.md"
                && diagnostic.line == Some(4)
        }));
        let custom_example = report
            .includes
            .iter()
            .find(|include| {
                include.kind == "example" && include.custom_name.as_deref() == Some("demo")
            })
            .expect("the custom example directive is inventoried");
        assert!(custom_example.resolved);
        assert_eq!(custom_example.cargo_targets.len(), 2);
        let hidden_include = report
            .includes
            .iter()
            .find(|include| include.target_path.as_deref() == Some("bk/src/missing-hidden.rs"))
            .expect("hidden include is inventoried");
        assert!(!hidden_include.visible);
        assert!(!report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "include-missing"
                && diagnostic.path == hidden_include.source_path
                && diagnostic.line == Some(hidden_include.line)
        }));
        assert!(report.modules.iter().any(|module| {
            module.source_path.ends_with("/gated.rs")
                && module
                    .conditions
                    .iter()
                    .any(|condition| condition.contains("target_os"))
        }));
        let hidden_chapter = report
            .chapters
            .iter()
            .find(|chapter| chapter.path == "bk/drafts/_hidden.md")
            .expect("Summary-listed hidden chapter is inventoried");
        assert!(hidden_chapter.listed);
        assert!(hidden_chapter.hidden);
        assert!(hidden_chapter.has_tested_example);
        assert!(
            report
                .example_sources
                .iter()
                .any(|source| source.path.ends_with("/first/gated.rs"))
        );
        assert!(
            report
                .example_sources
                .iter()
                .any(|source| source.path.ends_with("/second/gated.rs"))
        );
        assert!(
            report
                .standalone_snippets
                .iter()
                .any(|snippet| snippet.path == "bk/src/chapter.md")
        );
        assert!(
            !report
                .standalone_snippets
                .iter()
                .any(|snippet| snippet.path == "bk/src/fragments/one.md")
        );
        let chapter = report
            .chapters
            .iter()
            .find(|chapter| chapter.path == "bk/src/chapter.md")
            .expect("chapter is inventoried");
        assert!(chapter.has_example);
        assert!(chapter.has_tested_example);
        assert!(chapter.listed);
        assert!(
            report.has_errors(),
            "visible integrity gaps fail the report"
        );
        let repeated = build_report(root).expect("the report can be rebuilt");
        assert_eq!(
            serde_json::to_vec(&report).expect("first report serializes"),
            serde_json::to_vec(&repeated).expect("second report serializes"),
            "the JSON inventory is deterministic"
        );
        assert_eq!(
            report
                .cargo_targets
                .iter()
                .filter(|target| target.name == "demo")
                .count(),
            2,
            "duplicate Cargo example names retain their package identities"
        );
    }
}
