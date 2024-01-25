/// Functions that generate, sort, deduplicate reference definitions
/// and links
mod merge_refdefs;
mod refdefs_from_dependencies;

pub use merge_refdefs::*;
pub use refdefs_from_dependencies::*;
