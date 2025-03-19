// ANCHOR: example
use std::collections::BTreeMap;

use once_cell::sync::Lazy;
use regex::Regex;

// Regular expression and the names of its capture groups.
struct Re(Regex, Vec<&'static str>);

// Regexes take a while to compile; it is reasonable to store them in
// a global static
static GLOBAL_REGEX: Lazy<BTreeMap<&str, Re>> = Lazy::new(|| {
    println!("Initializing Regexes...\n");
    // A sorted map:
    let mut m = BTreeMap::new();
    // A Markdown inline link:
    // (?<name>  ) is a named capture group.
    // \s is a whitespace. \S is a not-whitespace.
    m.insert(
        "[text](http...)",
        Re(
            Regex::new(r"[^!]\[(?<text>.*?)\]\s?\(\s*?(?<link>\S*?)\s*?\)")
                .unwrap(),
            vec!["text", "link"],
        ),
    );
    // A Markdown autolink
    m.insert(
        "<http...>",
        Re(Regex::new(r"<(?<link>http.*?)>").unwrap(), vec!["link"]),
    );
    // A Markdown shortcut link
    // or <spaces>( or :
    m.insert(
        "[text] ...",
        Re(
            Regex::new(r"[^!\]]\[(?<text>[^\[\]]+?)\]\s*?[^\[\(:]").unwrap(),
            vec!["text"],
        ),
    );
    // A Markdown reference-style link
    m.insert(
        "[text][label]",
        Re(
            Regex::new(r"[^!\]]\[(?<text>.*?)\]\s?\[(?<label>.+?)\]").unwrap(),
            vec!["text", "label"],
        ),
    );
    // A Markdown reference definition (with optional title):
    // (?:  ) is a non-capturing group.
    // (?m) flags multi-line mode. ^ and $ are the beginning and end of a
    // line, respectively.
    m.insert(
        "[label]: url \"title\"",
        Re(Regex::new(r#"(?m)^\s*?\[(?<label>.*?)\]:\s*?(?<url>\S+)\s*?(?:"(?<title>.*)")?\s*$"#).unwrap(),
        vec!["label", "url", "title"])
    );
    m
});

#[allow(dead_code)]
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

// Locate markup in text
fn search_with_all_regexes(contents: &str) {
    // Try to match all reggular expressions
    for (key, re) in GLOBAL_REGEX.iter() {
        println!("----------------------\nLooking for {}:\n", key);
        for caps in re.0.captures_iter(contents) {
            // Print the whole match
            print!("{} -> ", &caps[0]);
            for group in re.1.iter() {
                print!(
                    "{}={}; ",
                    group,
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

// Example Markdown to process
fn get_test_markdown() -> String {
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
    md.to_owned()
}

fn main() {
    search_with_all_regexes(get_test_markdown().as_str());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [^!] excludes !
// [text] not preceded by ! or ], not followed by [ or <spaces>[ or (
// TODO WIP review NOW
