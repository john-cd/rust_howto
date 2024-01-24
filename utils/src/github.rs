/// Generate links and reference definitions
use std::io::Write;

use anyhow::Result;
use pulldown_cmark::LinkDef;
use pulldown_cmark::Parser;
use regex::Regex;
use tracing::info;

/// Get existing reference definitions from a Markdown parser,
/// identify URLs that are GitHub repos, create badge URLs for these
/// links, and write to a writer / file.
///
/// parser: Markdown parser
/// w: Writer (e.g. File) to write to
pub(super) fn write_github_repo_badge_refdefs<'input, 'callback, W>(
    parser: &'input Parser<'input, 'callback>,
    w: &mut W,
) -> Result<()>
where
    W: Write,
{
    let sorted_refdefs = super::parser::get_sorted_ref_defs(parser);

    let rule = &crate::link::GLOBAL_RULES["github repo"];
    let re = Regex::new(rule.re).unwrap();

    let mut buf = Vec::new();

    // Iterate through all ref defs
    for (lbl, LinkDef { dest, .. }) in sorted_refdefs {
        // if the URL is a github repo...
        if let Some(c) = re.captures(dest.as_ref()) {
            info!("{}: {:?}", dest, c);
            let badge_image_url =
                re.replace(dest.as_ref(), rule.badge_url_pattern);
            info!("{}", badge_image_url);
            crate::link::write_ref_def_and_link_to(
                lbl,
                badge_image_url,
                w,
                &mut buf,
            )?;
        }
    }
    w.write_all(&buf)?;
    Ok(())
}
