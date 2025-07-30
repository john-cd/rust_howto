use super::Document;

#[derive(Debug, PartialEq, Eq)]
pub struct Documents<'s> {
    pub documents: Vec<Document<'s>>,
}

impl<'s> Documents<'s> {
    /// Creates a new document collection.
    pub fn new(documents: Vec<Document<'s>>) -> Self {
        Self { documents }
    }
}

// impl std::fmt::Display for Documents<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for doc in &self.documents {
//             write!(f, "{doc}")?;
//         }
//         Ok(())
//     }
// }
