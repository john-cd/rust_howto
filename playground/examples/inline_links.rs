// #!/usr/bin/env -S cargo +nightly -Zscript
// ---cargo
// [dependencies]
// anyhow = "1.0.95"
// regex = "1.11.1"
// walkdir = "2.5.0"
// xshell = "0.2.7"
// ---
// Rust script - see https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#script

// - Convert inline links e.g. [...](http://...) into reference-style links: [...][...] [...]: http://...
// - Process http://... naked links.
// - Do not convert links to GitHub issues e.g. https://github.com/john-cd/rust_howto/issues links.
// - Skip text between ``` and ```.
// - Move refdefs into central file.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;
use walkdir::WalkDir;
use xshell::{Shell, cmd};

static GITHUB: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(^|[^"'([]https?://github.com/)([^/ ]+/)([^/ ]+)(/[^"'[])⮳]*)?"#).unwrap();
});

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(^|[^"'([]https?://)([^/\s]+)(/[^"'[])⮳]*)?"#).unwrap();
});

fn main() -> anyhow::Result<()> {
    let root = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Missing root folder argument"))?;
    let root = Path::new(&root).canonicalize()?;

    let files = WalkDir::new(root.join("src"))
        .into_iter()
        .chain(WalkDir::new(root.join("drafts")).into_iter())
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().map_or(false, |ext| ext == "md")
                && !e.path().ends_with("refs.incl.md")
                && !e.path().ends_with("SUMMARY.md")
                && !e.path().to_string_lossy().contains("refs.md")
        })
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    for file in files {
        println!(">> {}", file.display());
        process_file(&file)?;
    }

    println!("DONE");
    Ok(())
}

fn process_file(file: &PathBuf) -> anyhow::Result<()> {
    let content = fs::read_to_string(file)?;
    let re_github = &*GITHUB;
    let re_general = &*RE;

    //let after = re.replace_all(before, "$m/$d/$y");
    // TODO
}
