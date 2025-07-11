use std::fmt::Display;

use super::Element;

#[derive(Debug, PartialEq, Eq)]
pub struct Document<'s> {
    elements: Vec<Element<'s>>,
}

impl<'s> Document<'s> {
    pub fn new(elements: Vec<Element<'s>>) -> Self {
        Document { elements }
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
