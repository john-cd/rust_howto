use std::borrow::Cow;
use std::fs::read_to_string;
use std::fs::File;
use std::fs::{self};
use std::io::BufRead;
use std::io::{self};
use std::path::Path;

use anyhow::Result;
use tracing::debug;

// // Read a single file to String
// pub fn read_to_string<P: AsRef<Path>>(path: P) -> String {
// let mut file = File::open(path).expect(format!("{:?} should
// exist.", path).as_str());
// let mut buf = String::new();
// // or String::with_capacity(50000);
// file.read_to_string(&mut buf).expect("file should be readable");
// buf
// }

pub fn read_all_markdown_files_in<'a, P>(
    markdown_root: P,
) -> Result<Cow<'a, str>>
where
    P: AsRef<Path>,
{
    // Locate the Markdown files with the src directory
    let paths = super::md::find_markdown_paths(markdown_root)?;

    // Read all .md files into one big String
    let mut buf = Vec::<String>::with_capacity(120);
    for p in paths {
        let s = fs::read_to_string(p.as_path())?;
        // debug!("{:?}: length = {}", p, s.len());
        buf.push(s);
    }
    let all_markdown = buf.concat(); //or .join("");

    Ok(Cow::from(all_markdown))
}

/// Read an existing reference definition file
/// (not a complete Markdown file with links and ref defs)
/// reader: Reader (e.g. File) to read from
// pub(super) fn read_refdefs_from<R, P>(ref_def_file_path: P) ->
// Result<Vec<Link>> where
//     P: AsRef<Path>,
//     R: std::io::Read,
// {
//     let lines = read_lines(ref_def_file_path)?;

//     Ok()
// }

pub fn read_lines<P>(file_path: P) -> Result<Vec<Cow<'static, str>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    // Returns an Iterator to the Reader of the lines of the file.
    Ok(io::BufReader::new(file)
        .lines()
        .flatten()
        .map(Cow::from)
        .collect())
}
