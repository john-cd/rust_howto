/// Markdown (inline or reference-style) links
/// Rules to create a reference label and/or a badge URL from a link URL


mod link;
mod rules;
mod write_link_to_file;

pub(crate) use link::*;
pub(crate) use rules::*;
pub(crate) use write_link_to_file::*;
