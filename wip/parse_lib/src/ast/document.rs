use std::path::Path;

use super::Element;

#[derive(Debug, PartialEq, Eq)]
pub struct Document<'s> {
    pub name: &'s str,
    pub path: Option<&'s Path>,
    pub elements: Vec<Element<'s>>,
}

impl<'s> Document<'s> {
    /// Creates a new `Document` with the given elements.
    pub fn new(name: &'s str, path: Option<&'s Path>, elements: Vec<Element<'s>>) -> Self {
        Document {
            name,
            path,
            elements,
        }
    }

    // /// Returns a reference to the elements in the document.
    // pub fn elements(&'s self) -> &'s [Element<'s>] {
    //     &self.elements
    // }

    // /// Returns a mutable reference to the elements in the document.
    // pub fn elements_mut(&mut self) -> &mut Vec<Element<'s>> {
    //     &mut self.elements
    // }

    /// Returns true if the document has no elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
