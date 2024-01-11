use std::collections::HashMap;

use anyhow::Result;
use heck::ToKebabCase;
use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

static GLOBAL_REGEX: Lazy<HashMap<&str, Regex>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "[text](http...)",
        Regex::new(r"\[(?<text>.*?)\]\((?<link>http.*?)\)").unwrap(),
    );
    m.insert("<http...>", Regex::new(r"<(?<link>http.*?)>").unwrap());
    m.insert(
        "[text] ...",
        Regex::new(r"\[(?<text>.*?)\]\s?[^\[]]").unwrap(),
    ); // not followed by [ or <space>[
    m.insert(
        "[text][label]",
        Regex::new(r"\[(?<text>.*?)\]\s??\[(?<label>.*?)\]").unwrap(),
    );
    // link label with optional title; (?:non-capturing group)
    // (?<name>named group)
    m.insert("[label]: url \"title\"", Regex::new(r#"(?m)^\s*?\[(?<label>.*?)\]:\s*?(?<url>.*?)\s*?(?:"(?<title>.*?)")?\s*$"#).unwrap());
    m
});

#[derive(Debug)]
struct Link {
    text: Option<String>,            // [text](...)
    url: Url,                        // [...]: url or [...](url)
    title: Option<String>, // [...]: url "title" or [...](url "title")
    reference_label: Option<String>, /* [...][reference_label] and [reference_label]: ... */
}

impl Link {
    fn new(
        text: Option<String>,
        url: Url,
        title: Option<String>,
        reference_label: Option<String>,
    ) -> Self {
        Self {
            text,
            url,
            title,
            reference_label,
        }
    }

    fn get_text(&self) -> String {
        if let Some(ref txt) = self.text {
            txt.to_owned()
        } else {
            "TODO".to_string()
        }
    }

    fn get_url_and_title(&self) -> String {
        let u = self.url.as_str();
        if let Some(ref t) = self.title {
            format!("{u} \"{t}\"")
        } else {
            u.to_string()
        }
    }

    fn get_reference_label(&self) -> String {
        if let Some(ref rl) = self.reference_label {
            rl.to_string()
        } else if let Some(txt) = &self.text {
            txt.to_kebab_case()
        } else {
            "XYZ".to_string()
        }
    }

    // [text](url) or [text](url "title")
    fn to_inline_link(&self) -> String {
        format!("[{}]({})", self.get_text(), self.get_url_and_title())
    }

    // [text][label] or [text/label]
    fn to_reference_link(&self) -> String {
        let txt = self.get_text();
        let rl = self.get_reference_label();
        if txt == rl {
            format!("[{txt}]")
        } else {
            format!("[{txt}][{rl}]")
        }
    }

    // [label]: url or [label]: url "title"
    fn to_link_label(&self) -> String {
        format!(
            "[{}]: {}",
            self.get_reference_label(),
            self.get_url_and_title()
        )
    }

    fn from_text(contents: &str) -> Self {
        // let mut h = HashMap::new();
        for (s, re) in GLOBAL_REGEX.iter() {
            let mut results = vec![];
            for (_, [text, link]) in
                re.captures_iter(contents).map(|c| c.extract())
            {
                results.push((text, link));
                println!("{text} {link}");
            }
        }
        todo!();
    }
}
