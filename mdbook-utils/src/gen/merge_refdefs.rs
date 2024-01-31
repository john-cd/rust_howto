//! Merge existing reference definitions and new ones
//! and write the result to a file
use crate::link::Link;

/// Append, sort and dedupe reference definitions
pub(crate) fn merge_links<'a>(
    mut existing_links: Vec<Link<'a>>,
    new_links: &mut Vec<Link<'a>>,
) -> Vec<Link<'a>>
{
    let mut buf = existing_links.clone();
    buf.append(new_links);

    // `Link` has a custom Ord / Eq implementation,
    // thus we can sort them.
    buf.sort();
    buf.dedup();
    buf
}
