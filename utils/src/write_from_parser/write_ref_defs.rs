use std::io::Write;

use anyhow::Result;
use pulldown_cmark::LinkDef;
use pulldown_cmark::Parser;

/// Write reference definitions parsed from a Markdown parser to a
/// file / writer.
///
/// parser: Markdown parser
/// w: Writer e.g. File
pub(crate) fn write_ref_defs<'input, 'callback, W>(
    parser: &'input Parser<'input, 'callback>,
    mut w: W,
) -> Result<()>
where
    W: Write,
    'callback: 'input,
{
    let sorted_refdefs =
        crate::parser::get_sorted_ref_defs::<'input, 'callback>(parser);

    for (s, LinkDef { dest, title, .. }) in sorted_refdefs {
        if let Some(t) = title {
            writeln!(&mut w, "[{s}]: {dest} \"{t:?}\"")?;
        } else {
            writeln!(&mut w, "[{s}]: {dest}")?;
        }
    }
    Ok(())
}
