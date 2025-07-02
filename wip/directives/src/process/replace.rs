
#![allow(dead_code)]

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

/// Replaces all directives in a (Markdown) file.
///
/// # Arguments
///
/// * `filepath` - The path to the file to process.
///
/// # Returns
pub fn replace_in_file(filepath: &Path) -> Result<()> {
    let mut file = File::open(filepath)?;
    let size = file.metadata()?.len() as usize;
    let mut buffer = String::with_capacity(size);
    file.read_to_string(&mut buffer)?;
    drop(file); // Close the file early.

    if REGEX.is_match(&buffer) {
        let temp_filepath = filepath.with_extension(".tmp");
        let mut temp_file = File::create(&temp_filepath)?;
        buffer = replace(&buffer);
        temp_file.write_all(buffer.as_bytes())?;
        // Renames a file or directory to a new name, replacing the original file if it already exists.
        std::fs::rename(&temp_filepath, filepath)?;
    }
    Ok(())
}

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\{\{\s*!\s*(docs|github|lib\.rs|crates\.io|web|crate)\s*:*\s*([^ }][^}]*)\}\}")
        .unwrap()
});

/// Replaces the following directives by the corresponding markdown, in a given string.
/// {{!crate xyz}}
/// {{!docs xyz}}
/// {{!github xyz}}
/// {{!lib.rs xyz}}
/// {{!crates.io xyz}}
/// {{!web xyz}}
///
/// # Arguments
///
/// * `text` - The text to process.
///
/// # Returns the updated String.
fn replace(text: &str) -> String {
    let mut res = text.to_string();
    res.reserve(150);
    for (matching, typ, crates) in REGEX.captures_iter(text).map(|caps| {
        (
            caps.get(0).map_or("", |m| m.as_str()), // Guaranteed to return a non-None value.
            caps.get(1).map_or("", |m| m.as_str()),
            caps.get(2).map_or("", |m| m.as_str()),
        )
    }) {
        let suffix = match typ {
            "crate" => "~docs", // FIXME
            "docs" => "~docs",
            "github" => "~github",
            "lib.rs" => "~lib.rs",
            "crates.io" => "~crates.io",
            "web" => "~website",
            _ => unreachable!(),
        };
        let replacement = crates.split_whitespace().map(|crate_name| { format!(
                "[![{crate_name}][c~{crate_name}{suffix}~badge]][c~{crate_name}{suffix}]{{{{hi:{crate_name}}}}}"
            ) }).collect::<Vec<_>>().join("");
        tracing::debug!("{} {} {} {}", matching, typ, crates, replacement);
        res = res.replace(matching, &replacement);
    }
    res
}

// #[allow(dead_code)]
// fn directives(conf: &PreprocConfig) -> Vec<RegexAndReplacement> {
//     let mut rr = vec![];

//     if conf.process_crate_directives {
//         // {{c: xyz }}
//         let re_string: String = r"\{\{c:\s*(\S+)\s*\}\}".into();
//         let re = Regex::new(&re_string).expect("Invalid regex");
//         //let replacement = "";
//         rr.push(RegexAndReplacement {
//             re,
//             replacement: None,
//         });
//     }
//     if conf.process_category_directives {
//         // {{c: parsing }} -> [parsing][cat~parsing]⮳{{hi:parsing}}
//         // [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard
//         // library}}
//         let re_string: String = r"\{\{cat:\s*(\S+)\s*\}\}".into();
//         let re = Regex::new(&re_string).expect("Invalid regex");
//         let replacement = "[$1][cat~$1]{{hi: $1}}";
//         rr.push(RegexAndReplacement {
//             re,
//             replacement: None, Some(Box::new(|_| replacement.into())),
//         });
//     }
//     if conf.process_page_directives {
//         let re_string: String = r"\s*(\S+)\s*".into();
//         let re = Regex::new(&re_string).expect("Invalid regex");
//         let replacement = "[$1][p~$1]{{hi:$1}}";
//         rr.push(RegexAndReplacement {
//             re,
//             replacement: Some(Box::new(|_| replacement.into())),
//         });
//     }
//     rr
// }
