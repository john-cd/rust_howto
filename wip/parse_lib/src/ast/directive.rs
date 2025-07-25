//! The model for link / badge / crate block / example block directives.

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum DirectiveData<'a> {
    /// Insert a Markdown link.
    ///
    /// - category: {{cat xyz}}
    /// - internal page: {{crate xyz}}
    /// - `docs.rs` link: {{docs xyz}}
    /// - Github link: {{github xyz}}
    /// - `lib.rs` link: {{lib.rs xyz}}
    /// - `crates.io` link: {{crates.io xyz}}
    /// - Website for the crate: {{web xyz}}
    Link {
        kind: DestinationKind,
        name: &'a str,
    },

    /// Insert a Badge,
    /// e.g. `[![some_crate][c~some_crate~docs~badge]][c~some_crate~docs]`.
    ///
    /// {{!cat xyz}}
    /// {{!crate xyz}}
    /// {{!docs xyz}}
    /// {{!github xyz}}
    /// {{!lib.rs xyz}}
    /// {{!crates.io xyz}}
    /// {{!web xyz}}
    Badge {
        kind: DestinationKind,
        name: &'a str,
    },

    /// Insert a Crate block e.g., multiple badges for a given crate:
    /// {{#crate crt}}
    ///
    /// With optional additional categories:
    /// {{#crate: crt cat1 cat-2 cat-2-2 cat3::sub-cat-3 }}
    CrateBlock {
        crate_name: &'a str,
        additional_categories: Vec<&'a str>,
    },

    /// Insert a fenced code block with an `include` directive pointing to a new code example (.rs) file:
    /// {{#example some_example}} -> ```rust,editable\n{{#include ...}}```
    ExampleBlock { name: &'a str },
    // [finish](https://github.com/john-cd/rust_howto/issues/1426)
    // /// Word Index Anchor:
    // /// {{i:<text>}}
    // /// {{ii:<text>}}
    // /// {{hi:<text>}}
    // IndexAnchor {
    //     kind: super::IndexAnchorKind,
    //     text: &'a str,
    // },
}

impl Display for DirectiveData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectiveData::Link { kind, name } => write!(f, "{{{kind} {name}}}"),
            DirectiveData::Badge { kind, name } => write!(f, "{{!{kind} {name}}}"),
            DirectiveData::CrateBlock {
                crate_name,
                additional_categories,
            } => {
                if additional_categories.is_empty() {
                    write!(f, "{{#crate {crate_name}}}")
                } else {
                    let categories = &additional_categories.join(" ");
                    write!(f, "{{#crate {crate_name} {categories}}}")
                }
            }
            DirectiveData::ExampleBlock { name } => write!(f, "{{#example {name}}}"),
        }
    }
}

/// Represents the kind of directive being parsed.
/// For link or badge directives only.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DestinationKind {
    Category,   // Link to or badge for a `crates.io` category page.
    Crate,      // Link to or badge for an internal crate page.
    Docs,       // Link to or badge for a `docs.rs` crate page.
    GithubRepo, // Link to or badge for a GitHub repo.
    LibRs,      // Link to or badge for a `lib.rs` crate page.
    CratesIo,   // Link to or badge for a `crates.io` crate page.
    Web,        // Link to or badge for another website.
}

impl Display for DestinationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DestinationKind::Category => write!(f, "cat"),
            DestinationKind::Crate => write!(f, "crate"),
            DestinationKind::Docs => write!(f, "docs"),
            DestinationKind::GithubRepo => write!(f, "github"),
            DestinationKind::LibRs => write!(f, "lib.rs"),
            DestinationKind::CratesIo => write!(f, "crates.io"),
            DestinationKind::Web => write!(f, "web"),
        }
    }
}
