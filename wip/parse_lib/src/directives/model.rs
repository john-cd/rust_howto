// TODO revise the AST for directives.
// review mdbook-scrub test book for directive examples.
#[derive(Debug, PartialEq)]
pub enum Directive<'a> {
    /// Insert a Markdown link:
    /// -> `[text][label]`⮳.
    Link { kind: LinkKind, name: &'a str },
    /// Insert a Badge:
    /// -> `[![some_crate][c~some_crate~docs~badge]][c~some_crate~docs]`.
    Badge { kind: LinkKind, name: &'a str },
    /// Insert a Crate block e.g., multiple badges for a given crate:
    /// {{#crate crt}} ->
    CrateBlock { crate_name: &'a str },
    /// Insert a fenced code block with an `include` directive pointing to a new code example (.rs) file:
    /// {{#example some_example}} -> ```rust,editable\n{{#include ...}}```
    ExampleBlock { name: &'a str },
    // Word Index Entry:
    // {{i: }} ->
    // {{hi: }} ->
    // TODO
}

/// Represents the kind of directive being parsed.
/// For links or badges only.
#[derive(Debug, PartialEq)]
pub enum LinkKind {
    Category,   //  Link or badge to `crates.io` category page.
    Crate,      // Link or badge to internal crate page.
    Docs,       // Link or badge to `docs.rs` crate page.
    GithubRepo, // Link or badge to GitHub repo.
    LibRs,      // Link or badge to `lib.rs` crate page.
    CratesIo,   // link or badge to `crates.io` crate page.
    Web,        // Other website.
}
