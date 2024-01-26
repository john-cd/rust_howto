use std::borrow::Cow;
/// Generate links and reference definitions
use std::io::Write;

use anyhow::Result;
use pulldown_cmark::LinkDef;
use pulldown_cmark::Parser;
use regex::Regex;
use tracing::info;

use crate::link::write_ref_defs_and_links_to_two;
use crate::link::LinkBuilder;

/// Get existing reference definitions from a Markdown parser,
/// identify URLs that are GitHub repos, create badge URLs for these
/// links, and write to a writer / file.
///
/// parser: Markdown parser
/// w: Writer (e.g. File) to write to
pub(super) fn write_github_repo_badge_refdefs<W>(
    parser: Parser,
    w: &mut W,
) -> Result<()>
where
    W: Write,
{
    let sorted_refdefs = super::parser::get_sorted_ref_defs(&parser);

    let rule = &crate::link::GLOBAL_RULES["github repo"];
    let re = Regex::new(rule.re).unwrap();

    let mut links = Vec::new();

    // Iterate through all ref defs
    for (lbl, LinkDef { dest: dest_url, .. }) in sorted_refdefs {
        // if the URL is a github repo...
        if let Some(capture) = re.captures(dest_url.as_ref()) {
            info!("dest_url: {} -> {:?}", dest_url, capture);

            // ...create the URL for the badge...
            let badge_image_url =
                re.replace(dest_url.as_ref(), rule.badge_url_pattern);
            info!("badge_image_url: {}", badge_image_url);

            let link = LinkBuilder::default()
                .set_label(Cow::from(lbl))
                .set_image_url(badge_image_url)
                .build();
            links.push(link);
        }
    }

    // ...and write the reference definition and link to it.
    let mut link_buffer = Vec::new();

    write_ref_defs_and_links_to_two(links, &mut link_buffer, w)?;

    // Write links after reference definitions
    w.write_all(&link_buffer)?;
    Ok(())
}
