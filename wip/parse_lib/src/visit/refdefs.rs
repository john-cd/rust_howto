use std::fmt::Result;

use crate::ast::*;

/// A visitor that collects reference definitions.
struct RefDefVisitor<'v>(RefDefManager<'v>);

impl RefDefVisitor<'_> {
    fn new() -> Self {
        Self(RefDefManager::new())
    }
}

impl<'v> super::Visitor<'v> for RefDefVisitor<'v> {
    fn visit_reference_definition(&mut self, refdef: &'v ReferenceDefinitionData) -> Result {
        self.0.refdefs.push(refdef);
        Ok(())
    }
}

/// A newtype that store a list of reference definitions
/// and provide useful methods.
pub struct RefDefManager<'v> {
    pub refdefs: Vec<&'v ReferenceDefinitionData<'v>>,
}

use multimap::MultiMap;

impl RefDefManager<'_> {
    fn new() -> Self {
        Self {
            refdefs: Vec::new(),
        }
    }

    /// Return labels used more than once.
    pub fn dupe_labels(&mut self) -> Vec<&str> {
        // Could use `itertools` as well.
        let mm: MultiMap<&str, &ReferenceDefinitionData<'_>> =
            self.refdefs.iter().map(|&rd| (rd.label, rd)).collect();
        let mut labels: Vec<&str> = mm.keys().filter(|&&k| mm.is_vec(k)).cloned().collect();
        labels.sort();
        labels
    }

    // Return urls used more than once.
    pub fn dupe_urls(&mut self) -> Vec<&str> {
        let mm: MultiMap<&str, &ReferenceDefinitionData<'_>> =
            self.refdefs.iter().map(|&rd| (rd.url, rd)).collect();
        let mut urls: Vec<&str> = mm.keys().filter(|&&k| mm.is_vec(k)).cloned().collect();
        urls.sort();
        urls
    }

    pub fn get_sorted_dedupe_refdefs(&self) -> Vec<&ReferenceDefinitionData<'_>> {
        let mut refdefs = self.refdefs.clone();
        refdefs.sort();
        // If the vector is sorted, this removes all duplicates:
        refdefs.dedup();
        refdefs
    }
}

/// Returns a `RefDefManager` that contains the list of refdefs for the provided documents.
/// It offers methods to check refdefs.
pub fn refdefs<'d, 'a>(docs: &'a Documents<'d>) -> anyhow::Result<RefDefManager<'d>>
where
    'a: 'd,
{
    let mut vis = RefDefVisitor::new();
    docs.accept(&mut vis)?;
    Ok(vis.0)
}
