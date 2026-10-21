// TODO Reviewed ??-??-2026
use std::ops::Range;

use anyhow::Context;
use parse_lib::ast::CodeSpanData;
use parse_lib::ast::FencedCodeBlockData;
use parse_lib::ast::HtmlCommentData;
use parse_lib::parse_document;
use parse_lib::visit::Visitor;
use regex::Captures;
use regex::Regex;

use super::conf::PreprocConfig;

#[derive(Default)]
pub struct ScrubRegexes {
    pub hidden_sections: Option<Regex>,
    pub hidden_includes: Option<Regex>,
}

/// Generate the regexes used to remove hidden content and hidden file includes.
pub fn get_regexes(conf: &PreprocConfig) -> ScrubRegexes {
    let hidden_sections = conf.remove_hidden_sections.then(|| {
        Regex::new(r#"<div *class *= *"hidden">([^<]|<[^/])+</ *div *>"#)
            .expect("Invalid hidden-section regex")
    });
    let hidden_includes = conf.do_not_include_hidden_chapters.then(|| {
        let re_string = r"[{]{2} *#(include|rustdoc_include|playground) *".to_string()
            + &conf.hidden_chapter_prefix
            + r"[^}]*?[}]{2}";
        Regex::new(&re_string).expect("Invalid hidden-chapter regex")
    });

    ScrubRegexes {
        hidden_sections,
        hidden_includes,
    }
}

pub fn wikilink_regex() -> Regex {
    Regex::new(r#"\[\[ *[^|\]]*?(?:\| *([^\]]+)?)?\]\]"#).expect("Invalid wikilink regex")
}

/// Apply a replacement outside code spans, fenced code blocks and HTML comments.
pub fn replace_outside_protected_spans<F>(
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
        source_base: markdown.as_ptr() as usize,
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

struct ProtectedSpanCollector {
    source_base: usize,
    source_len: usize,
    spans: Vec<Range<usize>>,
}

impl ProtectedSpanCollector {
    fn add_span(&mut self, text: &str) {
        let Some(start) = (text.as_ptr() as usize).checked_sub(self.source_base) else {
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

    #[test]
    fn replacement_skips_code_and_html_comments() {
        let source = concat!(
            "[[hidden]]\n\n",
            "`[[hidden-inline]]`\n\n",
            "```rust\n[[hidden-fenced]]\n```\n\n",
            "<!-- [[hidden-comment]] -->\n"
        );
        let regex = Regex::new(r"\[\[hidden[^]]*\]\]").unwrap();
        let result = replace_outside_protected_spans(source, &regex, |_| String::new()).unwrap();

        assert!(!result.contains("[[hidden]]"));
        assert!(result.contains("`[[hidden-inline]]`"));
        assert!(result.contains("[[hidden-fenced]]"));
        assert!(result.contains("[[hidden-comment]]"));
    }

    #[test]
    fn legacy_wikilink_cleanup_keeps_its_title() {
        let regex = wikilink_regex();
        let result = replace_outside_protected_spans("[[file | title]]", &regex, |captures| {
            captures
                .get(1)
                .map_or("", |matched| matched.as_str().trim())
                .to_string()
        })
        .unwrap();

        assert_eq!(result, "title");
    }
}
