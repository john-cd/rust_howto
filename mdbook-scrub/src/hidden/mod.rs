use regex::Regex;
use std::borrow::Cow;
use std::sync::LazyLock;

static HIDDEN_SECTIONS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"").expect("Invalid regex"));

static INCLUDES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"").expect("Invalid regex"));

// Remove the hidden sections between `<div class="hidden">` and `</div>`
pub fn remove_hidden_sections(content: &mut Cow<'_, str>) {}

// Remove `{{#includes }}` that points to hidden files (files that start with `hidden_chapter_prefix` e.g. _)
pub fn remove_include_hidden_chapters(content: &mut Cow<'_, str>, hidden_chapter_prefix: &str) {}
