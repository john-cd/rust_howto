use std::collections::BTreeMap;
use std::io::Write;

use anyhow::Result;
use pulldown_cmark::LinkDef;
use pulldown_cmark::Parser;
use regex::Regex;
use tracing::info;

use super::super::link::LinkBuilder;

// REFERENCE DEFINITIONS

/// Write reference definitions parsed from a Markdown parser to a
/// file / writer.
///
/// parser: Markdown parser
/// w: Writer e.g. File
pub(super) fn write_ref_defs<W>(parser: &Parser, mut w: W) -> Result<()>
where
    W: Write,
{
    // BTreeMap is a sorted map
    let sorted_refdefs: BTreeMap<_, _> =
        parser.reference_definitions().iter().collect();

    for (s, LinkDef { dest, title, .. }) in sorted_refdefs {
        if let Some(t) = title {
            writeln!(&mut w, "[{s}]: {dest} \"{t:?}\"")?;
        } else {
            writeln!(&mut w, "[{s}]: {dest}")?;
        }
    }
    Ok(())
}

/// Get existing reference definitions from a Markdown parser,
/// identify URLs that are GitHub repos, create badge URLs for these
/// links, and write to a writer / file.
///
/// parser: Markdown parser
/// w: Writer (e.g. File) to write to
pub(super) fn write_github_repo_badge_refdefs<W>(
    parser: &Parser,
    w: &mut W,
) -> Result<()>
where
    W: Write,
{
    let sorted_refdefs: BTreeMap<_, _> =
        parser.reference_definitions().iter().collect();

    let rule = &super::super::rules::GLOBAL_RULES["github repo"];
    let re = Regex::new(rule.re).unwrap();

    let mut buf = Vec::new();

    // Iterate through all ref defs
    for (lbl, LinkDef { dest, .. }) in sorted_refdefs {
        // if the URL is a github repo...
        if let Some(c) = re.captures(dest) {
            info!("{}: {:?}", dest, c);
            let badge_image_url = re.replace(dest, rule.badge_url_pattern);
            info!("{}", badge_image_url.to_string());
            let link = LinkBuilder::default()
                .set_label(lbl.to_string())
                .set_image_url(badge_image_url.to_string())
                .build();
            writeln!(w, "{}", link.to_badge_reference_definition())?;
            writeln!(&mut buf, "{}", link.to_link_with_badge())?;
        }
    }
    w.write_all(&buf)?;
    Ok(())
}
