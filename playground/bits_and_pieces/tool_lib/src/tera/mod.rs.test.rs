use std::collections::HashMap;
use anyhow::anyhow;
use walkdir::DirEntry;
use walkdir::WalkDir;

// fn build_table(templ: Tera) -> anyhow::Result<()> {
#[allow(dead_code)]
fn build_table() -> anyhow::Result<()> {
    let walker = WalkDir::new("./templates").into_iter()
        .filter_entry(|e| !is_hidden(e)) // no files / directories starting with `.`
        .filter_map(|e| e.ok()); // ignore errors
    let mut h: HashMap<String, String> = HashMap::new();

    for entry in walker {
        println!("Rendering {}", entry.path().display());
        let p = entry
            .path()
            .strip_prefix("templates")?
            .to_str()
            .ok_or(anyhow!("Error converting path to &str"))?;
        h.insert(p.to_string(), p.to_string());
    }
    Ok(())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}
