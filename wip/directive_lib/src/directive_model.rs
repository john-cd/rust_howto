//! The model for link / badge / crate block / example block directives.

#[derive(Debug, PartialEq, Eq)]
pub enum Directive<'a> {
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
    // // Word Index Anchor:
    // // {{i:<text>}}
    // // {{ii:<text>}}
    // // {{hi:<text>}}
    // IndexAnchor {
    //     kind: IndexAnchorKind,
    //     text: &'a str,
    // },

    // /// Convert a Wikipedia-style link to an internal page
    // /// into a Markdown link:
    // /// [[page | title]]
    // Wikilink { page: &'a str, title: &'a str },
}

#[derive(Debug, PartialEq, Eq)]
pub enum IndexAnchorKind {
    /// Indexed text appears as is.
    /// {{i:<text>}}
    Regular,
    /// Indexed text appears in italics.
    /// {{ii:<text>}}
    Italics,
    /// Indexed text is hidden.
    /// {{hi:<text>}}
    Hidden,
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

// ------------------------------

// [finish](https://github.com/john-cd/rust_howto/issues/1435)

// /// Process directive.
// pub fn process_directive(directive: &Directive) {
//     use Directive::*;
//     match directive {
//         Link { kind, name } => {}
//         Badge { kind, name } => {}
//         CrateBlock { crate_name, additional_categories } => {}
//         ExampleBlock { name } => {}
//         // IndexAnchor { kind, text } => {
//         //     // Do nothing.
//         // }
//         Wikilink { page, title } => {}
//     }
// }

/// Returns the replacement string for a given destination kind and crate name.
///
/// # Arguments
///
/// * `kind` - The kind of destination.
/// * `crate_name` - The name of the crate.
///
/// # Returns
///
/// A `String` containing the replacement string.
fn get_replacement_string(kind: DestinationKind, crate_name: &str) -> String {
    use DestinationKind::*;
    let (prefix, suffix) = match kind {
        Category => ("cat~", ""),
        Crate => ("c~", "~crate"),
        Docs => ("c~", "~docs"),
        GithubRepo => ("c~", "~repo"),
        LibRs => ("c~", "~lib.rs"),
        CratesIo => ("c~", "~crates.io"),
        Web => ("c~", "~website"),
    };

    format!(
        "[![{crate_name}][{prefix}{crate_name}{suffix}~badge]][{prefix}{crate_name}{suffix}]{{{{hi:{crate_name}}}}}"
    )
}
