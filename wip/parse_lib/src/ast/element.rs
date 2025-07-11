//! - Naked URLs: `http://...` or `https://...` optionally between < and >,
//! - Inline links: `[...](...)`,
//! - Reference-style links: `[...][...]`,
//! - Images,
//! - Reference definitions: `[...]: http://...`,
//! - Code spans and triple quoted text between ``` and ```.
//! - Hidden sections: <div class="hidden"> </div>

use std::fmt::Display;

use super::Directive;

#[derive(Debug, PartialEq, Eq)]
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

    /// Fenced Code Block enclosed in e.g. triple backticks (```).
    /// <https://spec.commonmark.org/0.31.2/#fenced-code-blocks>
    FencedCodeBlock(&'a str),

    /// An HTML div block, storing its raw content between `<div class="hidden"> </div>`.
    /// <https://rust-lang.github.io/mdBook/format/mdbook.html#html-classes-provided-by-mdbook>
    HiddenHtmlDiv(&'a str),

    /// A Markdown ATX heading: `# Some Text {#an-anchor}`.
    /// <https://spec.commonmark.org/0.31.2/#atx-headings>
    /// <https://rust-lang.github.io/mdBook/format/markdown.html#heading-attributes>
    Heading {
        level: u8,
        content: Option<&'a str>,
        id: Option<&'a str>,
    },

    /// Plain text that doesn't match other elements.
    Text(&'a str),

    /// Custom directive, e.g., `{{#crate my_crate}}`.
    CustomDirective(Directive<'a>),
}

impl Display for Element<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Autolink(url) => write!(f, "<{url}>"),
            Element::InlineLink { text, url, title } => {
                if let Some(title) = title {
                    write!(f, "[{text}]({url} \"{title}\")")
                } else {
                    write!(f, "[{text}]({url})")
                }
            }
            Element::ReferenceStyleLink { text, label } => {
                write!(f, "[{text}][{label}]")
            }
            Element::InlineImage {
                image_description,
                url,
                title,
            } => {
                if let Some(title) = title {
                    write!(f, "![{image_description}]({url} \"{title}\")")
                } else {
                    write!(f, "![{image_description}]({url})")
                }
            }
            Element::ReferenceStyleImage {
                image_description,
                label,
            } => {
                write!(f, "![{image_description}][{label}]")
            }
            Element::ReferenceDefinition { label, url, title } => {
                if let Some(title) = title {
                    write!(f, "[{label}]: {url} \"{title}\"")
                } else {
                    write!(f, "[{label}]: {url}")
                }
            }
            Element::WikiLink {
                target,
                display,
                immediately_after,
            } => {
                let immediately_after = immediately_after.unwrap_or("");
                if let Some(display) = display {
                    write!(f, "[[{target} | {display}]]{immediately_after}")
                } else {
                    write!(f, "[[{target}]]{immediately_after}")
                }
            }
            Element::CodeSpan(content) => write!(f, "`{content}`"),
            Element::FencedCodeBlock(content) => write!(f, "```\n{content}\n```"),
            Element::HiddenHtmlDiv(content) => {
                write!(f, "<div class=\"hidden\">\n{content}\n</div>")
            }
            Element::Heading { level, content, id } => {
                let hashes = "#".repeat(*level as usize);
                let content = content.map(|c| format!(" {c}")).unwrap_or("".into());
                if let Some(id) = id {
                    write!(f, "{hashes}{content} {{#{id}}}")
                } else {
                    write!(f, "{hashes}{content}")
                }
            }
            Element::Text(content) => write!(f, "{content}"),
            Element::CustomDirective(directive) => write!(f, "{directive}"),
        }
    }
}
