use std::io::Write;

use anyhow::Result;
use pulldown_cmark::LinkDef;
use pulldown_cmark::Parser;

/// Write reference definitions parsed from a Markdown parser to a
/// file / writer.
///
/// parser: Markdown parser
/// w: Writer e.g. File
pub(crate) fn write_ref_defs_to<W>(parser: Parser, w: &mut W) -> Result<()>
where
    W: Write,
{
    let sorted_refdefs = crate::parser::get_sorted_ref_defs(&parser); // ::<'input, 'callback>

    for (s, LinkDef { dest, title, .. }) in sorted_refdefs {
        if let Some(t) = title {
            writeln!(w, "[{s}]: {dest} \"{t:?}\"")?;
        } else {
            writeln!(w, "[{s}]: {dest}")?;
        }
    }
    Ok(())
}
