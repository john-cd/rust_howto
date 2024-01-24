use std::borrow::Cow;
use std::collections::BTreeMap;
use std::io::Write;

use anyhow::Result;
use pulldown_cmark::LinkDef;
use pulldown_cmark::Parser;
use regex::Regex;
use tracing::info;

use super::super::link::LinkBuilder;

/// Extract (and sort) reference definitions from a Markdown parser
///
/// parser: Markdown parser
pub(super) fn get_sorted_ref_defs<'input, 'callback>(
    parser: &'input Parser<'input, 'callback>,
) -> BTreeMap<&'input str, &'input LinkDef<'input>>
where 'callback: 'input
{
    // BTreeMap is a sorted map
    let sorted_refdefs: BTreeMap<_, _> =
        parser.reference_definitions().iter().collect();
    sorted_refdefs
}

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
    let sorted_refdefs = get_sorted_ref_defs(parser);

    let rule = &super::super::rules::GLOBAL_RULES["github repo"];
    let re = Regex::new(rule.re).unwrap();

    let mut buf = Vec::new();

    // Iterate through all ref defs
    for (lbl, LinkDef { dest, .. }) in sorted_refdefs {
        // if the URL is a github repo...
        if let Some(c) = re.captures(dest) {
            info!("{}: {:?}", dest, c);
            let badge_image_url = re.replace(dest, rule.badge_url_pattern);
            info!("{}", badge_image_url);
            write_ref_def_and_link_to(lbl, badge_image_url, w, &mut buf)?;
        }
    }
    w.write_all(&buf)?;
    Ok(())
}

/// Write a reference definition and link to two separate writers / files
fn write_ref_def_and_link_to<W1, W2>(lbl: &str, image_url: Cow<'_, str>, ref_def_writer: &mut W1, link_writer: &mut W2) -> Result<()>
where
    W1: Write,
    W2: Write,
{
    let link = LinkBuilder::default()
                .set_label(lbl.into())
                .set_image_url(image_url.into())
                .build();
    writeln!(ref_def_writer, "{}", link.to_badge_reference_definition())?;
    writeln!(link_writer, "{}", link.to_link_with_badge())?;
    Ok(())
}
