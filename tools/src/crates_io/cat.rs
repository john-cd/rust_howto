// Category is not Hash / Ord / Eq, thus we define a new struct that does.\
// Otherwise, define a newtype and impl Ord, PartialOrd, PartialEq: https://doc.rust-lang.org/stable/core/cmp/trait.Ord.html

use crates_io_api::Category;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Cat {
    pub category: String,
    pub slug: String,
    pub description: String,
}

impl Cat {
    pub(super) fn new(cat: Category) -> Self {
        Self {
            category: cat.category,
            slug: cat.slug,
            description: cat.description,
        }
    }
}
