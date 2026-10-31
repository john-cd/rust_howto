// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Extracting links from Markdown with several regular expressions.
//!
//! A regular expression (regex) describes a text pattern. Compile it once and
//! reuse it, because compilation is costly. This example extracts links from
//! Markdown text, where links look like `[text](url)`.

use std::collections::BTreeMap;

use regex::Regex;

/// Store a regular expression and the names of its capture groups.
struct Re(Regex, Vec<&'static str>);

/// Regexes take a while to compile.
/// It is reasonable to store them in a global static.
static GLOBAL_REGEX: std::sync::LazyLock<BTreeMap<&str, Re>> =
    std::sync::LazyLock::new(|| {
        println!("Initializing Regexes...\n");
        // A sorted map:
        let mut m = BTreeMap::new();
        // A Markdown inline link - see <https://spec.commonmark.org/>.
        // (?<name>  ) is a named capture group.
        // \s is a whitespace. \S is a not-whitespace.
        // [^!] excludes ! before [.
        m.insert(
            "[text](http...)",
            Re(
                Regex::new(r"[^!]\[(?<text>.*?)\]\s?\(\s*?(?<link>\S*?)\s*?\)")
                    .expect("the regex example invariant should hold"),
                vec!["text", "link"],
            ),
        );
        // A Markdown autolink.
        m.insert(
            "<http...>",
            Re(
                Regex::new("<(?<link>http.*?)>")
                    .expect("regular expression should be valid"),
                vec!["link"],
            ),
        );
        // A Markdown shortcut link.
        // [text] not preceded by ! or ], not followed by <optional spaces>[ or
        // ( or :
        m.insert(
            "[text] ...",
            Re(
                Regex::new(r"[^!\]]\[(?<text>[^\[\]]+?)\]\s*?[^\[\(:]")
                    .expect("the regex example invariant should hold"),
                vec!["text"],
            ),
        );
        // A Markdown reference-style link.
        m.insert(
            "[text][label]",
            Re(
                Regex::new(r"[^!\]]\[(?<text>.*?)\]\s?\[(?<label>.+?)\]")
                    .expect("the regex example invariant should hold"),
                vec!["text", "label"],
            ),
        );
        // A Markdown reference definition (with optional title):
        // (?:  ) is a non-capturing group.
        // (?m) flags the multi-line mode.
        // ^ and $ are the beginning and end of a line, respectively.
        m.insert(
        "[label]: url \"title\"",
        Re(Regex::new(r#"(?m)^\s*?\[(?<label>.*?)\]:\s*?(?<url>\S+)\s*?(?:"(?<title>.*)")?\s*$"#).expect("regular expression should be valid"),
        vec!["label", "url", "title"])
    );
        m
    });

/// Extract and print Markdown inline links e.g. [text](http...).
fn extract_inline_links(contents: &str) {
    for (_, [text, link]) in GLOBAL_REGEX["[text](http...)"]
        .0
        // `captures_iter` iterates through `Captures`, which stores the
        // capture groups for each match.
        .captures_iter(contents)
        // `extract` returns a tuple where
        // the first element corresponds to the full substring of the contents
        // that matched the regex. The second element is an array of
        // substrings, with each corresponding to the substring that matched
        // for a particular capture group.
        .map(|c| c.extract())
    {
        println!("[{text}]({link})\n");
    }
}

/// Locate markup in text.
fn search_with_all_regexes(contents: &str) {
    // Try to match all reggular expressions.
    for (key, re) in GLOBAL_REGEX.iter() {
        println!("----------------------\nLooking for {key}:\n");
        for caps in re.0.captures_iter(contents) {
            // Print the whole match.
            print!("{} -> ", &caps[0]);
            for group in &re.1 {
                print!(
                    "{group}={}; ",
                    // Retrieve each named capture group in turn...
                    // `extract` can't be used here, since the # of capture
                    // groups varies.
                    caps.name(group).map_or("", |m| m.as_str())
                );
            }
            println!("\n");
        }
    }
}

/// Extracts the inline links of a sample Markdown text.
fn main() {
    // Example Markdown links to process:
    let md: &'static str = "
<http://url0/>

[text1](url1)

[text2][lbl2]

[lbl2]: url2 \"title2\"

[lbl3][]

[lbl4]

![image5](image_url5)

![image6][image_lbl6]

image_lbl6: image_url6

![image_lbl7]

![image_lbl8][]
";

    extract_inline_links(md);

    search_with_all_regexes(md);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the regex example through its entry point.
    #[test]
    fn test_regex_example_entry_point() {
        main();
    }

    /// Inline links are captured with their text and target.
    #[test]
    fn captures_inline_links() {
        let re = &GLOBAL_REGEX["[text](http...)"].0;
        let caps: Vec<_> = re
            .captures_iter("x [a](u1) and [b](u2)")
            .map(|c| (c["text"].to_owned(), c["link"].to_owned()))
            .collect();
        assert_eq!(
            caps,
            [
                ("a".to_owned(), "u1".to_owned()),
                ("b".to_owned(), "u2".to_owned())
            ]
        );
    }

    /// Autolinks and reference definitions are captured.
    #[test]
    fn captures_autolinks_and_definitions()
    -> Result<(), Box<dyn std::error::Error>> {
        let autolink = &GLOBAL_REGEX["<http...>"].0;
        assert_eq!(
            &autolink.captures("<http://x/>").ok_or("missing value")?["link"],
            "http://x/"
        );
        let def = &GLOBAL_REGEX["[label]: url \"title\""].0;
        let caps = def.captures("[lbl]: url2 \"t\"").ok_or("missing value")?;
        assert_eq!(&caps["label"], "lbl");
        assert_eq!(&caps["url"], "url2");
        assert_eq!(&caps["title"], "t");
        Ok(())
    }

    /// Images are not matched as links.
    #[test]
    fn images_are_not_inline_links() {
        let re = &GLOBAL_REGEX["[text](http...)"].0;
        assert!(!re.is_match("![alt](img)"));
    }

    /// Each registered regex lists the groups it defines.
    #[test]
    fn group_names_exist_in_each_regex() {
        for (key, re) in GLOBAL_REGEX.iter() {
            for group in &re.1 {
                assert!(
                    re.0.capture_names().flatten().any(|n| n == *group),
                    "{key} lacks {group}"
                );
            }
        }
    }
}
