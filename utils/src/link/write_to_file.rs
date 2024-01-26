/// Write links and reference definitions to file(s)
use std::io::Write;

use anyhow::Result;

use super::Link;

/// Write links to a writer (e.g. file)
pub(crate) fn write_links_to<W>(
    links: Vec<Link>,
    link_writer: &mut W,
) -> Result<()>
where
    W: Write,
{
    for l in links.iter() {
        write_link_to(l, link_writer)?;
    }
    Ok(())
}

/// Write a reference definition to a writer (e.g. file)
pub(crate) fn write_ref_defs_to<W>(
    links: Vec<Link>,
    ref_def_writer: &mut W,
) -> Result<()>
where
    W: Write,
{
    for l in links.iter() {
        write_ref_def_to(l, ref_def_writer)?;
    }
    Ok(())
}

/// Write reference definitions and links to a writer (e.g. file)
pub(crate) fn write_ref_defs_and_links_to<W>(
    links: Vec<Link>,
    writer: &mut W,
) -> Result<()>
where
    W: Write,
{
    for l in links.iter() {
        write_link_to(l, writer)?;
        write_ref_def_to(l, writer)?;
    }
    Ok(())
}

/// Write a reference definition and link
/// to two separate writers (e.g. files)
pub(crate) fn write_ref_defs_and_links_to_two<W1, W2>(
    links: Vec<Link>,
    link_writer: &mut W1,
    ref_def_writer: &mut W2,
) -> Result<()>
where
    W1: Write,
    W2: Write,
{
    for l in links.iter() {
        write_link_to(l, link_writer)?;
        write_ref_def_to(l, ref_def_writer)?;
    }
    Ok(())
}

/// Write a reference definition to a writer (e.g. file)
#[inline]
fn write_ref_def_to<W>(l: &Link, ref_def_writer: &mut W) -> Result<()>
where
    W: Write,
{
    writeln!(
        ref_def_writer,
        "{:?}\n{}\n{}",
        l,
        l.to_reference_definition(),
        l.to_badge_reference_definition()
    )?;
    Ok(())
}

// TODO
/// Write a link to a writer (e.g. file)
#[inline]
fn write_link_to<W>(l: &Link, link_writer: &mut W) -> Result<()>
where
    W: Write,
{
    writeln!(
        link_writer,
        "{:?}\n{}\n{}\n{}\n{}\n{}\n",
        l,
        l.to_inline_link(),
        l.to_reference_link(),
        l.to_reference_definition(),
        l.to_link_with_badge(),
        l.to_badge_reference_definition()
    )?;
    Ok(())
}
