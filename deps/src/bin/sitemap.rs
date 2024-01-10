//! Inspired by https://github.com/rxdn/mdbook-sitemap-generator/tree/master
use anyhow::anyhow;
use anyhow::bail;
use anyhow::Error;
use anyhow::Result;
use quick_xml::events::BytesText;
use quick_xml::writer::Writer;
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

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
        //.min_depth(1)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| !is_hidden(e)); // Skip hidden files and directories efficiently on unix systems

    for entry in walker
        .filter_map(|res| res.ok())
        .filter(|de| de.file_type().is_file())
    {
        if let Some(extension) = entry.path().extension() {
            if extension == "md" {
                //debug: println!("{}", entry.path().display());
                paths.push(entry.into_path());
            }
        }
    }
    Ok(paths)
}

fn write_xml(links: Vec<String>, mut dest_file: File) -> Result<()> {
    let mut writer = Writer::new_with_indent(&mut dest_file, b' ', 2);

    writer.write_bom()?;
    // Insert <?xml version="1.0" encoding="UTF-8"?>
    writer
        .get_mut()
        .write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n")?;
    // <urlset>
    writer
        .create_element("urlset")
        .with_attribute(("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9"))
        .write_inner_content(|writer| {
            for link in links.iter() {
                // <url><loc>
                writer.create_element("url").write_inner_content(|w| {
                    w.create_element("loc")
                        .write_text_content(BytesText::new(link.as_str()))?;
                    Ok::<_, Error>(())
                })?;
            }
            Ok::<_, Error>(())
        })?;
    Ok::<_, Error>(())
}

fn main() -> Result<()> {
    let src = Path::new("/code/src/");

    // Locate the Markdown file
    let paths = find_paths(src)?;

    // Remove a few exceptions
    let exclude = ["refs.md", "SUMMARY.md"];
    let l = paths.into_iter().filter(|p| {
        !exclude.iter().any(|&ex| {
            p.file_name()
                .unwrap_or(OsStr::new(""))
                .to_str()
                .unwrap_or("")
                .ends_with(ex)
        })
    }); // p.ends_with(ex) did not work?
        //debug: let l = l.map(|path| { println!("{:?}", path); path });

    let domain = "https://john-cd.com/rust_howto/";
    let l = l.map(|p| {
        p.with_extension("html")
            .strip_prefix(src) // Result<&Path, _>
            .map_err(anyhow::Error::from)
            .and_then(|p| p.to_str().ok_or(anyhow!("Non UTF-8 path: {:?}", p)))
            .map(|s| format!("{domain}{s}")) // Prefix with domain
    });

    // Separate links from errors and print
    let (links, errors): (Vec<_>, Vec<_>) = l.partition(Result::is_ok);
    let links: Vec<_> = links.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    //debug: println!("Links: {:?}", links);
    println!("Errors: {:?}", errors);

    // Create directory
    let dest_dir = "/code/book/";
    match Path::new(dest_dir).try_exists() {
        Ok(false) => {
            std::fs::create_dir_all(dest_dir)?;
            println!("{} created", dest_dir);
        }
        Ok(true) => {
            println!("{} already exists", dest_dir);
        }
        Err(_) => {
            bail!(
                "{}'s existence can neither be confirmed nor denied.",
                dest_dir
            );
        }
    }

    // Write the sitemap
    let sitemap_full_path = format!("{dest_dir}sitemap.xml");
    write_xml(links, File::create(Path::new(sitemap_full_path.as_str()))?)?;
    Ok(())
}
