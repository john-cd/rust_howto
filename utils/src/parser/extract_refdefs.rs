/// Extract reference definitions
use std::collections::BTreeMap;

use pulldown_cmark::LinkDef;
use pulldown_cmark::Parser;

/// Extract (and sort) reference definitions from a Markdown parser
///
/// parser: Markdown parser
pub(crate) fn get_sorted_ref_defs<'input, 'callback>(
    parser: &'input Parser<'input, 'callback>,
) -> BTreeMap<&'input str, &'input LinkDef<'input>>
where
    'callback: 'input,
{
    // BTreeMap is a sorted map
    let sorted_refdefs: BTreeMap<_, _> =
        parser.reference_definitions().iter().collect();
    sorted_refdefs
}
