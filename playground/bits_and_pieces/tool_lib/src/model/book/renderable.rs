//! Objects that can be rendered with templates.
//!
//! TODO Reviewed ??-??-2026.

use serde::Serialize;

/// An object that can be rendered with templates.
pub trait Renderable: Serialize {
    /// Returns the templates to render and where to write them.
    fn get_what_to_render(&self) -> Vec<RenderInfo>;

    /// Returns the objects contained in this one.
    fn get_children(&self) -> impl IntoIterator<Item = impl Renderable>;
}

/// An empty implementation of the `Renderable` trait.
#[derive(Serialize, Debug)]
pub struct Render;

impl Renderable for Render {
    fn get_what_to_render(&self) -> Vec<RenderInfo> {
        vec![]
    }

    fn get_children(&self) -> impl IntoIterator<Item = impl Renderable> {
        std::iter::empty::<Render>()
    }
}

// All the information required to render a given chapter / subchapter / index
/// Everything required to render a chapter, subchapter or index.
#[derive(Serialize, Debug)]
pub struct RenderInfo {
    // name (e.g. path without glob prefix) of the template for a given object
    /// Name of the template, without the glob prefix.
    pub template_name: String,
    // String containing a template to build the destination file path
    /// Template of the destination file path.
    pub dest_filepath_template: String,
}

impl RenderInfo {
    /// Creates the information required to render an object.
    pub fn new(template_name: String, dest_filepath_template: String) -> Self {
        Self {
            template_name,
            dest_filepath_template,
        }
    }
}
