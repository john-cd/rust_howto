//! AST (Abstract Syntax Tree):
//!
//! - Naked URLs: `http://...` or `https://...` optionally between < and >,
//! - Inline links: `[...](...)`,
//! - Reference-style links: `[...][...]`,
//! - Images,
//! - Reference definitions: `[...]: http://...`,
//! - Code spans and triple quoted text between ``` and ```.
//! - Hidden sections: <div class="hidden"> </div>

// TODO finish

#[derive(Debug, PartialEq)]
pub enum Element<'a> {
    /// A standalone URL: `http://...` or `https://...`
    /// optionally between < and > (i.e., an autolink).
    /// By design, more permissive than the CommonMark spec:
    /// <https://spec.commonmark.org/0.31.2/#autolinks>
    Autolink(&'a str),

    /// A Markdown-style inline link: `[text](url)`.
    /// <https://spec.commonmark.org/0.31.2/#inline-link>
    InlineLink {
        text: &'a str,
        url: &'a str,
        title: Option<&'a str>,
    },

    /// Reference-style link: `[...][...]`.
    /// <https://spec.commonmark.org/0.31.2/#reference-link>
    ReferenceStyleLink { text: &'a str, label: &'a str },

    /// Images: `![foo](/url "title")`.
    /// <https://spec.commonmark.org/0.31.2/#images>
    InlineImage {
        image_description: &'a str,
        url: &'a str,
        title: Option<&'a str>,
    },

    /// Reference-style image: `![foo][bar]`.
    ReferenceStyleImage {
        image_description: &'a str,
        label: &'a str,
    },

    /// Reference definition: `[...]: http://... "title"`.
    /// <https://spec.commonmark.org/0.31.2/#link-reference-definitions>
    ReferenceDefinition {
        label: &'a str,
        url: &'a str,
        title: Option<&'a str>,
    },

    /// Wikilink: `[[target_page]]` or `[[target_page | display]]`, with or without spaces.
    /// <https://en.wikipedia.org/wiki/Help:Link#Wikilinks_(internal_links)>
    WikiLink {
        target: &'a str, // Target page.
        display: Option<&'a str>,
        immediately_after: Option<&'a str>,
    },

    /// CodeSpan enclosed between ` and `.
    /// <https://spec.commonmark.org/0.31.2/#code-spans>
    CodeSpan(&'a str),

    /// Fenced Code Block enclosed in e.g. triple single quotes (''').
    /// <https://spec.commonmark.org/0.31.2/#fenced-code-blocks>
    FencedCodeBlock(&'a str),

    /// An HTML div block, storing its raw content between `<div class="hidden"> </div>`.
    /// <https://rust-lang.github.io/mdBook/format/mdbook.html#html-classes-provided-by-mdbook>
    HiddenHtmlDiv(&'a str),

    /// Plain text that doesn't match other elements.
    Text(&'a str),
}
