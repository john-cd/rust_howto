#![allow(clippy::vec_init_then_push)]

use anyhow::Result;
use once_cell::sync::Lazy;
use url::Url;
use tracing::info;

pub struct LabelAndBadge(String, String);

// TODO
pub fn process(url: Url, existing_label: Option<&str>) -> LabelAndBadge {
    let l = existing_label.unwrap_or_default();
    for r in GLOBAL_RULES.iter() {
        // if {

        // }
    }
    LabelAndBadge(String::new(), String::new())
}

fn extract_from_url(s: &str) -> Result<()> {
    let u = Url::parse(s)?;
    if u.scheme() != "https" {
        info!("Not-https: {}", u.as_str());
    }
    if let Some(d) = u.domain() {}
    //(u.path())
    Ok(())
}

/// Rule to create a reference label and/or a badge URL from a link URL
#[derive(Debug, Default)]
pub(crate) struct Rule<'a> {
    pub re: &'a str,                // Regex pattern to match the url
    pub label_pattern: &'a str,     // regex replacement pattern
    pub badge_url_pattern: &'a str, // optional pattern to build a badge link
}

static GLOBAL_RULES: Lazy<Vec<Rule>> = Lazy::new(|| {
    let mut v = Vec::new();

    // CATEGORIES
    // [cat-websocket-badge]: https://badge-cache.kominick.com/badge/websocket--x.svg?style=social
    // [cat-websocket]: https://crates.io/categories/web-programming::websocket/
    // Optional query
    v.push(Rule {
        re: r"https://crates.io/categories/(?<catg>\S*?)/?(?:\?\S+)?",
        label_pattern: "cat-${catg}",
        badge_url_pattern: "https://badge-cache.kominick.com/badge/${catg}--x.svg?style=social"
    });

    // CRATES
    // [crates-io]: https://crates.io/
    v.push(Rule {
        re: r"https://crates.io(?:/crates)?/?",
        label_pattern: "crates-io",
        ..Rule::default()
    });

    // [smol-badge]: https://badge-cache.kominick.com/crates/v/smol.svg?label=smol
    // [smol-crate]: https://crates.io/crates/smol/
    v.push(Rule {
        re: r"https://crates.io/crates/(?<crate>\S+?)/?",
        label_pattern: "crate-${crate}",
        badge_url_pattern: "https://badge-cache.kominick.com/crates/v/${crate}.svg?label=${crate}"
    });

    // DOCS
    v.push(Rule {
        re: r"https://docs.rs/?",
        label_pattern: "docs-rs",
        ..Rule::default()
    });

    // [sqlx-badge]: https://badge-cache.kominick.com/crates/v/sqlx.svg?label=sqlx
    // [sqlx]: https://docs.rs/sqlx/
    // [actix-web]: https://docs.rs/actix-web/latest/actix_web/
    // [join]: https://docs.rs/rayon/latest/rayon/fn.join.html
    // [spawn-blocking]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
    v.push(Rule {
        re: r"https://docs.rs/(?<crate>\S+?)(/latest/\1)?(?<item>/\S+*)*(?:.html)?",
        label_pattern: "${crate}-${item}",
        badge_url_pattern: "https://badge-cache.kominick.com/crates/v/${crate}.svg?label=${crate}"
    });

    // STD DOCS
    // [std]: https://doc.rust-lang.org/std/"
    // [std-badge]: https://badge-cache.kominick.com/badge/std-1.75.0-blue.svg
    v.push(Rule {
        re: r"https://doc.rust-lang.org/std/?",
        label_pattern: "std",
        badge_url_pattern:
            "https://badge-cache.kominick.com/badge/std-1.75.0-blue.svg",
    });

    // [std::option::Option]: https://doc.rust-lang.org/std/option/"
    // [std::sync::atomic]: https://doc.rust-lang.org/std/sync/atomic/"
    // [core::cell::OnceCell]: https://doc.rust-lang.org/core/cell/struct.OnceCell.html
    v.push(Rule {
        re: r"https://doc.rust-lang.org/(?<lib>std|core)/(?<rest>\S*?)(?:/|.html)?",
        label_pattern: "${lib}-${rest}",
        ..Rule::default()
    });

    // LIB.RS
    // https://lib.rs/
    v.push(Rule {
        re: r"https://lib.rs/?",
        label_pattern: "lib-rs",
        ..Rule::default()
    });
    // [sqlx-librs]: https://lib.rs/crates/sqlx/
    v.push(Rule {
        re: r"https://lib.rs/crates/(?<crate>\S+?)/?",
        label_pattern: "lib-rs-${crate}",
        ..Rule::default()
    });

    // GITHUB REPO WIKI
    // https://github.com/cross-rs/cross/wiki/Getting-Started
    v.push(Rule {
        re: r"https://github.com/(?<owner>\S*?)/(?<repo>\S*?)/wiki/(?:\S+?)",
        label_pattern: "${repo}-wiki",
        ..Rule::default()
    });

    // GITHUB
    // https://github.com/john-cd/rust_howto/blob/main/CONTRIBUTING.md
    v.push(Rule{
        re: r"https://github.com/john-cd/rust_howto/(?:\S+?/)*(?<last>\S*)(?:/|.md)?",
        label_pattern: "rust-howto-${last}",
        ..Rule::default()});

    // [sqlx-github]: https://github.com/launchbadge/sqlx/
    // https://github.com/amar-laksh/workstation/blob/master/src/main.rs
    v.push(Rule {
        re: r"https://github.com/(?<owner>\S*?)/(?<repo>\S*?)(?:\S+?)",
        label_pattern: "${repo}-github",
        ..Rule::default()
    });

    // [cross-example-toml]: https://github.com/cross-rs/wiki_assets/blob/main/Configuration/Cross.toml

    // GITHUB PAGES
    // [rustup-documentation]: https://rust-lang.github.io/rustup/
    // [rustup-command-examples]: https://rust-lang.github.io/rustup/examples.html
    v.push(Rule {
        re: r"https://(?<owner>\S+?).github.io/(?<repo>\S+?)/?(?:\S*?)",
        label_pattern: "${repo}-github-pages",
        badge_url_pattern:
            "https://img.shields.io/badge/${repo}-red?logo=githubpages",
    });

    //  BOOKS

    // RUST BOOK
    // [rust-book-badge]: https://img.shields.io/badge/Rust_Book-blue?logo=mdbook
    // [rust-book]: https://doc.rust-lang.org/book/
    v.push(Rule {
        re: r"https://doc.rust-lang.org/book/?",
        label_pattern: "rust-book",
        badge_url_pattern:
            "https://img.shields.io/badge/Rust_Book-blue?logo=mdbook",
    });

    // [box-rust-book-badge]: https://img.shields.io/badge/Box-blue?logo=mdbook
    // [box-rust-book]: https://doc.rust-lang.org/book/ch15-01-box.html
    v.push(Rule {
        re: r"https://doc.rust-lang.org/book/ch\d{2}-\d{2}-(?<item>).html",
        label_pattern: "rust-book-${item}",
        badge_url_pattern:
            "https://img.shields.io/badge/${item}-blue?logo=mdbook",
    });

    // RUST REFERENCE
    // [object-safe-reference-badge]: https://img.shields.io/badge/Object_Safe_Traits-green?logo=mdbook
    // [object-safe-reference]: https://doc.rust-lang.org/nightly/reference/items/traits.html#object-safety
    // [attributes-reference]: https://doc.rust-lang.org/reference/attributes.html
    // [conditional-compilation]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
    v.push(Rule {
        re: r"https://doc.rust-lang.org(?:/nightly)?/reference/(?<chapter>)/(?<item>\S+).html(?:#\S+)?",
        label_pattern: "rust-reference-book-${chapter}-${item}",
        badge_url_pattern: "https://img.shields.io/badge/${item}-green?logo=mdbook",
    });

    // RUST BY EXAMPLE
    // [rust-by-example-book-badge]: https://img.shields.io/badge/RBE-violet?logo=mdbook
    // [rust-by-example-book]: https://doc.rust-lang.org/rust-by-example/
    v.push(Rule {
        re: r"https://doc.rust-lang.org/rust-by-example/?",
        label_pattern: "rust-by-example",
        badge_url_pattern:
            "https://img.shields.io/badge/Rust_by_example-violet?logo=mdbook",
    });

    // [visibility-rules-rust-by-example-badge]: https://img.shields.io/badge/RBE-Visibility_Rules-violet?logo=mdbook
    // [visibility-rules-rust-by-example]: https://doc.rust-lang.org/rust-by-example/mod/visibility.html
    v.push(Rule {
        re: r"https://doc.rust-lang.org/rust-by-example/(?:\S+?/)*(?<last>\S*?)(?:.html)?",
        label_pattern: "rust-by-example-${last}",
        badge_url_pattern: "https://img.shields.io/badge/Rust_by_example-${last}-violet?logo=mdbook"
    });

    // CARGO BOOK
    // [cargo-book]: https://doc.rust-lang.org/cargo/index.html
    v.push(Rule {
        re: r"https://doc.rust-lang.org/cargo/(?<rest>\S+?)(?:.html)?",
        label_pattern: "cargo-book-${rest}",
        badge_url_pattern: "https://img.shields.io/badge/Cargo_Book-${last}-yellow?logo=mdbook",
    });

    // GENERIC
    v.push(Rule {
        re: r"http[s]?://(?<domain>[^/]+/?",
        label_pattern: "${domain}-website",
        ..Rule::default()
    });

    // [My terminal became more Rusty Community]: https://dev.to/22mahmoud/my-terminal-became-more-rusty-4g8l
    // [tokio-glossary]: https://tokio.rs/tokio/glossary
    // [tokio-tutorial]: https://tokio.rs/tokio/tutorial
    v.push(Rule {
        re: r"http[s]?://(?<domain>[^/]+)/(?:\S+?)/(?<last>[^\]+)(?:/|.html)?",
        label_pattern: "${domain}-${last}",
        ..Rule::default()
    });

    v
});
