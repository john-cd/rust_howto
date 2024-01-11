use std::collections::HashMap;

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

#[allow(clippy::vec_init_then_push)]
static GLOBAL_REGEX: Lazy<Vec<Rule>> = Lazy::new(|| {
    let mut v = Vec::new();

    // CATEGORIES
    // [cat-websocket-badge]: https://badge-cache.kominick.com/badge/websocket--x.svg?style=social
    // [cat-websocket]: https://crates.io/categories/web-programming::websocket/
    v.push(Rule{ re: r"https://crates.io/categories/(?<catg>\S*?)/?", label: "cat-${catg}",
                 badge_link: "https://badge-cache.kominick.com/badge/${catg}--x.svg?style=social"});

    // CRATES
    // [crates-io]: https://crates.io/
    v.push(Rule {
        re: r"https://crates.io(?:/crates)?/?",
        label: "crates-io",
        ..Rule::default()
    });

    // [smol-badge]: https://badge-cache.kominick.com/crates/v/smol.svg?label=smol
    // [smol-crate]: https://crates.io/crates/smol/
    v.push(Rule { re: r"https://crates.io/crates/(?<crate>\S+?)/?", label: "${crate}-crate",
 badge_link: "https://badge-cache.kominick.com/crates/v/${crate}.svg?label=${crate}"});

    // DOCS
    v.push(Rule {
        re: r"https://docs.rs/?",
        label: "docs-rs",
        badge_link: "TODO",
    });

    // [sqlx-badge]: https://badge-cache.kominick.com/crates/v/sqlx.svg?label=sqlx
    // [sqlx]: https://docs.rs/sqlx/
    v.push(Rule { re: r"https://docs.rs/(?<crate>)\S+?/?", label: "${crate}", badge_link: "https://badge-cache.kominick.com/crates/v/${crate}.svg?label=${crate}" });

    // STD DOCS
    // [std]: https://doc.rust-lang.org/std/"
    // [std-badge]: https://badge-cache.kominick.com/badge/std-1.75.0-blue.svg
    v.push(Rule { re: r"https://doc.rust-lang.org/std/?", label: "std", badge_link: "https://badge-cache.kominick.com/badge/std-1.75.0-blue.svg"});

    // [std::option::Option]: https://doc.rust-lang.org/std/option/"
    // [std::sync::atomic]: https://doc.rust-lang.org/std/sync/atomic/"
    // https://doc.rust-lang.org/core/cell/struct.OnceCell.html
    v.push(Rule {
        re: r"https://doc.rust-lang.org/(std|core)/(?<rest>\S*?)(?:.html)?",
        label: "TODO",
        ..Rule::default()
    });

    // LIB.RS
    v.push(Rule {
        re: r"https://lib.rs/?",
        label: "lib-rs",
        badge_link: "TODO",
    });
    // [sqlx-librs]: https://lib.rs/crates/sqlx/
    v.push(Rule {
        re: r"https://lib.rs/crates/(?<crate>\S+?)/?",
        label: "${crate}-lib-rs",
        ..Rule::default()
    });

    // GITHUB REPO WIKI
    // https://github.com/cross-rs/cross/wiki/Getting-Started
    v.push(Rule {
        re: r"https://github.com/(?<owner>\S*?)/(?<repo>\S*?)/wiki/(?:\S+?)",
        label: "${repo}-wiki",
        ..Rule::default()
    });

    // GITHUB
    // https://github.com/john-cd/rust_howto/blob/main/CONTRIBUTING.md
    v.push(Rule{ re: r"https://github.com/john-cd/rust_howto/(?:\S+?/)*(?<last>\S*)(?:/|.md)?", label: "${last}-rust-howto", ..Rule::default()});

    // [sqlx-github]: https://github.com/launchbadge/sqlx/
    // https://github.com/amar-laksh/workstation/blob/master/src/main.rs
    v.push(Rule {
        re: r"https://github.com/(?<owner>\S*?)/(?<repo>\S*?)(?:\S+?)",
        label: "${repo}-github",
        ..Rule::default()
    });

    // GITHUB PAGES
    // [rustup-documentation]: https://rust-lang.github.io/rustup/
    v.push(Rule {
        re: r"https://(?<owner>\S+?).github.io/(?<repo>\S+?)/?(?:\S*?)",
        label: "${repo}-github-pages",
        badge_link: "TODO",
    });

    //  BOOKS

    // RUST BOOK
    // [rust-book-badge]: https://img.shields.io/badge/Rust_Book-blue?logo=mdbook
    // [rust-book]: https://doc.rust-lang.org/book/
    v.push(Rule {
        re: r"https://doc.rust-lang.org/book/?",
        label: "rust-book",
        badge_link: "https://img.shields.io/badge/Rust_Book-blue?logo=mdbook",
    });

    // [box-rust-book-badge]: https://img.shields.io/badge/Box-blue?logo=mdbook
    // [box-rust-book]: https://doc.rust-lang.org/book/ch15-01-box.html
    v.push(Rule {
        re: r"https://doc.rust-lang.org/book(?:/\S+?)",
        label: "rust-book",
        badge_link: "https://img.shields.io/badge/${}-blue?logo=mdbook",
    });

    // RUST REFERENCE
    // [object-safe-reference-badge]: https://img.shields.io/badge/Object_Safe_Traits-green?logo=mdbook
    // [object-safe-reference]: https://doc.rust-lang.org/nightly/reference/items/traits.html#object-safety
    v.push(Rule {
        re: r"",
        label: "${}-rust-reference-book",
        badge_link: "https://img.shields.io/badge/${}-green?logo=mdbook",
    });

    // RUST BY EXAMPLE
    // [visibility-rules-rust-by-example-badge]: https://img.shields.io/badge/RBE-Visibility_Rules-violet?logo=mdbook
    // [visibility-rules-rust-by-example]: https://doc.rust-lang.org/rust-by-example/mod/visibility.html
    v.push(Rule{ re: r"https://doc.rust-lang.org/rust-by-example/(?:\S+?/)*(?<last>\S*?)(?:.html)?",
    label: "",
    badge_link: "https://img.shields.io/badge/RBE-${last}-violet?logo=mdbook"});

    // CARGO BOOK
    // [cargo-book]: https://doc.rust-lang.org/cargo/index.html
    v.push(Rule { re: r"https://doc.rust-lang.org/cargo/(?<rest>\S+?)(?:.html)?", label: "TODO",
    badge_link: "https://img.shields.io/badge/RBE-${last}-yellow?logo=mdbook"});

    // GENERIC
    v.push(Rule {
        re: r"http[s]?://\S+",
        label: "${domain}-website",
        ..Rule::default()
    });

    v
});

#[derive(Debug, Default)]
struct Rule<'a> {
    re: &'a str,         // Regex pattern to locate the url
    label: &'a str,      // regex replacement pattern
    badge_link: &'a str, // optional pattern to build a badge link
}

impl<'a> Rule<'a> {
    fn get_label(&self) -> &str {
        let mut l = "XYZ";
        // if !self.prefix.is_empty() { l = &[self.prefix, l].join("-") }
        // if !self.suffix.is_empty() { l = &[l, self.suffix].join("-") }
        l.trim()
    }

    fn get_badge_reference(&self) -> &str {
        self.badge_link
    }

    // [cat-emulators-badge]: https://badge-cache.kominick.com/badge/emulators--x.svg?style=social
    // [cat-emulators]: https://crates.io/categories/emulators/
    // fn get_badge(&self) -> String {
    //     let badge_link = "";

    //     format!("[{}-badge]: {}", self.get_reference_label(),
    // badge_link) }

    fn extract_from_url(s: &str) -> Result<()> {
        let u = Url::parse(s)?;
        if u.scheme() != "https" {
            println!("Not-https: {}", u.as_str());
        }
        if let Some(d) = u.domain() {}
        //(u.path())
        Ok(())
    }
}
