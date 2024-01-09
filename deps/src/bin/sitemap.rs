//! Inspired by https://github.com/rxdn/mdbook-sitemap-generator/tree/master

use quick_xml::events::BytesText;
use quick_xml::writer::Writer;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};
use std::io::Write;

/// True if the directory entry is hidden (starts with a `.`)
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn find_paths<P: AsRef<Path>>(root_directory: P) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    let walker = WalkDir::new(root_directory)
        .min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| !is_hidden(e));

    for entry in walker
        .filter_map(|e| e.ok())
        .filter(|de| !is_hidden(de) && de.file_type().is_file())
    {
        if let Some(extension) = entry.path().extension() {
            if extension == "md" {
                println!("{}", entry.path().display());
                paths.push( entry.into_path().with_extension("html") );
            }
        }
    }
    Ok(paths)
}

fn write_xml(paths: Vec<PathBuf>, to: &Path) -> Result<(), Box<dyn Error>> {
    let mut buffer = File::create(to)?;
    let mut writer = Writer::new(&mut buffer);

    writer.write_bom()?;
    // Insert <?xml version="1.0" encoding="UTF-8"?>
    writer.get_mut().write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    // <urlset>
    writer
        .create_element("urlset")
        .with_attribute(("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9"))
        .write_inner_content(|writer| {
            for path in paths.iter() {
                // <url><loc>
                writer
                    .create_element("url")
                    .write_inner_content(
                        |w| {
                            w.create_element("loc")
                             .write_text_content(BytesText::new(path.to_str().ok_or("")?))?;
                            Ok::<_, Box<dyn Error>>(())
                        }
                    )?;
            }
            Ok::<_, Box<dyn Error>>(())
        })?;
    Ok::<_, Box<dyn Error>>(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let src = Path::new("src");
    let paths = find_paths(src)?;

    let sitemap_path = Path::new("sitemap.xml");
    write_xml(paths, sitemap_path)?;
    Ok(())
}
