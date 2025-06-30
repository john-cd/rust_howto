// use std::fs::File;
// use std::io::Read;
// use std::io::Write;
use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

static RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"^|[^"'](https?://)([^/\s]+)(/[^"'\[\])⮳]*)?"#).unwrap());

static GITHUB: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^|[^"'](https?://github.com/)([^/ ]+/)([^/ ]+)(/[^"'\[\])⮳]*)?"#).unwrap()
});

/// Replaces all directives in a (Markdown) file.
///
/// # Arguments
///
/// * `filepath` - The path to the file to process.
///
/// # Returns
pub fn replace_in_file(_filepath: &Path) -> Result<()> {
    // let mut file = File::open(filepath)?;
    // let size = file.metadata()?.len() as usize;
    // let mut buffer = String::with_capacity(size);
    // file.read_to_string(&mut buffer)?;
    // drop(file); // Close the file early.

    // if REGEX.is_match(&buffer) {
    //     let temp_filepath = filepath.with_extension(".tmp");
    //     let mut temp_file = File::create(&temp_filepath)?;
    //     buffer = replace(&buffer);
    //     temp_file.write_all(buffer.as_bytes())?;
    //     // Renames a file or directory to a new name, replacing the original file if it already exists.
    //     std::fs::rename(&temp_filepath, filepath)?;
    // }
    Ok(())
}

fn replace(text: &str) -> String {
    let mut res = text.to_string();
    res.reserve(150);
    // for (matching, typ, crates) in REGEX.captures_iter(text).map(|caps| {
    //     (
    //         caps.get(0).map_or("", |m| m.as_str()), // Guaranteed to return a non-None value.
    //         caps.get(1).map_or("", |m| m.as_str()),
    //         caps.get(2).map_or("", |m| m.as_str()),
    //     )
    // }) {

    //     let replacement = crates.split_whitespace().map(|crate_name| { format!(
    //             "[![{crate_name}][c~{crate_name}{suffix}~badge]][c~{crate_name}{suffix}]{{{{hi:{crate_name}}}}}"
    //         ) }).collect::<Vec<_>>().join("");
    //     tracing::debug!("{} {} {} {}", matching, typ, crates, replacement);
    //     res = res.replace(matching, &replacement);
    // }
    res
}

// # replace https::/github.com/.../...
// s=([^"'\''(]?https?://github.com/)([^/ ]+/)([^/ ]+)(/[^"'\'')⮳]*)?=[`\3`][\3~github] [\3~github]: \1\2\3\4=gp;

// # General case http://...
// s=([^"'\''(]?https?://)([^/\s]+)(/[^"'\'')⮳]*)?=[\2][\2] [\2]: \1\2\3=gp ;

// echo "${contents}" | rg --pcre2 --only-matching -r '[`$2`][$2~website] [$2~website]: $1$2$3' '(?<!: |["`([])(http(?:s)?://(?:www\d?\.)?)([^./]+)(\S+)?'
// echo "${contents}" | rg --pcre2 --only-matching -r '[`$2`][$2~github] [$2~github]: $1$2$3' '(?<!: |["`([])(http(?:s)?://(?:github\.com/)?)([^./]+)(\S+)?'

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_replace() {}
}
