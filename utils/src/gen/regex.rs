use regex::Regex;

/// Regexes to retrieve Markdown elements
static GLOBAL_REGEX: Lazy<BTreeMap<&str, Re>> = Lazy::new(|| {
    tracing::debug!("Initializing Regexes...\n");
    // A sorted map
    let mut m = BTreeMap::new();
    // A Markdown reference definition (with optional title)
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
