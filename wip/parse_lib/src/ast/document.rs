use std::fmt::Display;

use super::Element;

#[derive(Debug, PartialEq, Eq)]
pub struct Document<'s> {
    elements: Vec<Element<'s>>,
}

impl<'s> Document<'s> {
    /// Creates a new `Document` with the given elements.
    pub fn new(elements: Vec<Element<'s>>) -> Self {
        Document { elements }
    }

    /// Returns a reference to the elements in the document.
    pub fn elements(&self) -> &[Element<'s>] {
        &self.elements
    }

    // pub fn elements_mut(&mut self) -> &mut Vec<Element<'s>> {
    //     &mut self.elements
    // }

    /// Returns true if the document has no elements.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Returns an iterator over all reference definitions in the document.
    pub fn refdefs(&self) -> impl Iterator<Item = &Element<'s>> {
        self.elements
            .iter()
            .filter(|e| matches!(e, Element::ReferenceDefinition { .. }))
    }
}

impl Display for Document<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in &self.elements {
            write!(f, "{element}")?;
        }
        Ok(())
    }
}
