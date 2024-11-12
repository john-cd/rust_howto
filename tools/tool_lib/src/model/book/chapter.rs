#![allow(dead_code)]

// use std::collections::HashMap;

use serde::Serialize;

use super::*;

#[derive(Serialize, Debug)]
pub struct Chapter {
    description: String,
    subchapters: Vec<Subchapter>,
}

// impl Renderable for Chapter {
//     fn get_templates() -> Vec<String> {
//         vec![
//             "chapter/index.md".into(),
//             "chapter/index.incl.md".into(),
//             "chapter/refs.incl.md".into(),
//         ]
//     }
// }

impl Chapter {
    fn new(description: String, subchapters: Vec<Subchapter>) -> Self {
        Self {
            description,
            subchapters,
        }
    }
}
