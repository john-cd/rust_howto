//! Regular expressions that find hidden content, applied outside of code.
//!
//! TODO Reviewed ??-??-2026.

use std::ops::Range;

use anyhow::Context as _;
use parse_lib::ast::CodeSpanData;
use parse_lib::ast::FencedCodeBlockData;
use parse_lib::ast::HtmlCommentData;
use parse_lib::parse_document;
use parse_lib::visit::Visitor;
use regex::Captures;
use regex::Regex;

use super::conf::PreprocConfig;

/// Compiled patterns used to remove hidden content and includes.
#[derive(Default)]
pub(crate) struct ScrubRegexes {
    /// Pattern for HTML sections marked as hidden, when enabled.
    pub hidden_sections: Option<Regex>,
    /// Pattern for includes whose paths begin with the configured hidden prefix.
    pub hidden_includes: Option<Regex>,
}

/// Generate the regexes used to remove hidden content and hidden file includes.
pub(crate) fn get_regexes(conf: &PreprocConfig) -> ScrubRegexes {
    let hidden_sections = conf.remove_hidden_sections.then(|| {
        Regex::new(r#"<div *class *= *"hidden">([^<]|<[^/])+</ *div *>"#)
            .expect("Invalid hidden-section regex")
    });
    let hidden_includes = conf.do_not_include_hidden_chapters.then(|| {
        let hidden_chapter_prefix = regex::escape(&conf.hidden_chapter_prefix);
        let re_string = "[{]{2} *#(include|rustdoc_include|playground) *".to_owned()
            + &hidden_chapter_prefix
            + "[^}]*?[}]{2}";
        Regex::new(&re_string).expect("Invalid hidden-chapter regex")
    });

    ScrubRegexes {
        hidden_sections,
        hidden_includes,
    }
}

/// Builds the pattern used to recognize mdBook wikilinks.
pub(crate) fn wikilink_regex() -> Regex {
    Regex::new(r"\[\[ *[^|\]]*?(?:\| *([^\]]+)?)?\]\]").expect("Invalid wikilink regex")
}

/// Apply a replacement outside code spans, fenced code blocks and HTML comments.
pub(crate) fn replace_outside_protected_spans<F>(
    markdown: &str,
    regex: &Regex,
    replacement: F,
) -> anyhow::Result<String>
where
    F: Fn(&Captures<'_>) -> String,
{
    let document = parse_document("<in-memory Markdown>", None, markdown)
        .context("Failed to parse Markdown while cleaning residual directives")?;
    let mut collector = ProtectedSpanCollector {
        source_base: markdown.as_ptr().addr(),
        source_len: markdown.len(),
        spans: Vec::new(),
    };
    collector.visit_document(&document);
    collector.spans.sort_by_key(|range| range.start);

    let mut output = String::with_capacity(markdown.len());
    let mut cursor = 0;
    for span in collector.spans {
        let start = span.start.max(cursor);
        let end = span.end.max(start);
        if let Some(text) = markdown.get(cursor..start) {
            output.push_str(
                &regex.replace_all(text, |captures: &Captures<'_>| replacement(captures)),
            );
        }
        if let Some(text) = markdown.get(start..end) {
            output.push_str(text);
        }
        cursor = end;
    }
    if let Some(text) = markdown.get(cursor..) {
        output.push_str(&regex.replace_all(text, |captures: &Captures<'_>| replacement(captures)));
    }

    Ok(output)
}

/// Collects byte ranges occupied by Markdown constructs where replacements are skipped.
struct ProtectedSpanCollector {
    /// Address of the source buffer's first byte.
    source_base: usize,
    /// Length of the source buffer in bytes.
    source_len: usize,
    /// Protected ranges measured from the start of the source buffer.
    spans: Vec<Range<usize>>,
}

impl ProtectedSpanCollector {
    /// Adds the source range for a protected Markdown span when it belongs to the input buffer.
    fn add_span(&mut self, text: &str) {
        let Some(start) = text.as_ptr().addr().checked_sub(self.source_base) else {
            return;
        };
        let Some(end) = start.checked_add(text.len()) else {
            return;
        };
        if end <= self.source_len {
            self.spans.push(start..end);
        }
    }
}

impl<'a> Visitor<'a> for ProtectedSpanCollector {
    fn visit_fenced_code_block(&mut self, data: &'a FencedCodeBlockData<'a>) {
        for line in &data.lines {
            self.add_span(line);
        }
    }

    fn visit_code_span(&mut self, data: &'a CodeSpanData<'a>) {
        self.add_span(data.content);
    }

    fn visit_html_comment(&mut self, data: &'a HtmlCommentData<'a>) {
        for line in &data.lines {
            self.add_span(line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the default configuration builds both hidden-content patterns.
    #[test]
    fn test_get_regexes_default_config() {
        let conf = PreprocConfig::default();
        let regexes = get_regexes(&conf);

        let hidden_sections = regexes
            .hidden_sections
            .expect("hidden_sections regex should be created by default");
        assert!(hidden_sections.is_match(r#"<div class="hidden">secret text</div>"#));
        assert!(hidden_sections.is_match(r#"<div class = "hidden">secret text</div>"#));

        let hidden_includes = regexes
            .hidden_includes
            .expect("hidden_includes regex should be created by default");
        assert!(hidden_includes.is_match("{{#include _hidden.rs}}"));
        assert!(hidden_includes.is_match("{{#rustdoc_include _hidden.rs:anchor}}"));
        assert!(hidden_includes.is_match("{{#playground _hidden.rs}}"));
        assert!(!hidden_includes.is_match("{{#include public.rs}}"));
    }

    /// Verifies that disabling both hidden-content options omits their patterns.
    #[test]
    fn test_get_regexes_disabled() {
        let conf = PreprocConfig {
            remove_hidden_sections: false,
            do_not_include_hidden_chapters: false,
            hidden_chapter_prefix: "_".into(),
        };
        let regexes = get_regexes(&conf);

        assert!(regexes.hidden_sections.is_none());
        assert!(regexes.hidden_includes.is_none());
    }

    /// Verifies that the configured chapter prefix controls hidden include matches.
    #[test]
    fn test_get_regexes_custom_prefix() {
        let conf = PreprocConfig {
            remove_hidden_sections: true,
            do_not_include_hidden_chapters: true,
            hidden_chapter_prefix: "draft.".into(),
        };
        let regexes = get_regexes(&conf);

        let hidden_includes = regexes
            .hidden_includes
            .expect("the configured include pattern should be created");
        assert!(hidden_includes.is_match("{{#include draft.chapter.md}}"));
        assert!(!hidden_includes.is_match("{{#include draftXchapter.md}}"));
        assert!(!hidden_includes.is_match("{{#include _hidden_chapter.md}}"));
    }

    /// Verifies that enabling only one option generates only that specific pattern.
    #[test]
    fn test_get_regexes_partial_config() {
        let sections_only = PreprocConfig {
            remove_hidden_sections: true,
            do_not_include_hidden_chapters: false,
            hidden_chapter_prefix: "_".into(),
        };
        let regexes_sections = get_regexes(&sections_only);
        assert!(regexes_sections.hidden_sections.is_some());
        assert!(regexes_sections.hidden_includes.is_none());

        let includes_only = PreprocConfig {
            remove_hidden_sections: false,
            do_not_include_hidden_chapters: true,
            hidden_chapter_prefix: "_".into(),
        };
        let regexes_includes = get_regexes(&includes_only);
        assert!(regexes_includes.hidden_sections.is_none());
        assert!(regexes_includes.hidden_includes.is_some());
    }

    /// Verifies that regex meta-characters in the chapter prefix are properly escaped.
    #[test]
    fn test_get_regexes_special_characters_in_prefix() {
        let conf = PreprocConfig {
            remove_hidden_sections: false,
            do_not_include_hidden_chapters: true,
            hidden_chapter_prefix: "[draft].".into(),
        };
        let regexes = get_regexes(&conf);

        let hidden_includes = regexes
            .hidden_includes
            .expect("prefix with regex special characters should be built");
        assert!(hidden_includes.is_match("{{#include [draft].chapter.md}}"));
        assert!(!hidden_includes.is_match("{{#include d.chapter.md}}"));
        assert!(!hidden_includes.is_match("{{#include draft.chapter.md}}"));
    }

    /// Verifies hidden section HTML matching against various div structures and attributes.
    #[test]
    fn test_get_regexes_hidden_sections_matching_behavior() {
        let conf = PreprocConfig::default();
        let regexes = get_regexes(&conf);
        let hidden_sections = regexes
            .hidden_sections
            .expect("hidden_sections regex should be created");

        assert!(hidden_sections.is_match(r#"<div class="hidden">secret text</div>"#));
        assert!(hidden_sections.is_match(r#"<div   class  =  "hidden"  >secret text</  div  >"#));
        assert!(hidden_sections.is_match("<div class=\"hidden\">line1\nline2</div>"));
        assert!(hidden_sections.is_match(r#"<div class="hidden"><span>nested</span></div>"#));

        assert!(!hidden_sections.is_match(r#"<div class="visible">public text</div>"#));
        assert!(!hidden_sections.is_match("<div>no class</div>"));
    }

    /// Verifies hidden include pattern matching across all directive types and spacing.
    #[test]
    fn test_get_regexes_hidden_includes_directives() {
        let conf = PreprocConfig::default();
        let regexes = get_regexes(&conf);
        let hidden_includes = regexes
            .hidden_includes
            .expect("hidden_includes regex should be created");

        assert!(hidden_includes.is_match("{{#include _secret.rs}}"));
        assert!(hidden_includes.is_match("{{#rustdoc_include _secret.rs:anchor}}"));
        assert!(hidden_includes.is_match("{{#playground _secret.rs}}"));
        assert!(hidden_includes.is_match("{{  #include  _secret.rs  }}"));

        assert!(!hidden_includes.is_match("{{#include public.rs}}"));
        assert!(!hidden_includes.is_match("{{#other_directive _secret.rs}}"));
    }

    /// Verifies that cleanup leaves wikilinks inside code and HTML comments unchanged.
    #[test]
    fn replacement_skips_code_and_html_comments() {
        let source = concat!(
            "[[hidden]]\n\n",
            "`[[hidden-inline]]`\n\n",
            "```rust\n[[hidden-fenced]]\n```\n\n",
            "<!-- [[hidden-comment]] -->\n"
        );
        let regex = Regex::new(r"\[\[hidden[^]]*\]\]").expect("input should parse");
        let result = replace_outside_protected_spans(source, &regex, |_| String::new())
            .expect("the Markdown fragment should parse");

        assert!(!result.contains("[[hidden]]"));
        assert!(result.contains("`[[hidden-inline]]`"));
        assert!(result.contains("[[hidden-fenced]]"));
        assert!(result.contains("[[hidden-comment]]"));
    }

    /// Verifies that legacy wikilink cleanup keeps the displayed title.
    #[test]
    fn legacy_wikilink_cleanup_keeps_its_title() {
        let regex = wikilink_regex();
        let result = replace_outside_protected_spans("[[file | title]]", &regex, |captures| {
            captures
                .get(1)
                .map_or("", |matched| matched.as_str().trim())
                .to_owned()
        })
        .expect("the Markdown fragment should parse");

        assert_eq!(result, "title");
    }
}
