//! Finds inline link candidates in Markdown with a Pest grammar.
//!
//! TODO Reviewed ??-??-2026.

#![expect(
    clippy::exhaustive_enums,
    reason = "Pest derives a closed Rule enum for this grammar."
)]
#![allow(
    missing_docs,
    reason = "pest_derive emits Rule variants and parser methods without documentation."
)]

// ANCHOR: example
use pest::Parser;
use pest_derive::Parser;

/// Pest parser generated from the Markdown link-candidate grammar.
#[derive(Parser)]
#[grammar = "examples/pest_markdown/markdown_links.pest"]
struct MarkdownLinkParser;

/// Find the source spans of inline link candidates.
///
/// # Errors
///
/// Returns an error if the document grammar fails to parse the source.
fn inline_links(source: &str) -> anyhow::Result<Vec<&str>> {
    let pairs = MarkdownLinkParser::parse(Rule::document, source)?;
    Ok(pairs
        .flatten()
        .filter(|pair| pair.as_rule() == Rule::inline_link)
        .map(|pair| pair.as_str())
        .collect())
}

/// Print the link candidate outside the fenced code block.
fn main() -> anyhow::Result<()> {
    let source = "```text\n[example](ignored)\n```\n[guide](guide.md)";
    for link in inline_links(source)? {
        println!("{link}");
    }
    Ok(())
}
// ANCHOR_END: example

/// Checks link extraction and complete grammar matches across Markdown constructs.
#[cfg(test)]
mod tests {
    use super::*;

    /// Returns only the inline link outside the fenced example.
    #[test]
    fn extracts_visible_inline_link() -> anyhow::Result<()> {
        let source = "```text\n[example](ignored)\n```\n[guide](guide.md)";
        assert_eq!(inline_links(source)?, ["[guide](guide.md)"]);
        Ok(())
    }

    /// Check whether a grammar rule consumes the complete input.
    fn matches(rule: Rule, source: &str) -> bool {
        let Ok(mut pairs) = MarkdownLinkParser::parse(rule, source) else {
            return false;
        };
        pairs
            .next()
            .is_some_and(|pair| pair.as_span().end() == source.len())
    }

    /// Return every named rule emitted by the document scanner.
    ///
    /// # Arguments
    ///
    /// * `source` - Markdown source text to scan.
    ///
    /// # Panics
    ///
    /// Panics if the document grammar does not consume the entire source.
    fn document_rules(source: &str) -> Vec<Rule> {
        MarkdownLinkParser::parse(Rule::document, source)
            .expect("the document scanner should consume all source")
            .flatten()
            .map(|pair| pair.as_rule())
            .collect()
    }

    /// Parses inline links and images with balanced brackets, destinations, and optional titles.
    #[test]
    fn recognizes_inline_links_images_and_titles() {
        assert!(matches(Rule::inline_link, "[label](url)"));
        assert!(matches(Rule::inline_link, "[a [nested] label](path(a(b)))"));
        assert!(matches(Rule::inline_link, "[label](<a\\>b> 'title')"));
        assert!(matches(
            Rule::inline_link,
            "[label](\r\n  path\r\n  \"title\"\r\n)"
        ));
        assert!(matches(Rule::image_inline_link, "![alt](url)"));
        assert!(matches(Rule::inline_link, "[](url)"));
    }

    /// Reject invalid destinations while accepting empty and balanced destinations.
    #[test]
    fn rejects_invalid_inline_destinations() {
        assert!(!matches(Rule::inline_link, "[x](a b)"));
        assert!(!matches(Rule::inline_link, "[x](a(b)"));
        assert!(!matches(Rule::inline_link, "[x](<a<b>)"));
        assert!(matches(Rule::inline_link, "[x]()"));
        assert!(matches(Rule::inline_link, "[x](a(b)c)"));
    }

    /// Recognize each full, collapsed, and shortcut link or image candidate form.
    #[test]
    fn recognizes_reference_candidates() {
        assert!(matches(Rule::full_reference_link, "[text][label]"));
        assert!(matches(Rule::collapsed_reference_link, "[label][]"));
        assert!(matches(Rule::shortcut_reference_candidate, "[label]"));
        assert!(matches(Rule::image_full_reference_link, "![alt][label]"));
        assert!(matches(Rule::image_collapsed_reference_link, "![label][]"));
        assert!(matches(
            Rule::image_shortcut_reference_candidate,
            "![label]"
        ));
        assert!(!matches(Rule::full_reference_link, "[text][]"));
    }

    /// Parse definitions at document boundaries and reject invalid trailing content.
    #[test]
    fn recognizes_reference_definitions_and_boundaries() {
        assert!(matches(Rule::reference_definition, "[label]: /url"));
        assert!(matches(
            Rule::reference_definition,
            "   [label]:\r\n  <url>\r\n  'a title'"
        ));
        assert!(!matches(
            Rule::reference_definition,
            "[label]: /url \"title\" extra"
        ));
        assert!(!matches(Rule::reference_definition, "    [label]: /url"));
        assert!(!matches(Rule::reference_definition, "[]: /url"));
    }

    /// Match valid URI and email autolinks within CommonMark syntax limits.
    #[test]
    fn recognizes_uri_and_email_autolinks() {
        assert!(matches(
            Rule::commonmark_autolink,
            "<https://example.com/path?q=1>"
        ));
        assert!(matches(Rule::commonmark_autolink, "<a+b:c>"));
        assert!(matches(
            Rule::commonmark_autolink,
            "<Name+tag@example-domain.com>"
        ));
        assert!(matches(Rule::commonmark_autolink, "<n@x.y>"));
        assert!(matches(
            Rule::commonmark_autolink,
            &format!("<{}:x>", "a".repeat(32))
        ));
        assert!(matches(
            Rule::email_address,
            &format!("a@{}.com", "a".repeat(63))
        ));
        assert!(!matches(Rule::commonmark_autolink, "<m:abc>"));
        assert!(!matches(
            Rule::commonmark_autolink,
            &format!("<{}:x>", "a".repeat(33))
        ));
        assert!(!matches(
            Rule::commonmark_autolink,
            "<https://example.com/a b>"
        ));
        assert!(!matches(Rule::commonmark_autolink, "<n@hyphen-.com>"));
        assert!(!matches(
            Rule::email_address,
            &format!("a@{}.com", "a".repeat(64))
        ));
        assert!(!matches(Rule::commonmark_autolink, "<not-an-email>"));
    }

    /// Keep code spans and raw HTML ahead of link-text brackets.
    #[test]
    fn honors_inline_precedence_inside_link_text() {
        let source = "[code `a ] b` and <span title=\"]\">x</span>](url)";
        let rules = document_rules(source);
        assert!(rules.contains(&Rule::inline_link));
        assert!(rules.contains(&Rule::code_span));
        assert!(rules.contains(&Rule::html_opening_tag));
        assert!(rules.contains(&Rule::html_closing_tag));
    }

    /// Consume link-looking content inside fenced code blocks without recognizing it as a link.
    #[test]
    fn fenced_code_blocks_hide_link_candidates() {
        let source = "~~~rust\n[x](url)\n~~~~\n[visible](url)";
        let rules = document_rules(source);
        assert!(rules.contains(&Rule::tilde_fenced_code_block));
        assert_eq!(
            rules
                .iter()
                .filter(|rule| **rule == Rule::inline_link)
                .count(),
            1
        );
    }

    /// Recognize inline content after valid ATX opening markers only.
    #[test]
    fn recognizes_atx_heading_openings() {
        let rules = document_rules("## [heading](url)\n####### [ordinary](url)\n#hashtag");
        assert!(rules.contains(&Rule::atx_heading_opening));
        assert_eq!(
            rules
                .iter()
                .filter(|rule| **rule == Rule::inline_link)
                .count(),
            2
        );
        assert!(!matches(Rule::atx_heading, "####### heading\n"));
        assert!(!matches(Rule::atx_heading, "#hashtag\n"));
    }

    /// Keep book-specific URLs and directives separate from CommonMark autolinks.
    #[test]
    fn preserves_project_extensions() {
        let url_rules = document_rules("https://github.com/john-cd/rust_howto/issues/2243");
        assert!(url_rules.contains(&Rule::repository_bare_url));
        assert!(!url_rules.contains(&Rule::commonmark_autolink));
        assert!(document_rules("{{#include file.md}}").contains(&Rule::include_directive));
        assert!(document_rules("{{#crate parser}}").contains(&Rule::crate_directive));
        assert!(
            !document_rules("<div class=\"hidden\">[x](url)</div>\n").contains(&Rule::inline_link)
        );
    }

    /// Close code spans only with an equal-length, standalone backtick run.
    #[test]
    fn does_not_close_a_code_span_inside_a_longer_backtick_run() {
        assert!(!matches(Rule::code_span, "``code```"));
        assert!(matches(Rule::code_span, "``code``"));
    }
}
