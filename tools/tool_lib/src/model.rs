#![allow(dead_code)]

use serde::Serialize;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Serialize)]
pub struct Category {
    pub category: String,
    pub slug: String,
    pub description: String,
}
