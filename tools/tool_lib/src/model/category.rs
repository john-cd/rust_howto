use serde::Serialize;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Debug)]
pub struct Category {
    pub category: String,
    pub slug: String,
    pub description: String,
}
