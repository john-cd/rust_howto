use std::fs;
use std::path::Path;

use anyhow::Result;

// // Read a single file to String
// pub(crate) fn read_to_string<P: AsRef<Path>>(path: P) -> String {
// let mut file = File::open(path).expect(format!("{:?} should
// exist.", path).as_str());
// let mut buf = String::new();
// // or String::with_capacity(50000);
// file.read_to_string(&mut buf).expect("file should be readable");
// buf
// }

pub(crate) fn read_all_markdown_files_in(
    markdown_root: &str,
) -> Result<String> {
    // Locate the Markdown files with the src directory
    let paths = utils::find_markdown_paths(Path::new(markdown_root))?;

    // Read all .md files into one big String
    let mut buf = Vec::<String>::with_capacity(120);
    for p in paths {
        let s = fs::read_to_string(p.as_path())?;
        // debug!("{:?}: length = {}", p, s.len());
        buf.push(s);
    }
    let all_markdown = buf.concat(); //or .join("");

    Ok(all_markdown)
}
