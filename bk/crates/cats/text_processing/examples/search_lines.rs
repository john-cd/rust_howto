// TODO Reviewed ??-??-2026.
//! Finds matching lines and collects their line numbers without invoking a CLI.

// ANCHOR: example
use grep_regex::RegexMatcher;
use grep_searcher::SearcherBuilder;
use grep_searcher::sinks::UTF8;

/// Searches a UTF-8 log fixture and sends matching lines to a closure.
///
/// # Errors
///
/// Returns an error if the pattern is invalid or the sink cannot decode a match.
fn main() -> anyhow::Result<()> {
    let log = b"INFO started\nWARN retrying\nINFO connected\nWARN slow response\n";
    let matcher = RegexMatcher::new(r"^WARN\b")?;
    let mut hits = Vec::new();

    SearcherBuilder::new()
        .line_number(true)
        .build()
        .search_slice(
            &matcher,
            log,
            UTF8(|number, line| {
                hits.push((number, line.trim_end_matches('\n').to_owned()));
                Ok(true)
            }),
        )?;

    assert_eq!(
        hits,
        [
            (2, String::from("WARN retrying")),
            (4, String::from("WARN slow response"))
        ]
    );
    for (number, line) in hits {
        println!("{number}: {line}");
    }
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Reports both matching lines with their one-based line numbers.
    #[test]
    fn collects_matching_log_lines() -> anyhow::Result<()> {
        super::main()
    }
}
