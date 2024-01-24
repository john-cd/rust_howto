/// Merge existing refdefs and new ones and write the result to file
use std::io::Read;
use std::io::Write;

use anyhow::Result;

use crate::link::Link;

// Append, sort and dedupe reference definitions
pub(crate) fn merge_refdefs<R1, R2>(
    existing_refdefs: R1,
    new_refdefs: R2,
) -> Result<Vec<Link<'static>>>
where
    R1: Read,
    R2: Read,
{
    let mut buf: Vec<Link> = Vec::new();

    // Link has a custom Ord / Eq implementation, thus we can sort them.
    buf.sort();

    Ok(buf)
}

fn write_refdefs<W>(w: &mut W) -> Result<()>
where
    W: Write,
{
    // w.write_all(&buf)?;
    Ok(())
}
