use regex::Regex;

use super::conf::PreprocConfig;

// Generate the replacement Regexes needed, depending on the configuration
// Can return an empty Vec.
pub fn get_regexes(conf: &PreprocConfig) -> Vec<Regex> {
    let mut re = vec![];
    if conf.remove_hidden_sections {
        re.push(
            Regex::new(r#"(<div class="hidden">)[^<]+?(</div>)"#)
                .expect("Invalid regex"),
        );
    }
    if conf.do_not_include_hidden_chapters {
        let re_string: String =
            r"[{]{2} *#(include|rustdoc_include|playground) *".to_string()
                + &conf.hidden_chapter_prefix.clone()
                + r"[^}]*?[}]{2}";
        // { and } are special for Regex, thus must be escaped with \ except if
        // within []
        // We don't use format! here, since { and } are
        // special for format! and must be escaped with the same
        // character.
        // tracing::debug!(re_string);
        re.push(Regex::new(&re_string).expect("Invalid regex"));
    }
    re
}
