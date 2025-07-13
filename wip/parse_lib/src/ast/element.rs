//! - Naked URLs: `http://...` or `https://...` optionally between < and >,
//! - Inline links: `[...](...)`,
//! - Reference-style links: `[...][...]`,
//! - Images,
//! - Reference definitions: `[...]: http://...`,
//! - Code spans and triple quoted text between ``` and ```.
//! - Hidden sections: <div class="hidden"> </div>

use std::fmt::Display;

use super::DirectiveData;

#[derive(Debug, PartialEq, Eq)]
pub struct AutolinkData<'a> {
    pub url: &'a str, // The URL of the autolink.
}

#[derive(Debug, PartialEq, Eq)]
pub struct InlineLinkData<'a> {
    pub text: &'a str,
    pub url: &'a str,
    pub title: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReferenceStyleLinkData<'a> {
    pub text: &'a str,
    pub label: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InlineImageData<'a> {
    pub image_description: &'a str,
    pub url: &'a str,
    pub title: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReferenceStyleImageData<'a> {
    pub image_description: &'a str,
    pub label: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReferenceDefinitionData<'a> {
    pub label: &'a str,
    pub url: &'a str,
    pub title: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WikiLinkData<'a> {
    pub target: &'a str, // Target page.
    pub display: Option<&'a str>,
    pub immediately_after: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CodeSpanData<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FencedCodeBlockData<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HiddenHtmlDivData<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HeadingData<'a> {
    pub level: u8,
    pub content: Option<&'a str>,
    pub id: Option<&'a str>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TextData<'a> {
    pub content: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Element<'a> {
    /// A standalone URL: `http://...` or `https://...`
    /// optionally between < and > (i.e., an autolink).
    /// By design, more permissive than the CommonMark spec:
    /// <https://spec.commonmark.org/0.31.2/#autolinks>
    Autolink(AutolinkData<'a>),

    /// A Markdown-style inline link: `[text](url)`.
    /// <https://spec.commonmark.org/0.31.2/#inline-link>
    InlineLink(InlineLinkData<'a>),

    /// Reference-style link: `[...][...]`.
    /// <https://spec.commonmark.org/0.31.2/#reference-link>
    ReferenceStyleLink(ReferenceStyleLinkData<'a>),

    /// Images: `![foo](/url "title")`.
    /// <https://spec.commonmark.org/0.31.2/#images>
    InlineImage(InlineImageData<'a>),

    /// Reference-style image: `![foo][bar]`.
    ReferenceStyleImage(ReferenceStyleImageData<'a>),

    /// Reference definition: `[...]: http://... "title"`.
    /// <https://spec.commonmark.org/0.31.2/#link-reference-definitions>
    ReferenceDefinition(ReferenceDefinitionData<'a>),

    /// Wikilink: `[[target_page]]` or `[[target_page | display]]`, with or without spaces.
    /// <https://en.wikipedia.org/wiki/Help:Link#Wikilinks_(internal_links)>
    WikiLink(WikiLinkData<'a>),

    /// CodeSpan enclosed between ` and `.
    /// <https://spec.commonmark.org/0.31.2/#code-spans>
    CodeSpan(CodeSpanData<'a>),

    /// Fenced Code Block enclosed in e.g. triple backticks (```).
    /// <https://spec.commonmark.org/0.31.2/#fenced-code-blocks>
    FencedCodeBlock(FencedCodeBlockData<'a>),

    /// An HTML div block, storing its raw content between `<div class="hidden"> </div>`.
    /// <https://rust-lang.github.io/mdBook/format/mdbook.html#html-classes-provided-by-mdbook>
    HiddenHtmlDiv(HiddenHtmlDivData<'a>),

    /// A Markdown ATX heading: `# Some Text {#an-anchor}`.
    /// <https://spec.commonmark.org/0.31.2/#atx-headings>
    /// <https://rust-lang.github.io/mdBook/format/markdown.html#heading-attributes>
    Heading(HeadingData<'a>),

    /// Plain text that doesn't match other elements.
    Text(TextData<'a>),

    /// Custom directive, e.g., `{{#crate my_crate}}`.
    CustomDirective(DirectiveData<'a>),
}

impl Display for Element<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        Ok(())
    }
}
