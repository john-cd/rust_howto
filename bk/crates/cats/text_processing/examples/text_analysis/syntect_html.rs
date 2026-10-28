// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Highlights source code with `syntect` (TextMate grammars and themes), as
//! HTML and as terminal escape codes.

use std::sync::LazyLock;

use syntect::easy::HighlightLines;
use syntect::highlighting::Style;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use syntect::util::as_24_bit_terminal_escaped;

// Loading the bundled grammars and themes is slow: do it once.
static SYNTAXES: LazyLock<SyntaxSet> =
    LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEMES: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

const THEME: &str = "base16-ocean.dark";

fn main() -> anyhow::Result<()> {
    let code = "fn main() {\n    println!(\"hello\");\n}\n";

    let syntax = SYNTAXES
        .find_syntax_by_extension("rs")
        .ok_or_else(|| anyhow::anyhow!("no Rust syntax"))?;
    let theme = THEMES
        .themes
        .get(THEME)
        .ok_or_else(|| anyhow::anyhow!("unknown theme {THEME}"))?;

    // 1. HTML with inline styles.
    let html = highlighted_html_for_string(code, &SYNTAXES, syntax, theme)?;
    println!("{html}");
    assert!(html.contains("<span style="));

    // 2. Terminal output: highlight line by line, keeping the state.
    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut terminal = String::new();
    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> =
            highlighter.highlight_line(line, &SYNTAXES)?;
        terminal.push_str(&as_24_bit_terminal_escaped(&ranges, false));
    }
    println!("{terminal}\x1b[0m");
    assert!(terminal.contains("\x1b[38;2;"));
    Ok(())
}
// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the syntect html example through its entry point.
    #[test]
    fn test_syntect_html_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
