/// Functions that take a Markdown parser
/// and write (parts of) its contents to a file
mod github;
mod write_raw_to;
mod write_ref_defs;
mod write_whole;

pub(crate) use github::*;
pub(crate) use write_raw_to::*;
pub(crate) use write_ref_defs::*;
#[allow(unused_imports)]
pub(crate) use write_whole::*;
