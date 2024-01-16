use std::collections::HashMap;
use std::fmt::format;

use anyhow::Result;
use heck::ToKebabCase;
use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

// static GLOBAL_REGEX: Lazy<HashMap<&str, Regex>> = Lazy::new(|| {
//     let mut m = HashMap::new();
//     m.insert(
//         "[text](http...)",
//         Regex::new(r"\[(?<text>.*?)\]\((?<link>http.*?)\)").
// unwrap(),     );
//     m.insert("<http...>",
// Regex::new(r"<(?<link>http.*?)>").unwrap());     m.insert(
//         "[text] ...",
//         Regex::new(r"\[(?<text>.*?)\]\s?[^\[]]").unwrap(),
//     ); // not followed by [ or <space>[
//     m.insert(
//         "[text][label]",
//         Regex::new(r"\[(?<text>.*?)\]\s??\[(?<label>.*?)\]").
// unwrap(),     );
//     // link label with optional title; (?:non-capturing group)
//     // (?<name>named group)
//     m.insert("[label]: url \"title\"",
// Regex::new(r#"(?m)^\s*?\[(?<label>.*?)\]:\s*?(?<url>.*?)\s*?(?:"(?
// <title>.*?)")?\s*$"#).unwrap());     m
// });

#[derive(Debug)]
struct Link {
    text: Option<String>,  // [text](...)
    url: Url,              // [...]: url or [...](url) or <url>
    title: Option<String>, // [...]: url "title" or [...](url "title")
    id: Option<String>,    // [...][id] and [id]: ...
}

impl Link {
    fn new(
        text: Option<String>,
        url: Url,
        title: Option<String>,
        id: Option<String>,
    ) -> Self {
        Self {
            text,
            url,
            title,
            id,
        }
    }

    // return text or TODO if empty
    fn get_text(&self) -> String {
        if let Some(ref txt) = self.text {
            txt.to_owned()
        } else {
            "TODO".to_string()
        }
    }

    // return url (and title if present)
    fn get_url_and_title(&self) -> String {
        let u = self.url.as_str();
        if let Some(ref t) = self.title {
            format!("{u} \"{t}\"")
        } else {
            u.to_string()
        }
    }

    // TODO
    // build reference label from Rules
    fn get_id(&self) -> String {
        if let Some(ref rl) = self.id {
            rl.to_string()
        } else if let Some(txt) = &self.text {
            txt.to_kebab_case()
        } else {
            "XYZ".to_string()
        }
    }

    // return [text](url) or [text](url "title")
    pub fn to_inline_link(&self) -> String {
        format!("[{}]({})", self.get_text(), self.get_url_and_title())
    }

    // return [text][id] or [text/id]
    pub fn to_reference_link(&self) -> String {
        let txt = self.get_text();
        let id = self.get_id();
        if txt == id {
            format!("[{txt}]")
        } else {
            format!("[{txt}][{id}]")
        }
    }

    // return [label]: url or [label]: url "title"
    fn to_link_label(&self) -> String {
        format!("[{}]: {}", self.get_id(), self.get_url_and_title())
    }

    // BADGES

    // TODO
    // return <label>-badge, the label for the badge reference
    fn get_badge_url(&self) -> String {
        "".into()
    }

    // return [![id-badge]][id] a badge image with a link
    fn to_badge(&self) -> String {
        let id = self.get_id();
        format!("[![{id}-badge]][{id}]")
    }

    // return [...-badge]: https://badge-cache.kominick.com/...
    pub fn to_badge_link_label(&self) -> String {
        format!("[{}-badge]: {}", self.get_id(), self.get_badge_url())
    }

    // TODO
    //
    // fn from_text(contents: &str) -> Self {
    //     // let mut h = HashMap::new();
    //     for (s, re) in GLOBAL_REGEX.iter() {
    //         let mut results = vec![];
    //         for (_, [text, link]) in
    //             re.captures_iter(contents).map(|c| c.extract())
    //         {
    //             results.push((text, link));
    //             println!("{text} {link}");
    //         }
    //     }
    //     todo!();
    // }
}
