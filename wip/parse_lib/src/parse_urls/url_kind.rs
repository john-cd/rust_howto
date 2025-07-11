//! Recognize specific URLs:
//!
//! - GitHub issues for the book repo: https://github.com/john-cd/rust_howto/issues/123
//! - GitHub repo URLs: https://github.com/rust-unofficial/awesome-rust
//! - `crates.io` crate URLs: https://crates.io/crates/ab_glyph
//! - `docs.rs` crate doc URLs: https://docs.rs/ab_glyph
//! - `docs.rs` item doc URLs: https://docs.rs/ansi_term/latest/ansi_term/type.ANSIString.html
//! - `lib.rs`: https://lib.rs/crates/ab_glyph
//! - Other URLs e.g., https://actix.rs

use url::Url;

#[derive(Debug, PartialEq, Eq)]
pub enum UrlKind<'a> {
    /// GitHub issue for the book repo, e.g.
    /// https://github.com/john-cd/rust_howto/issues/123
    GitHubBookIssue { url_text: &'a str },
    /// GitHub repository, e.g.
    /// https://github.com/OWNER/REPOSITORY
    GitHubRepo { owner: &'a str, repository: &'a str },
    /// Crate on `crates.io`, e.g.
    /// https://crates.io/crates/ab_glyph
    /// https://crates.io/crates/data-encoding
    Crate { crate_name: &'a str },
    /// Documentation for a crate on `docs.rs`, e.g.
    /// https://docs.rs/ab_glyph
    CrateDoc { crate_name: &'a str },
    /// Item documentation on `docs.rs`, e.g.
    /// https://docs.rs/ansi_term/latest/ansi_term/type.ANSIString.html
    /// https://docs.rs/data-encoding/latest/data_encoding/struct.Encoding.html#method.encode
    ItemDoc { crate_name: &'a str, page: &'a str },
    /// Crate on `lib.rs`, e.g.
    /// https://lib.rs/crates/ab_glyph
    /// https://lib.rs/crates/data-encoding
    LibRsCrate { crate_name: &'a str },
    /// None of the above, e.g.
    /// https://actix.rs, https://lib.rs/
    Other { url_text: &'a str },
}

pub fn url_kind(url_text: &str) -> Result<UrlKind, url::ParseError> {
    let url = Url::parse(url_text)?;

    // Hack: `UrlKind` contains references to parts of the URL text,
    // `url` somehow internally clones,
    // therefore we can't return a reference to `Url::parse(url_text).path()` directly.
    let mut path: &str = "";
    if let Some(p) = url.path().strip_prefix('/') {
        let start = url_text.find(p).unwrap_or_default();
        // Get position / range of the path in the original string
        // (`substr_range()` is still experimental).
        path = &url_text[start..(start + p.len())];
    }

    let segments: Vec<&str> = path.split('/').collect();
    let (first, second, fourth) = (
        segments.first().filter(|s| !s.is_empty()),
        segments.get(1).filter(|s| !s.is_empty()),
        segments.get(3).filter(|s| !s.is_empty()),
    );

    let kind = match url.host_str() {
        Some("github.com") if first == Some(&"john-cd") && second == Some(&"rust_howto") => {
            UrlKind::GitHubBookIssue { url_text }
        }
        Some("github.com") if first.is_some() && second.is_some() => UrlKind::GitHubRepo {
            owner: first.unwrap(),
            repository: second.unwrap(),
        },
        Some("crates.io") if second.is_some() => UrlKind::Crate {
            crate_name: second.unwrap(),
        },
        Some("docs.rs") if fourth.is_some() => UrlKind::ItemDoc {
            crate_name: first.unwrap(),
            page: fourth.unwrap(),
        },
        Some("docs.rs") if first.is_some() => UrlKind::CrateDoc {
            crate_name: first.unwrap(),
        },
        Some("lib.rs") if second.is_some() => UrlKind::LibRsCrate {
            crate_name: second.unwrap(),
        },
        _ => UrlKind::Other { url_text },
    };
    Ok(kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_host_and_path() -> anyhow::Result<()> {
        let url_text = "https://github.com/john-cd/rust_howto/issues/123";
        let kind = url_kind(url_text)?;
        assert_eq!(kind, UrlKind::GitHubBookIssue { url_text });
        Ok(())
    }

    #[test]
    fn test_parse_url_host_and_path2() -> anyhow::Result<()> {
        let url_text = "https://github.com/rust-unofficial/awesome-rust";
        let kind = url_kind(url_text)?;
        assert_eq!(
            kind,
            UrlKind::GitHubRepo {
                owner: "rust-unofficial",
                repository: "awesome-rust"
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_url_host_and_path3() -> anyhow::Result<()> {
        let url_text = "https://crates.io/crates/data-encoding";
        let kind = url_kind(url_text)?;
        assert_eq!(
            kind,
            UrlKind::Crate {
                crate_name: "data-encoding"
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_url_host_and_path4() -> anyhow::Result<()> {
        let url_text =
            "https://docs.rs/data-encoding/latest/data_encoding/struct.Encoding.html#method.encode";
        let kind = url_kind(url_text)?;
        assert_eq!(
            kind,
            UrlKind::ItemDoc {
                crate_name: "data-encoding",
                page: "struct.Encoding.html"
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_url_host_and_path5() -> anyhow::Result<()> {
        let url_text = "https://docs.rs/ab_glyph";
        let kind = url_kind(url_text)?;
        assert_eq!(
            kind,
            UrlKind::CrateDoc {
                crate_name: "ab_glyph"
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_url_host_and_path6() -> anyhow::Result<()> {
        let url_text = "https://lib.rs/crates/data-encoding";
        let kind = url_kind(url_text)?;
        assert_eq!(
            kind,
            UrlKind::LibRsCrate {
                crate_name: "data-encoding"
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_url_host_and_path7() -> anyhow::Result<()> {
        let url_text = "https://docs.rs/";
        let kind = url_kind(url_text)?;
        assert_eq!(
            kind,
            UrlKind::Other {
                url_text: "https://docs.rs/"
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_url_host_and_path8() -> anyhow::Result<()> {
        let url_text = "https://docs.rs";
        let kind = url_kind(url_text)?;
        assert_eq!(
            kind,
            UrlKind::Other {
                url_text: "https://docs.rs"
            }
        );
        Ok(())
    }

    // use url::Host;
    // use url::Position;

    // #[test]
    // fn test() -> anyhow::Result<()> {
    //     let issue_list_url =
    //         Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;

    //     assert!(issue_list_url.scheme() == "https");
    //     assert!(issue_list_url.username() == "");
    //     assert!(issue_list_url.password() == None);
    //     assert!(issue_list_url.host_str() == Some("github.com"));
    //     assert!(issue_list_url.host() == Some(Host::Domain("github.com")));
    //     assert!(issue_list_url.port() == None);
    //     assert!(issue_list_url.path() == "/rust-lang/rust/issues");
    //     assert!(
    //         issue_list_url
    //             .path_segments()
    //             .map(|c| c.collect::<Vec<_>>())
    //             == Some(vec!["rust-lang", "rust", "issues"])
    //     );
    //     assert!(issue_list_url.query() == Some("labels=E-easy&state=open"));
    //     assert!(
    //         &issue_list_url[Position::BeforePath..]
    //             == "/rust-lang/rust/issues?labels=E-easy&state=open"
    //     );
    //     assert!(issue_list_url.fragment() == None);
    //     assert!(!issue_list_url.cannot_be_a_base());
    //     Ok(())
    // }
}
