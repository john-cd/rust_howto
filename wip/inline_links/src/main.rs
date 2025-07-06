#![allow(dead_code)]
//! Convert any naked URLs and inline links in Markdown files within a folder into reference-style links.
//!
//! - Convert inline links e.g. [...](http(s)://...) into reference-style links: [...][...] [...]: http://...
//! - Convert http(s)://... naked links into reference-style links.
//! - Skip URLs between ``` and ``` and within hidden sections: <div class="hidden">...</div>
//! - Ignore URLs within reference-style link labels: [https://...][...]
//! - Do not convert links to GitHub issues e.g. https://github.com/john-cd/rust_howto/issues links.
//! - Move refdefs into a central file (and sort them).

mod cli;

use std::borrow::Cow;

use clap::Parser;

fn main() -> anyhow::Result<()> {
    // Install a global tracing subscriber that listens for events
    // and filters based on the value of the RUST_LOG environment variable.
    tracing_subscriber::fmt::init();

    // Process command-line arguments to retrieve the directory to process:
    let args = cli::Args::parse();
    let scope = core_lib::Scope::default();
    for directory in &args.directories {
        let dir = directory.as_path().canonicalize()?;
        println!("Processing {}", dir.display());
        core_lib::walk_directory_and_process_files(&dir, &scope, convert_links)?;
    }
    println!("DONE");
    Ok(())
}

/// Convert inline and naked links to reference-style links.
fn convert_links(filepath: &std::path::Path) -> anyhow::Result<()> {
    // Read a text file in memory, test if its contents should be processed, and if true, update its contents.
    core_lib::process_text_file(
        filepath,
        |_s: &str| true, // TODO
        |_s: &str| Cow::Borrowed(""),
    )?;
    Ok(())
}

// TODO reuse fragments below if needed.

// # replace https::/github.com/.../...
// s=([^"'\''(]?https?://github.com/)([^/ ]+/)([^/ ]+)(/[^"'\'')⮳]*)?=[`\3`][\3~github] [\3~github]: \1\2\3\4=gp;

// # General case http://...
// s=([^"'\''(]?https?://)([^/\s]+)(/[^"'\'')⮳]*)?=[\2][\2] [\2]: \1\2\3=gp ;

// echo "${contents}" | rg --pcre2 --only-matching -r '[`$2`][$2~website] [$2~website]: $1$2$3' '(?<!: |["`([])(http(?:s)?://(?:www\d?\.)?)([^./]+)(\S+)?'
// echo "${contents}" | rg --pcre2 --only-matching -r '[`$2`][$2~github] [$2~github]: $1$2$3' '(?<!: |["`([])(http(?:s)?://(?:github\.com/)?)([^./]+)(\S+)?'

// REGEXES

// use std::sync::LazyLock;
// use regex::Regex;
// static RE: LazyLock<Regex> =
//     LazyLock::new(|| Regex::new(r#"^|[^"'](https?://)([^/\s]+)(/[^"'\[\])⮳]*)?"#).unwrap());

// static GITHUB: LazyLock<Regex> = LazyLock::new(|| {
//     Regex::new(r#"^|[^"'](https?://github.com/)([^/ ]+/)([^/ ]+)(/[^"'\[\])⮳]*)?"#).unwrap()
// });

//

// let (matching, typ, crates) =
//     (
//         caps.get(0).map_or("", |m| m.as_str()), // Guaranteed to return a non-None value.
//         caps.get(1).map_or("", |m| m.as_str()),
//         caps.get(2).map_or("", |m| m.as_str()),
//     );

// format!(
//             "[![{crate_name}][c~{crate_name}{suffix}~badge]][c~{crate_name}{suffix}]{{{{hi:{crate_name}}}}}"
//         ) }).collect::<Vec<_>>().join("");
